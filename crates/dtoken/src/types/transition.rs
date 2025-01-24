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

use std::{collections::HashMap, str::FromStr};

use tinyjson::JsonValue;

use crate::error::Error;

use super::{cubic_bezier::CubicBezier, duration::Duration};

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct Transition {
    pub duration: Duration,
    pub delay: Duration,
    pub timing_function: CubicBezier,
}

impl TryFrom<&JsonValue> for Transition {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<HashMap<_, _>>()
            .ok_or(Error::ExpectedObject)
            .and_then(Self::try_from)
    }
}

impl TryFrom<&HashMap<String, JsonValue>> for Transition {
    type Error = Error;

    fn try_from(value: &HashMap<String, JsonValue>) -> Result<Self, Self::Error> {
        let duration = value
            .get("duration")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Duration::from_str(v))
            .map_err(|err| Error::prop("duration", err))?;

        let delay = value
            .get("delay")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<String>().ok_or(Error::ExpectedString))
            .and_then(|v| Duration::from_str(v))
            .map_err(|err| Error::prop("delay", err))?;

        let timing_function = value
            .get("timingFunction")
            .ok_or(Error::MustExist)
            .and_then(|v| v.get::<Vec<_>>().ok_or(Error::ExpectedArray))
            .and_then(|v| CubicBezier::try_from(v.as_slice()))
            .map_err(|err| Error::prop("timingFunction", err))?;

        Ok(Transition {
            duration,
            delay,
            timing_function,
        })
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Transition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            duration,
            delay,
            timing_function,
        } = self;

        let new = quote::quote! { dtoken::types::transition::Transition {
            duration: #duration,
            delay: #delay,
            timing_function: #timing_function,
        }};

        tokens.extend(new);
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
                Ok(Transition {
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
                    ("duration".to_owned(), JsonValue::String("1ms".to_owned())),
                    ("delay".to_owned(), JsonValue::String("10s".to_owned())),
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
                Err(Error::prop("delay", Error::InvalidUnit(&["ms"]))),
            ),
            (
                HashMap::from([
                    ("duration".to_owned(), JsonValue::String("500ms".to_owned())),
                    ("delay".to_owned(), JsonValue::String("200ms".to_owned())),
                ]),
                Err(Error::prop("timingFunction", Error::MustExist)),
            ),
            (
                HashMap::from([
                    ("duration".to_owned(), JsonValue::String("500ms".to_owned())),
                    ("delay".to_owned(), JsonValue::String("500ms".to_owned())),
                    (
                        "timingFunction".to_owned(),
                        JsonValue::String("invalid".to_owned()),
                    ),
                ]),
                Err(Error::prop("timingFunction", Error::ExpectedArray)),
            ),
            (
                HashMap::from([
                    (
                        "duration".to_owned(),
                        JsonValue::String("invalid".to_owned()),
                    ),
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
                Err(Error::prop("duration", Error::InvalidUnit(&["ms"]))),
            ),
        ];

        for (input, expected) in test_cases {
            let result = Transition::try_from(&input);
            assert_eq!(result, expected);
        }
    }
}
