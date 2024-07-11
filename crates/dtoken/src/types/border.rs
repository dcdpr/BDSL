//! Represents a border style. The $type property MUST be set to the string border. The value MUST be an object with the following properties:
//!
//! - color: The color of the border. The value of this property MUST be a valid color value or a reference to a color token.
//! - width: The width or thickness of the border. The value of this property MUST be a valid dimension value or a reference to a dimension token.
//! - style: The border's style. The value of this property MUST be a valid stroke style value or a reference to a stroke style token.
//!
//! Example 31: Border composite token examples
//!
//! ```json,ignore
//! {
//!   "border": {
//!     "heavy": {
//!       "$type": "border",
//!       "$value": {
//!         "color": "#36363600",
//!         "width": "3px",
//!         "style": "solid"
//!       }
//!     },
//!     "focusring": {
//!       "$type": "border",
//!       "$value": {
//!         "color": "{color.focusring}",
//!         "width": "1px",
//!         "style": {
//!           "dashArray": ["0.5rem", "0.25rem"],
//!           "lineCap": "round"
//!         }
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! See: <https://tr.designtokens.org/format/#border>.

use std::{collections::HashMap, str::FromStr};

use tinyjson::JsonValue;

use crate::error::Error;

use super::{color::Color, dimension::Dimension, stroke_style::StrokeStyle};

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub color: Color,
    pub width: Dimension,
    pub style: StrokeStyle,
}

impl TryFrom<&JsonValue> for Border {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<HashMap<_, _>>()
            .ok_or(Error::ExpectedObject)
            .and_then(Self::try_from)
    }
}

impl TryFrom<&HashMap<String, JsonValue>> for Border {
    type Error = Error;

    fn try_from(map: &HashMap<String, JsonValue>) -> Result<Self, Self::Error> {
        let color = map
            .get("color")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Color::from_hex(v))
            .map_err(|err| Error::prop("color", err))?;

        let width = map
            .get("width")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Dimension::from_str(v))
            .map_err(|err| Error::prop("width", err))?;

        let style = map
            .get("style")
            .ok_or(Error::MustExist)
            .and_then(|v| match v {
                JsonValue::String(v) => StrokeStyle::from_str(v),
                JsonValue::Object(v) => StrokeStyle::try_from(v),
                _ => Err(Error::ExpectedString),
            })
            .map_err(|err| Error::prop("style", err))?;

        Ok(Border {
            color,
            width,
            style,
        })
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Border {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            color,
            width,
            style,
        } = self;

        let new = quote::quote! { dtoken::types::border::Border {
            color: #color,
            width: #width,
            style: #style,
        }};

        tokens.extend(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use JsonValue::{Number, String};

    #[test]
    fn test_from_map() {
        let test_cases = vec![
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("width".to_owned(), String("2px".to_owned())),
                    ("style".to_owned(), String("dotted".to_owned())),
                ]),
                Ok(Border {
                    color: Color {
                        r: 255,
                        g: 87,
                        b: 51,
                        a: 255,
                    },
                    width: Dimension::from_str("2px").unwrap(),
                    style: StrokeStyle::from_str("dotted").unwrap(),
                }),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#00FF00".to_owned())),
                    ("width".to_owned(), String("1rem".to_owned())),
                    ("style".to_owned(), String("solid".to_owned())),
                ]),
                Ok(Border {
                    color: Color {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    },
                    width: Dimension::from_str("1rem").unwrap(),
                    style: StrokeStyle::from_str("solid").unwrap(),
                }),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#12345".to_owned())),
                    ("width".to_owned(), String("2px".to_owned())),
                    ("style".to_owned(), String("dashed".to_owned())),
                ]),
                Err(Error::prop(
                    "color",
                    Error::InvalidFormat("must be 6 or 8 characters long"),
                )),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("width".to_owned(), String("invalid".to_owned())),
                    ("style".to_owned(), String("dotted".to_owned())),
                ]),
                Err(Error::prop("width", Error::InvalidUnit(&["px", "rem"]))),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("width".to_owned(), String("2px".to_owned())),
                ]),
                Err(Error::prop("style", Error::MustExist)),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("width".to_owned(), String("2px".to_owned())),
                    ("style".to_owned(), Number(42.0)),
                ]),
                Err(Error::prop("style", Error::ExpectedString)),
            ),
        ];

        for (input, expected) in test_cases {
            let result = Border::try_from(&input);
            assert_eq!(result, expected);
        }
    }
}
