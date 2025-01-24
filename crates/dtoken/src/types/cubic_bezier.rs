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

use crate::error::Error;

/// See module-level documentation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CubicBezier {
    pub p1x: f64,
    pub p1y: f64,
    pub p2x: f64,
    pub p2y: f64,
}

impl TryFrom<&JsonValue> for CubicBezier {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<Vec<_>>()
            .ok_or(Error::ExpectedArray)
            .and_then(|v| Self::try_from(v.as_slice()))
    }
}

impl TryFrom<&[JsonValue]> for CubicBezier {
    type Error = Error;

    fn try_from(value: &[JsonValue]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(Error::CollectionLength(4));
        }

        #[allow(clippy::get_first)]
        let p1x = *value[0].get::<f64>().ok_or(Error::ExpectedItemNumber)?;
        let p1y = *value[1].get::<f64>().ok_or(Error::ExpectedItemNumber)?;
        let p2x = *value[2].get::<f64>().ok_or(Error::ExpectedItemNumber)?;
        let p2y = *value[3].get::<f64>().ok_or(Error::ExpectedItemNumber)?;

        // Validating the x coordinates are within [0, 1]
        if (0.0..=1.0).contains(&p1x) && (0.0..=1.0).contains(&p2x) {
            Ok(CubicBezier { p1x, p1y, p2x, p2y })
        } else {
            Err(Error::NumberWithin(0, 1))
        }
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for CubicBezier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { p1x, p1y, p2x, p2y } = self;

        let new = quote::quote! { dtoken::types::cubic_bezier::CubicBezier {
            p1x: #p1x,
            p1y: #p1y,
            p2x: #p2x,
            p2y: #p2y,
        }};

        tokens.extend(new);
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
                Ok(CubicBezier {
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
                Ok(CubicBezier {
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
                Err(Error::NumberWithin(0, 1)),
            ),
            (
                vec![
                    JsonValue::Number(0.1),
                    JsonValue::Number(0.2),
                    JsonValue::String("invalid".to_owned()),
                    JsonValue::Number(0.4),
                ],
                Err(Error::ExpectedItemNumber),
            ),
            (
                vec![
                    JsonValue::Number(0.1),
                    JsonValue::Number(0.2),
                    JsonValue::Number(0.3),
                ],
                Err(Error::CollectionLength(4)),
            ),
            (
                vec![
                    JsonValue::Number(0.1),
                    JsonValue::Number(0.2),
                    JsonValue::Number(0.3),
                    JsonValue::Number(0.4),
                    JsonValue::Number(0.5),
                ],
                Err(Error::CollectionLength(4)),
            ),
            (vec![], Err(Error::CollectionLength(4))),
        ];

        for (input, expected) in test_cases {
            let result = CubicBezier::try_from(input.as_slice());
            assert_eq!(result, expected);
        }
    }
}
