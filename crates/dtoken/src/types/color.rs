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

use tinyjson::JsonValue;

use crate::error::Error;

/// See module documentation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[must_use]
    pub fn to_rgba(self) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            self.a as f32 / 255.,
        ]
    }

    pub fn from_hex(hex: &str) -> Result<Self, Error> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(Error::from)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(Error::from)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(Error::from)?;
                Ok(Color { r, g, b, a: 255 })
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(Error::from)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(Error::from)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(Error::from)?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(Error::from)?;
                Ok(Color { r, g, b, a })
            }
            _ => Err(Error::InvalidFormat("must be 6 or 8 characters long")),
        }
    }
}

impl TryFrom<&JsonValue> for Color {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<String>()
            .ok_or(Error::ExpectedString)
            .and_then(|v| Self::from_hex(v))
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Color {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { r, g, b, a } = self;

        tokens.extend(quote::quote! { dtoken::types::color::Color { r: #r, g: #g, b: #b, a: #a } });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("#FF0000",    Ok(Color { r: 255, g: 0, b: 0, a: 255 })),
            ("#00FF00",    Ok(Color { r: 0, g: 255, b: 0, a: 255 })),
            ("#0000FF",    Ok(Color { r: 0, g: 0, b: 255, a: 255 })),
            ("#123456",    Ok(Color { r: 18, g: 52, b: 86, a: 255 })),
            ("#AABBCCDD",  Ok(Color { r: 170, g: 187, b: 204, a: 221 })),
            ("#GHIJKL",    Err(Error::InvalidNumber("invalid digit found in string".to_owned()))),
            ("#12345",     Err(Error::InvalidFormat("must be 6 or 8 characters long"))),
            ("#123456789", Err(Error::InvalidFormat("must be 6 or 8 characters long"))),
            ("",           Err(Error::InvalidFormat("must be 6 or 8 characters long"))),
        ];

        for (input, expected) in test_cases {
            let result = Color::from_hex(input);
            assert_eq!(result, expected);
        }
    }
}
