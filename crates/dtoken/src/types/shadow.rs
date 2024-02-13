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

use std::collections::HashMap;

use tinyjson::JsonValue;

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

impl Shadow {
    pub fn from_map(value: &HashMap<String, JsonValue>) -> Option<Self> {
        let color_value = value.get("color")?.get::<String>()?;
        let offset_x_value = value.get("offsetX")?.get::<String>()?;
        let offset_y_value = value.get("offsetY")?.get::<String>()?;
        let blur_value = value.get("blur")?.get::<String>()?;
        let spread_value = value.get("spread")?.get::<String>()?;

        Some(Shadow {
            color: Color::from_hex(color_value)?,
            offset_x: Dimension::from_str(offset_x_value)?,
            offset_y: Dimension::from_str(offset_y_value)?,
            blur: Dimension::from_str(blur_value)?,
            spread: Dimension::from_str(spread_value)?,
        })
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
                Some(Shadow {
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
                Some(Shadow {
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
                None, // Invalid color value
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("invalid".to_owned())), // Invalid offsetX value
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                    ("spread".to_owned(), String("5px".to_owned())),
                ]),
                None, // Invalid offsetX value
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("invalid".to_owned())), // Invalid blur value
                    ("spread".to_owned(), String("5px".to_owned())),
                ]),
                None, // Invalid blur value
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                ]),
                None, // Missing spread key
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("offsetX".to_owned(), String("2px".to_owned())),
                    ("offsetY".to_owned(), String("3px".to_owned())),
                    ("blur".to_owned(), String("4px".to_owned())),
                    ("spread".to_owned(), Number(42.0)), // Invalid spread value
                ]),
                None, // Invalid spread value
            ),
        ];

        for (input, expected) in test_cases {
            let result = Shadow::from_map(&input);
            assert_eq!(result, expected);
        }
    }
}
