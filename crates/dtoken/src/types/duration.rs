//! Represents the length of time in milliseconds an animation or animation cycle takes to
//! complete, such as 200 milliseconds. The $type property MUST be set to the string duration. The
//! value MUST be a string containing a number (either integer or floating-point) followed by an
//! "ms" unit. A millisecond is a unit of time equal to one thousandth of a second.
//!
//! For example:
//!
//! EXAMPLE 21
//! ```json,ignore
//! {
//!   "Duration-100": {
//!     "$value": "100ms",
//!     "$type": "duration"
//!   },
//!   "Duration-200": {
//!     "$value": "200ms",
//!     "$type": "duration"
//!   }
//! }
//! ```
//!
//! See: <https://tr.designtokens.org/format/#duration>.

use std::str::FromStr;

use tinyjson::JsonValue;

use crate::error::Error;

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    pub milliseconds: f64,
}

impl TryFrom<&JsonValue> for Duration {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<String>()
            .ok_or(Error::ExpectedString)
            .and_then(|v| Self::from_str(v))
    }
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('-') {
            return Err(Error::NumberMustBePositive);
        }

        s.strip_suffix("ms")
            .ok_or(Error::InvalidUnit(&["ms"]))
            .and_then(|v| v.parse::<f64>().map_err(Error::from))
            .map(|milliseconds| Duration { milliseconds })
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Duration {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { milliseconds } = self;

        tokens.extend(quote::quote! { dtoken::types::duration::Duration {
            milliseconds: #milliseconds,
        }});
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let test_cases = vec![
            ("10ms", Ok(Duration { milliseconds: 10.0 })),
            ("2.5ms", Ok(Duration { milliseconds: 2.5 })),
            ("0.1ms", Ok(Duration { milliseconds: 0.1 })),
            (
                "ms",
                Err(Error::InvalidNumber(
                    "cannot parse float from empty string".to_owned(),
                )),
            ),
            (
                "abcms",
                Err(Error::InvalidNumber("invalid float literal".to_owned())),
            ),
            ("200s", Err(Error::InvalidUnit(&["ms"]))),
            ("", Err(Error::InvalidUnit(&["ms"]))),
            ("1000", Err(Error::InvalidUnit(&["ms"]))),
            ("-5ms", Err(Error::NumberMustBePositive)), // Negative value not supported
            (
                "1.23.45ms",
                Err(Error::InvalidNumber("invalid float literal".to_owned())),
            ),
        ];

        for (input, expected) in test_cases {
            let result = Duration::from_str(input);
            assert_eq!(result, expected);
        }
    }
}
