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

use std::collections::HashMap;

use tinyjson::JsonValue;

use super::{color::Color, dimension::Dimension, stroke_style::StrokeStyle};

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub color: Color,
    pub width: Dimension,
    pub style: StrokeStyle,
}

impl Border {
    pub fn from_map(map: &HashMap<String, JsonValue>) -> Option<Self> {
        let color_value = map.get("color")?.get::<String>()?;
        let width_value = map.get("width")?.get::<String>()?;
        let style_value = map.get("style")?;

        Some(Border {
            color: Color::from_hex(color_value)?,
            width: Dimension::from_str(width_value)?,
            style: match style_value {
                JsonValue::String(v) => StrokeStyle::from_str(v)?.into(),
                JsonValue::Object(v) => StrokeStyle::from_map(v)?.into(),
                _ => return None,
            },
        })
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
                Some(Border {
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
                Some(Border {
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
                    ("color".to_owned(), String("#12345".to_owned())), // Invalid hex value
                    ("width".to_owned(), String("2px".to_owned())),
                    ("style".to_owned(), String("dashed".to_owned())),
                ]),
                None, // Invalid color value
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("width".to_owned(), String("invalid".to_owned())), // Invalid width value
                    ("style".to_owned(), String("dotted".to_owned())),
                ]),
                None, // Invalid width value
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("width".to_owned(), String("2px".to_owned())),
                ]),
                None, // Missing style key
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("width".to_owned(), String("2px".to_owned())),
                    ("style".to_owned(), Number(42.0)), // Invalid style value
                ]),
                None, // Invalid style value
            ),
        ];

        for (input, expected) in test_cases {
            let result = Border::from_map(&input);
            assert_eq!(result, expected);
        }
    }
}
