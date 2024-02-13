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

/// See module-level documentation.
#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    pub milliseconds: f64,
}

impl Duration {
    pub fn from_str(s: &str) -> Option<Self> {
        if s.starts_with('-') {
            return None; // Reject negative values
        }

        if s.ends_with("ms") {
            s[..s.len() - 2]
                .parse::<f64>()
                .ok()
                .map(|milliseconds| Duration { milliseconds })
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
        let test_cases = vec![
            ("10ms", Some(Duration { milliseconds: 10.0 })),
            ("2.5ms", Some(Duration { milliseconds: 2.5 })),
            ("0.1ms", Some(Duration { milliseconds: 0.1 })),
            ("ms", None),        // Missing numeric value
            ("abcms", None),     // Invalid numeric value
            ("200s", None),      // Invalid unit
            ("", None),          // Empty input
            ("1000", None),      // Missing unit
            ("-5ms", None),      // Negative value not supported
            ("1.23.45ms", None), // Multiple periods not supported
        ];

        for (input, expected) in test_cases {
            let result = Duration::from_str(input);
            assert_eq!(result, expected);
        }
    }
}
