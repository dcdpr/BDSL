//! Represents a color gradient. The $type property MUST be set to the string gradient. The value
//! MUST be an array of objects representing gradient stops that have the following structure:
//!
//! - color: The color value at the stop's position on the gradient. The value of this property
//!   MUST be a valid color value or a reference to a color token.
//! - position: The position of the stop along the gradient's axis. The value of this property MUST
//!   be a valid number value or reference to a number token. The number values must be in the
//!   range [0, 1], where 0 represents the start position of the gradient's axis and 1 the end
//!   position. If a number value outside of that range is given, it MUST be considered as if it
//!   were clamped to the range [0, 1]. For example, a value of 42 should be treated as if it were
//!   1, i.e. the end position of the gradient axis. Similarly, a value of -99 should be treated as
//!   if it were 0, i.e. the start position of the gradient axis.
//!
//! If there are no stops at the very beginning or end of the gradient axis (i.e. with position 0
//! or 1, respectively), then the color from the stop closest to each end should be extended to
//! that end of the axis.
//!
//! Example 34: Gradient token example
//!
//! ```json,ignore
//! {
//!   "blue-to-red": {
//!     "$type": "gradient",
//!     "$value": [
//!       {
//!         "color": "#0000ff",
//!         "position": 0
//!       },
//!       {
//!         "color": "#ff0000",
//!         "position": 1
//!       }
//!     ]
//!   }
//! }
//! ```
//!
//! Describes a gradient that goes from blue to red:
//!
//! Example 35: Gradient token with omitted start stop example
//!
//! ```json.ignore
//! {
//!   "mostly-yellow": {
//!     "$type": "gradient",
//!     "$value": [
//!       {
//!         "color": "#ffff00",
//!         "position": 0.666
//!       },
//!       {
//!         "color": "#ff0000",
//!         "position": 1
//!       }
//!     ]
//!   }
//! }
//! ```
//!
//! Describes a gradient that is solid yellow for the first 2/3 and then fades to red:
//!
//! Example 36: Gradient token using references example
//!
//! ```json,ignore
//! {
//!   "brand-primary": {
//!     "$type": "color",
//!     "$value": "#99ff66"
//!   },
//!
//!   "position-end": {
//!     "$type": "number",
//!     "$value": 1
//!   },
//!
//!   "brand-in-the-middle": {
//!     "$type": "gradient",
//!     "$value": [
//!       {
//!         "color": "#000000",
//!         "position": 0
//!       },
//!       {
//!         "color": "{brand-primary}",
//!         "position": 0.5
//!       },
//!       {
//!         "color": "#000000",
//!         "position": "{position-end}"
//!       }
//!     ]
//!   }
//! }
//! ```
//!
//! Describes a color token called "brand-primary", which is referenced as the mid-point of a
//! gradient is black at either end.
//!
//! See: <https://tr.designtokens.org/format/#gradient>.

use std::collections::HashMap;

use tinyjson::JsonValue;

use super::color::Color;

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    pub stops: Vec<GradientStop>,
}

impl Gradient {
    pub fn from_slice(value: &[JsonValue]) -> Option<Self> {
        let stops = value
            .iter()
            .map(|v| GradientStop::from_map(v.get()?))
            .collect::<Option<Vec<_>>>()?;

        match stops.is_empty() {
            true => None,
            false => Some(Gradient { stops }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub color: Color,
    pub position: f64,
}

impl GradientStop {
    pub fn from_map(value: &HashMap<String, JsonValue>) -> Option<Self> {
        let color_value = value.get("color")?.get::<String>()?;
        let position = *value.get("position")?.get::<f64>()?;

        if position < 0.0 || position > 1.0 {
            return None; // Invalid position value
        }

        Some(GradientStop {
            color: Color::from_hex(color_value)?,
            position,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyjson::JsonValue::{Number, String};

    #[test]
    fn test_gradient_from_slice() {
        let test_cases = vec![
            (
                vec![
                    JsonValue::Object(HashMap::from([
                        ("color".to_owned(), String("#FF5733".to_owned())),
                        ("position".to_owned(), Number(0.1)),
                    ])),
                    JsonValue::Object(HashMap::from([
                        ("color".to_owned(), String("#00FF00".to_owned())),
                        ("position".to_owned(), Number(0.5)),
                    ])),
                    JsonValue::Object(HashMap::from([
                        ("color".to_owned(), String("#0000FF".to_owned())),
                        ("position".to_owned(), Number(0.9)),
                    ])),
                ],
                Some(Gradient {
                    stops: vec![
                        GradientStop {
                            color: Color {
                                r: 255,
                                g: 87,
                                b: 51,
                                a: 255,
                            },
                            position: 0.1,
                        },
                        GradientStop {
                            color: Color {
                                r: 0,
                                g: 255,
                                b: 0,
                                a: 255,
                            },
                            position: 0.5,
                        },
                        GradientStop {
                            color: Color {
                                r: 0,
                                g: 0,
                                b: 255,
                                a: 255,
                            },
                            position: 0.9,
                        },
                    ],
                }),
            ),
            (
                vec![],
                None, // Empty input
            ),
            (
                vec![
                    JsonValue::Object(HashMap::from([(
                        "color".to_owned(),
                        String("#FF5733".to_owned()),
                    )])),
                    JsonValue::Object(HashMap::from([("position".to_owned(), Number(0.5))])),
                ],
                None, // Missing position for some stops
            ),
            (
                vec![
                    JsonValue::Object(HashMap::from([
                        ("color".to_owned(), String("#FF5733".to_owned())),
                        ("position".to_owned(), String("invalid".to_owned())), // Invalid position value
                    ])),
                    JsonValue::Object(HashMap::from([
                        ("color".to_owned(), String("#00FF00".to_owned())),
                        ("position".to_owned(), Number(0.5)),
                    ])),
                ],
                None, // Invalid position value for one stop
            ),
            (
                vec![
                    JsonValue::Object(HashMap::from([
                        ("color".to_owned(), String("#FF5733".to_owned())),
                        ("position".to_owned(), Number(-0.1)), // Out of range position value
                    ])),
                    JsonValue::Object(HashMap::from([
                        ("color".to_owned(), String("#00FF00".to_owned())),
                        ("position".to_owned(), Number(1.1)), // Out of range position value
                    ])),
                ],
                None, // Out of range position values for some stops
            ),
        ];

        for (input, expected) in test_cases {
            let result = Gradient::from_slice(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_gradient_stop_from_map() {
        let test_cases = vec![
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("position".to_owned(), Number(0.1)),
                ]),
                Some(GradientStop {
                    color: Color {
                        r: 255,
                        g: 87,
                        b: 51,
                        a: 255,
                    },
                    position: 0.1,
                }),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#00FF00".to_owned())),
                    ("position".to_owned(), Number(0.5)),
                ]),
                Some(GradientStop {
                    color: Color {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    },
                    position: 0.5,
                }),
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#12345".to_owned())), // Invalid hex value
                    ("position".to_owned(), Number(0.7)),
                ]),
                None, // Invalid color value
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("position".to_owned(), String("invalid".to_owned())), // Invalid position value
                ]),
                None, // Invalid position value
            ),
            (
                HashMap::from([
                    ("color".to_owned(), String("#FF5733".to_owned())),
                    ("position".to_owned(), Number(-0.1)), // Out of range position value
                ]),
                None, // Out of range position value
            ),
        ];

        for (input, expected) in test_cases {
            let result = GradientStop::from_map(&input);
            assert_eq!(result, expected);
        }
    }
}
