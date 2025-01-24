//! Represents an amount of distance in a single dimension in the UI, such as a position, width,
//! height, radius, or thickness. The $type property MUST be set to the string dimension. The value
//! must be a string containing a number (either integer or floating-point) followed by either a
//! "px" or "rem" unit (future spec iterations may add support for additional units). This includes
//! 0 which also MUST be followed by either a "px" or "rem" unit.
//!
//! For example:
//!
//! EXAMPLE 18
//! ```json,ignore
//! {
//!   "spacing-stack-0": {
//!     "$value": "0rem",
//!     "$type": "dimension"
//!   },
//!   "spacing-stack-1": {
//!     "$value": "0.25rem",
//!     "$type": "dimension"
//!   }
//! }
//! ```
//!
//! The "px" and "rem" units are to be interpreted the same way they are in CSS:
//!
//! - px: Represents an idealized pixel on the viewport. The equivalent in Android is dp and iOS is
//!       pt. Translation tools SHOULD therefore convert to these or other equivalent units as
//!       needed.
//! - rem: Represents a multiple of the system's default font size (which MAY be configurable by
//!        the user). 1rem is 100% of the default font size. The equivalent of 1rem on Android is
//!        16sp. Not all platforms have an equivalent to rem, so translation tools MAY need to do a
//!        lossy conversion to a fixed px size by assuming a default font size (usually 16px) for
//!        such platforms.
//!
//! See: <https://tr.designtokens.org/format/#dimension>.

use std::str::FromStr;

use tinyjson::JsonValue;

use crate::error::Error;

/// See module docs.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum Dimension {
    Pixels(f64),
    Rems(f64),
}

impl Dimension {
    #[must_use]
    pub fn as_px(&self) -> Option<f64> {
        match self {
            Self::Pixels(v) => Some(*v),
            Self::Rems(_) => None,
        }
    }

    #[must_use]
    pub fn as_rem(&self) -> Option<f64> {
        match self {
            Self::Pixels(_) => None,
            Self::Rems(v) => Some(*v),
        }
    }
}

impl TryFrom<&JsonValue> for Dimension {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<String>()
            .ok_or(Error::ExpectedString)
            .and_then(|v| Self::from_str(v))
    }
}

impl FromStr for Dimension {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('-') {
            return Err(Error::NumberMustBePositive);
        }

        if let Some(s) = s.strip_suffix("px") {
            s.parse::<f64>().map_err(Error::from).map(Dimension::Pixels)
        } else if let Some(s) = s.strip_suffix("rem") {
            s.parse::<f64>().map_err(Error::from).map(Dimension::Rems)
        } else {
            Err(Error::InvalidUnit(&["px", "rem"]))
        }
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Dimension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let new = match self {
            Dimension::Pixels(v) => {
                quote::quote! { dtoken::types::dimension::Dimension::Pixels(#v) }
            }
            Dimension::Rems(v) => {
                quote::quote! { dtoken::types::dimension::Dimension::Rems(#v) }
            }
        };

        tokens.extend(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("10px",   Ok(Dimension::Pixels(10.0))),
            ("2.5px",  Ok(Dimension::Pixels(2.5))),
            ("3.0rem", Ok(Dimension::Rems(3.0))),
            ("0.5rem", Ok(Dimension::Rems(0.5))),
            ("1.2em",  Err(Error::InvalidUnit(&["px", "rem"]))),
            ("abcpx",  Err(Error::InvalidNumber("invalid float literal".to_owned()))),
            ("",       Err(Error::InvalidUnit(&["px", "rem"]))),
            ("5",      Err(Error::InvalidUnit(&["px", "rem"]))),
            ("-2px",   Err(Error::NumberMustBePositive)),
        ];

        for (input, expected) in test_cases {
            let result = Dimension::from_str(input);
            assert_eq!(result, expected);
        }
    }
}
