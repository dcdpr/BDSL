//! Represents the style applied to lines or borders. The $type property MUST be set to the string
//! `strokeStyle`. The value MUST be either:
//!
//! - a string value as defined in the corresponding section below, or
//! - an object value as defined in the corresponding section below
//!
//! See: <https://tr.designtokens.org/format/#stroke-style>.

use std::collections::HashMap;

use tinyjson::JsonValue;

use super::dimension::Dimension;

/// See module docs.
#[derive(Debug, Clone, PartialEq)]
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
pub enum LineCap {
    Round,
    Butt,
    Square,
}

impl StrokeStyle {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "solid" => Some(Self::Solid),
            "dashed" => Some(Self::Dashed),
            "dotted" => Some(Self::Dotted),
            "double" => Some(Self::Double),
            "groove" => Some(Self::Groove),
            "ridge" => Some(Self::Ridge),
            "outset" => Some(Self::Outset),
            "inset" => Some(Self::Inset),
            _ => None,
        }
    }

    pub fn from_map(map: &HashMap<String, JsonValue>) -> Option<Self> {
        let dash_array = map
            .get("dashArray")?
            .get::<Vec<_>>()?
            .iter()
            .map(|val| Dimension::from_str(val.get::<String>()?))
            .collect::<Option<Vec<_>>>()?;

        let line_cap = match map.get("lineCap")?.get::<String>()?.as_str() {
            "round" => LineCap::Round,
            "butt" => LineCap::Butt,
            "square" => LineCap::Square,
            _ => return None,
        };

        Some(Self::Custom {
            dash_array,
            line_cap,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("solid",   Some(StrokeStyle::Solid)),
            ("dotted",  Some(StrokeStyle::Dotted)),
            ("double",  Some(StrokeStyle::Double)),
            ("groove",  Some(StrokeStyle::Groove)),
            ("invalid", None),
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
                // Valid JSON input
                HashMap::from([
                    (
                        "dashArray".to_owned(),
                        Array(vec![String("5px".to_owned()), String("10px".to_owned())]),
                    ),
                    ("lineCap".to_owned(), String("round".to_owned())),
                ]),
                Some(StrokeStyle::Custom {
                    dash_array: vec![Dimension::Pixels(5.0), Dimension::Pixels(10.0)],
                    line_cap: LineCap::Round,
                }),
            ),
            (
                // Missing keys in JSON
                HashMap::new(),
                None,
            ),
            (
                // Invalid dashArray value in JSON
                HashMap::from([
                    (
                        "dashArray".to_owned(),
                        Array(vec![String("5px".to_owned()), String("invalid".to_owned())]),
                    ),
                    ("lineCap".to_owned(), String("round".to_owned())),
                ]),
                None,
            ),
            (
                // Unknown lineCap value in JSON
                HashMap::from([
                    (
                        "dashArray".to_owned(),
                        Array(vec![String("5px".to_owned()), String("10px".to_owned())]),
                    ),
                    ("lineCap".to_owned(), String("unknown".to_owned())),
                ]),
                None,
            ),
        ];

        for (input, expected) in test_cases {
            let result = StrokeStyle::from_map(&input);
            assert_eq!(result, expected);
        }
    }
}
