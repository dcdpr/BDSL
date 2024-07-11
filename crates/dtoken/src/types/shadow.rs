//! Represents a shadow style. The $type property MUST be set to the string shadow. The value must
//! be an object with the following properties:
//!
//! - color: The color of the shadow. The value of this property MUST be a valid color value or a reference to a color token.
//! - offsetX: The horizontal offset that shadow has from the element it is applied to. The value of this property MUST be a valid dimension value or a reference to a dimension token.
//! - offsetY: The vertical offset that shadow has from the element it is applied to. The value of this property MUST be a valid dimension value or a reference to a dimension token.
//! - blur: The blur radius that is applied to the shadow. The value of this property MUST be a valid dimension value or a reference to a dimension token.
//! - spread: The amount by which to expand or contract the shadow. The value of this property MUST be a valid dimension value or a reference to a dimension token.
//!
//! Example 33: Shadow token example
//!
//! ```json,ignore
//! {
//!   "shadow-token": {
//!     "$type": "shadow",
//!     "$value": {
//!       "color": "#00000080",
//!       "offsetX": "0.5rem",
//!       "offsetY": "0.5rem",
//!       "blur": "1.5rem",
//!       "spread": "0rem"
//!     }
//!   }
//! }
//! ```

use std::{collections::HashMap, str::FromStr};

use tinyjson::JsonValue;

use crate::error::Error;

use super::{color::Color, dimension::Dimension};

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Shadow {
    pub color: Color,
    pub offset_x: Dimension,
    pub offset_y: Dimension,
    pub blur: Dimension,
    pub spread: Dimension,
}

impl TryFrom<&JsonValue> for Shadow {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<HashMap<_, _>>()
            .ok_or(Error::ExpectedObject)
            .and_then(Self::try_from)
    }
}

impl TryFrom<&HashMap<String, JsonValue>> for Shadow {
    type Error = Error;

    fn try_from(value: &HashMap<String, JsonValue>) -> Result<Self, Self::Error> {
        let color = value
            .get("color")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Color::from_hex(v))
            .map_err(|err| Error::prop("color", err))?;

        let offset_x = value
            .get("offsetX")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Dimension::from_str(v))
            .map_err(|err| Error::prop("offsetX", err))?;

        let offset_y = value
            .get("offsetY")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Dimension::from_str(v))
            .map_err(|err| Error::prop("offsetY", err))?;

        let blur = value
            .get("blur")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Dimension::from_str(v))
            .map_err(|err| Error::prop("blur", err))?;

        let spread = value
            .get("spread")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Dimension::from_str(v))
            .map_err(|err| Error::prop("spread", err))?;

        Ok(Shadow {
            color,
            offset_x,
            offset_y,
            blur,
            spread,
        })
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Shadow {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            color,
            offset_x,
            offset_y,
            blur,
            spread,
        } = self;

        let new = quote::quote! { dtoken::types::shadow::Shadow {
            color: #color,
            offset_x: #offset_x,
            offset_y: #offset_y,
            blur: #blur,
            spread: #spread,
        }};

        tokens.extend(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyjson::JsonValue;

    use JsonValue::{Number, String};

    #[test]
    fn test_from_map() {
        let test_cases = vec![
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                    ("spread".to_owned(), String("5px".to_owned())),
                ]),
                Ok(Shadow {
                    color: Color {
                        r: 255,
                        g: 87,
                        b: 51,
                        a: 255,
                    },
                    offset_x: Dimension::from_str("2px").unwrap(),
                    offset_y: Dimension::from_str("3px").unwrap(),
                    blur: Dimension::from_str("4px").unwrap(),
                    spread: Dimension::from_str("5px").unwrap(),
                }),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#00FF00".to_owned())),
                    ("offsetX".to_owned(), String("1rem".to_owned())),
                    ("offsetY".to_owned(), String("0rem".to_owned())),
                    ("blur".to_owned(), String("0rem".to_owned())),
                    ("spread".to_owned(), String("0rem".to_owned())),
                ]),
                Ok(Shadow {
                    color: Color {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    },
                    offset_x: Dimension::from_str("1rem").unwrap(),
                    offset_y: Dimension::from_str("0rem").unwrap(),
                    blur: Dimension::from_str("0rem").unwrap(),
                    spread: Dimension::from_str("0rem").unwrap(),
                }),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#12345".to_owned())), // Invalid hex value
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                    ("spread".to_owned(), String("5px".to_owned())),
                ]),
                Err(Error::prop(
                    "color",
                    Error::InvalidFormat("must be 6 or 8 characters long"),
                )),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("invalid".to_owned())), // Invalid offsetX value
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                    ("spread".to_owned(), String("5px".to_owned())),
                ]),
                Err(Error::prop("offsetX", Error::InvalidUnit(&["px", "rem"]))),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("invalid".to_owned())), // Invalid blur value
                    ("spread".to_owned(), String("5px".to_owned())),
                ]),
                Err(Error::prop("blur", Error::InvalidUnit(&["px", "rem"]))),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                ]),
                Err(Error::prop("spread", Error::MustExist)),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                    ("spread".to_owned(), Number(42.0)), // Invalid spread value
                ]),
                Err(Error::prop("spread", Error::ExpectedString)),
            ),
        ];

        for (input, expected) in test_cases {
            let result = Shadow::try_from(&input);
            assert_eq!(result, expected);
        }
    }
}
