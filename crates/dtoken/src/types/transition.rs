//! Represents a animated transition between two states. The $type property MUST be set to the
//! string transition. The value MUST be an object with the following properties:
//!
//! - duration: The duration of the transition. The value of this property MUST be a valid duration value or a reference to a duration token.
//! - delay: The time to wait before the transition begins. The value of this property MUST be a valid duration value or a reference to a duration token.
//! - timingFunction: The timing function of the transition. The value of this property MUST be a valid cubic bézier value or a reference to a cubic bézier token.
//!
//! Example 32: Transition composite token examples
//!
//! ```json,ignore
//! {
//!   "transition": {
//!     "emphasis": {
//!       "$type": "transition",
//!       "$value": {
//!         "duration": "200ms",
//!         "delay": "0ms",
//!         "timingFunction": [0.5, 0, 1, 1]
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! See: <https://tr.designtokens.org/format/#transition>.

use std::collections::HashMap;

use tinyjson::JsonValue;

use super::{cubic_bezier::CubicBezier, duration::Duration};

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    pub duration: Duration,
    pub delay: Duration,
    pub timing_function: CubicBezier,
}

impl Transition {
    pub fn from_map(map: &HashMap<String, JsonValue>) -> Option<Self> {
        let duration = map.get("duration")?.get::<String>()?;
        let delay = map.get("delay")?.get::<String>()?;
        let timing_function = map.get("timingFunction")?.get::<Vec<_>>()?;

        Some(Transition {
            duration: Duration::from_str(duration)?,
            delay: Duration::from_str(delay)?,
            timing_function: CubicBezier::from_slice(timing_function)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyjson::JsonValue;

    #[test]
    fn test_from_map() {
        let test_cases = vec![
            (
                HashMap::from([
                    ("duration".to_owned(), JsonValue::String("500ms".to_owned())),
                    ("delay".to_owned(), JsonValue::String("200ms".to_owned())),
                    (
                        "timingFunction".to_owned(),
                        JsonValue::Array(vec![
                            JsonValue::Number(0.1),
                            JsonValue::Number(0.2),
                            JsonValue::Number(0.3),
                            JsonValue::Number(0.4),
                        ]),
                    ),
                ]),
                Some(Transition {
                    duration: Duration {
                        milliseconds: 500.0,
                    },
                    delay: Duration {
                        milliseconds: 200.0,
                    },
                    timing_function: CubicBezier {
                        p1x: 0.1,
                        p1y: 0.2,
                        p2x: 0.3,
                        p2y: 0.4,
                    },
                }),
            ),
            (
                HashMap::from([
                    ("duration".to_owned(), JsonValue::String("1s".to_owned())),
                    ("delay".to_owned(), JsonValue::String("0s".to_owned())),
                    (
                        "timingFunction".to_owned(),
                        JsonValue::Array(vec![
                            JsonValue::Number(0.0),
                            JsonValue::Number(0.5),
                            JsonValue::Number(1.0),
                            JsonValue::Number(0.9),
                        ]),
                    ),
                ]),
                None, // Invalid duration units
            ),
            (
                HashMap::from([
                    ("duration".to_owned(), JsonValue::String("500ms".to_owned())),
                    ("delay".to_owned(), JsonValue::String("200ms".to_owned())),
                ]),
                None, // Missing timingFunction key
            ),
            (
                HashMap::from([
                    ("duration".to_owned(), JsonValue::String("500ms".to_owned())),
                    (
                        "timingFunction".to_owned(),
                        JsonValue::String("invalid".to_owned()),
                    ), // Invalid timingFunction value
                ]),
                None, // Invalid timingFunction value
            ),
            (
                HashMap::from([
                    (
                        "duration".to_owned(),
                        JsonValue::String("invalid".to_owned()),
                    ), // Invalid duration value
                    ("delay".to_owned(), JsonValue::String("200ms".to_owned())),
                    (
                        "timingFunction".to_owned(),
                        JsonValue::Array(vec![
                            JsonValue::Number(0.1),
                            JsonValue::Number(0.2),
                            JsonValue::Number(0.3),
                            JsonValue::Number(0.4),
                        ]),
                    ),
                ]),
                None, // Invalid duration value
            ),
        ];

        for (input, expected) in test_cases {
            let result = Transition::from_map(&input);
            assert_eq!(result, expected);
        }
    }
}
