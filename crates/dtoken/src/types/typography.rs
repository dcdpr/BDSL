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

use std::collections::HashMap;

use tinyjson::JsonValue;

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

impl Typography {
    pub fn from_map(value: &HashMap<String, JsonValue>) -> Option<Self> {
        let font_family_value = value.get("fontFamily")?;
        let font_size_value = value.get("fontSize")?.get::<String>()?;
        let font_weight_value = value.get("fontWeight")?.get::<String>()?;
        let letter_spacing_value = value.get("letterSpacing")?.get::<String>()?;
        let line_height = *value.get("lineHeight")?.get::<f64>()?;

        Some(Typography {
            font_family: match font_family_value {
                JsonValue::String(v) => FontFamily::from_str(v).into(),
                JsonValue::Array(v) => FontFamily::from_slice(v)?.into(),
                _ => return None,
            },
            font_size: Dimension::from_str(font_size_value)?,
            font_weight: FontWeight::from_str(font_weight_value)?,
            letter_spacing: Dimension::from_str(letter_spacing_value)?,
            line_height,
        })
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
                Some(Typography {
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
                Some(Typography {
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
                    ("fontFamily".to_owned(), Number(123.)), // Invalid font family value
                    ("fontSize".to_owned(), String("12px".to_owned())),
                    ("fontWeight".to_owned(), String("bold".to_owned())),
                    ("letterSpacing".to_owned(), String("1px".to_owned())),
                    ("lineHeight".to_owned(), Number(1.0)),
                ]),
                None, // Invalid font family value
            ),
            (
                HashMap::from([
                    (
                        "fontFamily".to_owned(),
                        String("Arial, sans-serif".to_owned()),
                    ),
                    ("fontSize".to_owned(), String("invalid".to_owned())), // Invalid font size value
                    ("fontWeight".to_owned(), String("bold".to_owned())),
                    ("letterSpacing".to_owned(), String("1px".to_owned())),
                    ("lineHeight".to_owned(), Number(1.0)),
                ]),
                None, // Invalid font size value
            ),
            (
                HashMap::from([
                    (
                        "fontFamily".to_owned(),
                        String("Arial, sans-serif".to_owned()),
                    ),
                    ("fontSize".to_owned(), String("14px".to_owned())),
                    ("fontWeight".to_owned(), String("invalid".to_owned())), // Invalid font weight value
                    ("letterSpacing".to_owned(), String("1px".to_owned())),
                    ("lineHeight".to_owned(), Number(1.0)),
                ]),
                None, // Invalid font weight value
            ),
            (
                HashMap::from([
                    (
                        "fontFamily".to_owned(),
                        String("Arial, sans-serif".to_owned()),
                    ),
                    ("fontSize".to_owned(), String("14px".to_owned())),
                    ("fontWeight".to_owned(), String("bold".to_owned())),
                    ("letterSpacing".to_owned(), Number(1.0)), // Invalid letter spacing value
                    ("lineHeight".to_owned(), Number(1.0)),
                ]),
                None, // Invalid letter spacing value
            ),
            (
                HashMap::from([
                    (
                        "fontFamily".to_owned(),
                        String("Arial, sans-serif".to_owned()),
                    ),
                    ("fontSize".to_owned(), String("14px".to_owned())),
                    ("fontWeight".to_owned(), String("bold".to_owned())),
                    ("letterSpacing".to_owned(), String("1px".to_owned())),
                    ("lineHeight".to_owned(), String("invalid".to_owned())), // Invalid line height value
                ]),
                None, // Invalid line height value
            ),
        ];

        for (input, expected) in test_cases {
            let result = Typography::from_map(&input);
            assert_eq!(result, expected);
        }
    }
}
