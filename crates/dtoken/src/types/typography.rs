//! Represents a typographic style. The $type property MUST be set to the string typography. The
//! value MUST be an object with the following properties:
//!
//! - fontFamily: The typography's font. The value of this property MUST be a valid font family
//!   value or a reference to a font family token.
//! - fontSize: The size of the typography. The value of this property MUST be a valid dimension
//!   value or a reference to a dimension token.
//! - fontWeight: The weight of the typography. The value of this property MUST be a valid font
//!   weight or a reference to a font weight token.
//! - letterSpacing: The horizontal spacing between characters. The value of this property MUST be
//!   a valid dimension value or a reference to a dimension token.
//! - lineHeight: The vertical spacing between lines of typography. The value of this property MUST
//!   be a valid number value or a reference to a number token. The number SHOULD be interpreted as
//!   a multiplier of the fontSize.
//!
//! Example 37: Typography composite token examples
//!
//! ```json,ignore
//! {
//!   "type styles": {
//!     "heading-level-1": {
//!       "$type": "typography",
//!       "$value": {
//!         "fontFamily": "Roboto",
//!         "fontSize": "42px",
//!         "fontWeight": 700,
//!         "letterSpacing": "0.1px",
//!         "lineHeight": 1.2
//!       }
//!     },
//!     "microcopy": {
//!       "$type": "typography",
//!       "$value": {
//!         "fontFamily": "{font.serif}",
//!         "fontSize": "{font.size.smallest}",
//!         "fontWeight": "{font.weight.normal}",
//!         "letterSpacing": "0px",
//!         "lineHeight": 1
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! See: <https://tr.designtokens.org/format/#typography>.

use std::{collections::HashMap, str::FromStr};

use tinyjson::JsonValue;

use crate::error::Error;

use super::{dimension::Dimension, font_family::FontFamily, font_weight::FontWeight};

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Typography {
    pub font_family: FontFamily,
    pub font_size: Dimension,
    pub font_weight: FontWeight,
    pub letter_spacing: Dimension,
    pub line_height: f64,
}

impl TryFrom<&JsonValue> for Typography {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<HashMap<_, _>>()
            .ok_or(Error::ExpectedObject)
            .and_then(Self::try_from)
    }
}

impl TryFrom<&HashMap<String, JsonValue>> for Typography {
    type Error = Error;

    fn try_from(value: &HashMap<String, JsonValue>) -> Result<Self, Self::Error> {
        let font_family = value
            .get("fontFamily")
            .ok_or(Error::MustExist)
            .and_then(|v| match v {
                JsonValue::String(v) => Ok(FontFamily::primary(v)),
                JsonValue::Array(v) => FontFamily::try_from(v.as_slice()),
                _ => Err(Error::UnexpectedType),
            })
            .map_err(|err| Error::prop("fontFamily", err))?;

        let font_size = value
            .get("fontSize")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Dimension::from_str(v))
            .map_err(|err| Error::prop("fontSize", err))?;

        let font_weight = value
            .get("fontWeight")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| FontWeight::from_str(v))
            .map_err(|err| Error::prop("fontWeight", err))?;

        let letter_spacing = value
            .get("letterSpacing")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Dimension::from_str(v))
            .map_err(|err| Error::prop("letterSpacing", err))?;

        let line_height = *value
            .get("lineHeight")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<f64>().ok_or(Error::ExpectedNumber))
            .map_err(|err| Error::prop("lineHeight", err))?;

        Ok(Typography {
            font_family,
            font_size,
            font_weight,
            letter_spacing,
            line_height,
        })
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Typography {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            font_family,
            font_size,
            font_weight,
            letter_spacing,
            line_height,
        } = self;

        let new = quote::quote! { dtoken::types::typography::Typography {
            font_family: #font_family,
            font_size: #font_size,
            font_weight: #font_weight,
            letter_spacing: #letter_spacing,
            line_height: #line_height,
        }};

        tokens.extend(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyjson::JsonValue::{Array, Number, String};

    #[test]
    fn test_typography_from_map() {
        let test_cases = vec![
            (
                HashMap::from([
                    (
                        "fontFamily".to_owned(),
                        String("Arial, sans-serif".to_owned()),
                    ),
                    ("fontSize".to_owned(), String("16px".to_owned())),
                    ("fontWeight".to_owned(), String("bold".to_owned())),
                    ("letterSpacing".to_owned(), String("1px".to_owned())),
                    ("lineHeight".to_owned(), Number(1.5)),
                ]),
                Ok(Typography {
                    font_family: FontFamily {
                        primary: "Arial, sans-serif".to_owned(),
                        fallbacks: vec![],
                    },
                    font_size: Dimension::Pixels(16.0),
                    font_weight: FontWeight::from_str("bold").unwrap(),
                    letter_spacing: Dimension::Pixels(1.0),
                    line_height: 1.5,
                }),
            ),
            (
                HashMap::from([
                    (
                        "fontFamily".to_owned(),
                        Array(vec![
                            String("Arial".to_owned()),
                            String("sans-serif".to_owned()),
                        ]),
                    ),
                    ("fontSize".to_owned(), String("14px".to_owned())),
                    ("fontWeight".to_owned(), String("normal".to_owned())),
                    ("letterSpacing".to_owned(), String("0.5px".to_owned())),
                    ("lineHeight".to_owned(), Number(1.2)),
                ]),
                Ok(Typography {
                    font_family: FontFamily {
                        primary: "Arial".to_owned(),
                        fallbacks: vec!["sans-serif".to_owned()],
                    },
                    font_size: Dimension::Pixels(14.0),
                    font_weight: FontWeight::from_str("normal").unwrap(),
                    letter_spacing: Dimension::Pixels(0.5),
                    line_height: 1.2,
                }),
            ),
            (
                HashMap::from([
                    ("fontFamily".to_owned(), Number(123.)),
                    ("fontSize".to_owned(), String("12px".to_owned())),
                    ("fontWeight".to_owned(), String("bold".to_owned())),
                    ("letterSpacing".to_owned(), String("1px".to_owned())),
                    ("lineHeight".to_owned(), Number(1.0)),
                ]),
                Err(Error::prop("fontFamily", Error::UnexpectedType)),
            ),
        ];

        for (input, expected) in test_cases {
            let result = Typography::try_from(&input);
            assert_eq!(result, expected);
        }
    }
}
