//! Represents how the value of an animated property progresses towards completion over the
//! duration of an animation, effectively creating visual effects such as acceleration,
//! deceleration, and bounce. The $type property MUST be set to the string cubicBezier. The value
//! MUST be an array containing four numbers. These numbers represent two points (P1, P2) with one
//! x coordinate and one y coordinate each [P1x, P1y, P2x, P2y]. The y coordinates of P1 and P2 can
//! be any real number in the range [-∞, ∞], but the x coordinates are restricted to the range [0,
//! 1].
//!
//! For example:
//! Example 22
//!
//! ```json,ignore
//! {
//!   "Accelerate": {
//!     "$value": [0.5, 0, 1, 1],
//!     "$type": "cubicBezier"
//!   },
//!   "Decelerate": {
//!     "$value": [0, 0, 0.5, 1],
//!     "$type": "cubicBezier"
//!   }
//! }
//! ```
//!
//! See: <https://tr.designtokens.org/format/#cubic-bezier>.

use tinyjson::JsonValue;

/// See module-level documentation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier {
    pub p1x: f64,
    pub p1y: f64,
    pub p2x: f64,
    pub p2y: f64,
}

impl CubicBezier {
    pub fn from_slice(slice: &[JsonValue]) -> Option<Self> {
        if slice.len() != 4 {
            return None;
        }

        let p1x = *slice.get(0)?.get::<f64>()?;
        let p1y = *slice.get(1)?.get::<f64>()?;
        let p2x = *slice.get(2)?.get::<f64>()?;
        let p2y = *slice.get(3)?.get::<f64>()?;

        // Validating the x coordinates are within [0, 1]
        if (0.0..=1.0).contains(&p1x) && (0.0..=1.0).contains(&p2x) {
            Some(CubicBezier { p1x, p1y, p2x, p2y })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let test_cases = vec![
            (
                vec![
                    JsonValue::Number(0.1),
                    JsonValue::Number(0.2),
                    JsonValue::Number(0.3),
                    JsonValue::Number(0.4),
                ],
                Some(CubicBezier {
                    p1x: 0.1,
                    p1y: 0.2,
                    p2x: 0.3,
                    p2y: 0.4,
                }),
            ),
            (
                vec![
                    JsonValue::Number(0.0),
                    JsonValue::Number(0.5),
                    JsonValue::Number(1.0),
                    JsonValue::Number(0.9),
                ],
                Some(CubicBezier {
                    p1x: 0.0,
                    p1y: 0.5,
                    p2x: 1.0,
                    p2y: 0.9,
                }),
            ),
            (
                vec![
                    JsonValue::Number(0.2),
                    JsonValue::Number(0.3),
                    JsonValue::Number(1.1),
                    JsonValue::Number(0.7),
                ],
                None, // Invalid x coordinate (out of range)
            ),
            (
                vec![
                    JsonValue::Number(0.1),
                    JsonValue::Number(0.2),
                    JsonValue::String("invalid".to_owned()),
                    JsonValue::Number(0.4),
                ],
                None, // Invalid JSON value (not a number)
            ),
            (
                vec![
                    JsonValue::Number(0.1),
                    JsonValue::Number(0.2),
                    JsonValue::Number(0.3),
                ],
                None, // Insufficient number of values
            ),
            (
                vec![
                    JsonValue::Number(0.1),
                    JsonValue::Number(0.2),
                    JsonValue::Number(0.3),
                    JsonValue::Number(0.4),
                    JsonValue::Number(0.5),
                ],
                None, // Excess number of values
            ),
            (
                vec![],
                None, // Empty input
            ),
        ];

        for (input, expected) in test_cases {
            let result = CubicBezier::from_slice(&input);
            assert_eq!(result, expected);
        }
    }
}
