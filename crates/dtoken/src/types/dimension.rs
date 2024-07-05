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

use std::ops::Deref;

/// See module docs.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dimension {
    Pixels(f64),
    Rems(f64),
}

impl Dimension {
    pub fn as_f32(&self) -> f32 {
        match self {
            Self::Pixels(v) => *v as f32,
            Self::Rems(v) => *v as f32,
        }
    }
}

impl Deref for Dimension {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Pixels(v) => v,
            Self::Rems(v) => v,
        }
    }
}

impl Dimension {
    pub fn from_str(s: &str) -> Option<Self> {
        if s.starts_with('-') {
            return None; // Reject negative values
        }

        if let Some(s) = s.strip_suffix("px") {
            s.parse::<f64>().ok().map(Dimension::Pixels)
        } else if let Some(s) = s.strip_suffix("rem") {
            s.parse::<f64>().ok().map(Dimension::Rems)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("10px",   Some(Dimension::Pixels(10.0))),
            ("2.5px",  Some(Dimension::Pixels(2.5))),
            ("3.0rem", Some(Dimension::Rems(3.0))),
            ("0.5rem", Some(Dimension::Rems(0.5))),
            ("1.2em",  None), // Invalid unit
            ("abcpx",  None), // Invalid number
            ("",       None), // Empty input
            ("5",      None), // Missing unit
            ("-2px",   None), // Negative value not supported
        ];

        for (input, expected) in test_cases {
            let result = Dimension::from_str(input);
            assert_eq!(result, expected);
        }
    }
}
