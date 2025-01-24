//! Represents the style applied to lines or borders. The $type property MUST be set to the string
//! `strokeStyle`. The value MUST be either:
//!
//! - a string value as defined in the corresponding section below, or
//! - an object value as defined in the corresponding section below
//!
//! See: <https://tr.designtokens.org/format/#stroke-style>.

// NOTE: Something the `Reflect` derive generates triggers this warning.
#![cfg_attr(feature = "reflect", allow(clippy::used_underscore_binding))]

use std::{collections::HashMap, str::FromStr};

use tinyjson::JsonValue;

use crate::error::Error;

use super::dimension::Dimension;

/// See module docs.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum StrokeStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Outset,
    Inset,
    Custom {
        dash_array: Vec<Dimension>,
        line_cap: LineCap,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum LineCap {
    Round,
    Butt,
    Square,
}

impl TryFrom<&JsonValue> for StrokeStyle {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(v) => StrokeStyle::from_str(v),
            JsonValue::Object(v) => StrokeStyle::try_from(v),
            _ => Err(Error::UnexpectedType),
        }
    }
}

impl TryFrom<&HashMap<String, JsonValue>> for StrokeStyle {
    type Error = Error;

    fn try_from(map: &HashMap<String, JsonValue>) -> Result<Self, Self::Error> {
        let dash_array = map
            .get("dashArray")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<Vec<_>>().ok_or(Error::ExpectedArray))
            .and_then(|v| {
                v.iter()
                    .map(|val| {
                        Dimension::from_str(val.get::<String>().ok_or(Error::ExpectedItemString)?)
                    })
                    .collect::<Result<Vec<_>, Error>>()
            })
            .map_err(|err| Error::prop("dashArray", err))?;

        let line_cap = map
            .get("lineCap")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| match v.as_str() {
                "round" => Ok(LineCap::Round),
                "butt" => Ok(LineCap::Butt),
                "square" => Ok(LineCap::Square),
                _ => Err(Error::InvalidFormat("unknown variant")),
            })
            .map_err(|err| Error::prop("lineCap", err))?;

        Ok(Self::Custom {
            dash_array,
            line_cap,
        })
    }
}

impl FromStr for StrokeStyle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "solid" => Ok(Self::Solid),
            "dashed" => Ok(Self::Dashed),
            "dotted" => Ok(Self::Dotted),
            "double" => Ok(Self::Double),
            "groove" => Ok(Self::Groove),
            "ridge" => Ok(Self::Ridge),
            "outset" => Ok(Self::Outset),
            "inset" => Ok(Self::Inset),
            _ => Err(Error::InvalidFormat("unknown variant")),
        }
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for StrokeStyle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;

        tokens.extend(quote!(dtoken::types::stroke_style::));
        tokens.extend(match self {
            Self::Solid => quote! { StrokeStyle::Solid },
            Self::Dashed => quote! { StrokeStyle::Dashed },
            Self::Dotted => quote! { StrokeStyle::Dotted },
            Self::Double => quote! { StrokeStyle::Double },
            Self::Groove => quote! { StrokeStyle::Groove },
            Self::Ridge => quote! { StrokeStyle::Ridge },
            Self::Outset => quote! { StrokeStyle::Outset },
            Self::Inset => quote! { StrokeStyle::Inset },
            Self::Custom {
                dash_array,
                line_cap,
            } => {
                quote! { StrokeStyle::Custom {
                    dash_array: vec![#( #dash_array,)*],
                    line_cap: #line_cap
                } }
            }
        });
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for LineCap {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;

        tokens.extend(quote!(dtoken::types::stroke_style::));
        tokens.extend(match self {
            LineCap::Round => quote! { LineCap::Round },
            LineCap::Butt => quote! { LineCap::Butt },
            LineCap::Square => quote! { LineCap::Square },
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("solid",   Ok(StrokeStyle::Solid)),
            ("dotted",  Ok(StrokeStyle::Dotted)),
            ("double",  Ok(StrokeStyle::Double)),
            ("groove",  Ok(StrokeStyle::Groove)),
            ("invalid", Err(Error::InvalidFormat("unknown variant"))),
        ];

        for (input, expected) in test_cases {
            let result = StrokeStyle::from_str(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_from_map() {
        use JsonValue::{Array, String};

        let test_cases = vec![
            (
                HashMap::from([
                    (
                        "dashArray".to_owned(),
                        Array(vec![String("5px".to_owned()), String("10px".to_owned())]),
                    ),
                    ("lineCap".to_owned(), String("round".to_owned())),
                ]),
                Ok(StrokeStyle::Custom {
                    dash_array: vec![Dimension::Pixels(5.0), Dimension::Pixels(10.0)],
                    line_cap: LineCap::Round,
                }),
            ),
            (
                HashMap::new(),
                Err(Error::prop("dashArray", Error::MustExist)),
            ),
            (
                HashMap::from([
                    (
                        "dashArray".to_owned(),
                        Array(vec![String("5px".to_owned()), String("invalid".to_owned())]),
                    ),
                    ("lineCap".to_owned(), String("round".to_owned())),
                ]),
                Err(Error::prop("dashArray", Error::InvalidUnit(&["px", "rem"]))),
            ),
            (
                HashMap::from([
                    (
                        "dashArray".to_owned(),
                        Array(vec![String("5px".to_owned()), String("10px".to_owned())]),
                    ),
                    ("lineCap".to_owned(), String("unknown".to_owned())),
                ]),
                Err(Error::prop(
                    "lineCap",
                    Error::InvalidFormat("unknown variant"),
                )),
            ),
        ];

        for (input, expected) in test_cases {
            let result = StrokeStyle::try_from(&input);
            assert_eq!(result, expected);
        }
    }
}
