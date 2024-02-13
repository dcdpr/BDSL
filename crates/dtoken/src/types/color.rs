//! Represents a 24bit RGB or 24+8bit RGBA color in the sRGB color space. The $type property MUST
//! be set to the string color. The value MUST be a string containing a hex triplet/quartet
//! including the preceding # character. To support other color spaces, such as HSL, translation
//! tools SHOULD convert color tokens to the equivalent value as needed.
//!
//! For example, initially the color tokens MAY be defined as such:
//!
//! EXAMPLE 16
//! ```json,ignore
//! {
//!   "Majestic magenta": {
//!     "$value": "#ff00ff",
//!     "$type": "color"
//!   },
//!   "Translucent shadow": {
//!     "$value": "#00000080",
//!     "$type": "color"
//!   }
//! }
//! ```
//!
//! See: <https://tr.designtokens.org/format/#color>.

/// See module documentation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color { r, g, b, a: 255 })
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Color { r, g, b, a })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("#FF0000",    Some(Color { r: 255, g: 0, b: 0, a: 255 })),
            ("#00FF00",    Some(Color { r: 0, g: 255, b: 0, a: 255 })),
            ("#0000FF",    Some(Color { r: 0, g: 0, b: 255, a: 255 })),
            ("#123456",    Some(Color { r: 18, g: 52, b: 86, a: 255 })),
            ("#AABBCCDD",  Some(Color { r: 170, g: 187, b: 204, a: 221 })),
            ("#GHIJKL",    None), // Invalid hex characters
            ("#12345",     None), // Invalid hex length
            ("#123456789", None), // Invalid hex length
            ("",           None), // Empty input
        ];

        for (input, expected) in test_cases {
            let result = Color::from_hex(input);
            assert_eq!(result, expected);
        }
    }
}
