//! Represents a font weight. The $type property MUST be set to the string fontWeight. The value
//! must either be a number value in the range [1, 1000] or one of the pre-defined string values
//! defined in the table below.
//!
//! Lower numbers represent lighter weights, and higher numbers represent thicker weights, as per
//! the OpenType wght tag specification. The pre-defined string values are aliases for specific
//! numeric values. For example 100, "thin" and "hairline" are all the exact same value.
//!
//! numeric value  string       value aliases
//! 100            thin         hairline
//! 200            extra-light  ultra-light
//! 300            light
//! 400            normal       regular, book
//! 500            medium
//! 600            semi-bold    demi-bold
//! 700            bold
//! 800            extra-bold   ultra-bold
//! 900            black        heavy
//! 950            extra-black  ultra-black
//!
//! Number values outside of the [1, 1000] range and any other string values, including ones that
//! differ only in case, are invalid and MUST be rejected by tools.

use std::str::FromStr;

use tinyjson::JsonValue;

use crate::error::Error;

/// See module level documentation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy::reflect::Reflect))]
pub enum FontWeight {
    Numeric(u16),
    Thin,
    Hairline,
    ExtraLight,
    UltraLight,
    Light,
    Normal,
    Regular,
    Book,
    Medium,
    SemiBold,
    DemiBold,
    Bold,
    ExtraBold,
    UltraBold,
    Black,
    Heavy,
    ExtraBlack,
    UltraBlack,
}

impl TryFrom<&JsonValue> for FontWeight {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            #[allow(clippy::cast_sign_loss, clippy::float_cmp)]
            &JsonValue::Number(v) if v == (v as u16) as f64 => Self::try_from(v as u16),
            JsonValue::String(v) => Self::from_str(v),
            _ => Err(Error::UnexpectedType),
        }
    }
}

impl TryFrom<u16> for FontWeight {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::Thin),
            200 => Ok(Self::ExtraLight),
            300 => Ok(Self::Light),
            400 => Ok(Self::Normal),
            500 => Ok(Self::Medium),
            600 => Ok(Self::SemiBold),
            700 => Ok(Self::Bold),
            800 => Ok(Self::ExtraBold),
            900 => Ok(Self::Black),
            950 => Ok(Self::ExtraBlack),
            1..=1000 => Ok(Self::Numeric(value)),
            _ => Err(Error::NumberWithin(1, 1000)),
        }
    }
}

impl FromStr for FontWeight {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "thin" | "hairline" => Ok(Self::Thin),
            "extra-light" | "ultra-light" => Ok(Self::ExtraLight),
            "light" => Ok(Self::Light),
            "normal" | "regular" | "book" => Ok(Self::Normal),
            "medium" => Ok(Self::Medium),
            "semi-bold" | "demi-bold" => Ok(Self::SemiBold),
            "bold" => Ok(Self::Bold),
            "extra-bold" | "ultra-bold" => Ok(Self::ExtraBold),
            "black" | "heavy" => Ok(Self::Black),
            "extra-black" | "ultra-black" => Ok(Self::ExtraBlack),
            _ => Err(Error::InvalidFormat("unknown weight value")),
        }
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for FontWeight {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;

        tokens.extend(quote!(dtoken::types::font_weight::));

        let new = match self {
            Self::Numeric(v) => quote! { FontWeight::Numeric(#v) },
            Self::Thin => quote! { FontWeight::Thin },
            Self::Hairline => quote! { FontWeight::Hairline },
            Self::ExtraLight => quote! { FontWeight::ExtraLight },
            Self::UltraLight => quote! { FontWeight::UltraLight },
            Self::Light => quote! { FontWeight::Light },
            Self::Normal => quote! { FontWeight::Normal },
            Self::Regular => quote! { FontWeight::Regular },
            Self::Book => quote! { FontWeight::Book },
            Self::Medium => quote! { FontWeight::Medium },
            Self::SemiBold => quote! { FontWeight::SemiBold },
            Self::DemiBold => quote! { FontWeight::DemiBold },
            Self::Bold => quote! { FontWeight::Bold },
            Self::ExtraBold => quote! { FontWeight::ExtraBold },
            Self::UltraBold => quote! { FontWeight::UltraBold },
            Self::Black => quote! { FontWeight::Black },
            Self::Heavy => quote! { FontWeight::Heavy },
            Self::ExtraBlack => quote! { FontWeight::ExtraBlack },
            Self::UltraBlack => quote! { FontWeight::UltraBlack },
        };

        tokens.extend(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let test_cases = vec![
            ("thin", Ok(FontWeight::Thin)),
            ("extra-light", Ok(FontWeight::ExtraLight)),
            ("normal", Ok(FontWeight::Normal)),
            ("medium", Ok(FontWeight::Medium)),
            ("semi-bold", Ok(FontWeight::SemiBold)),
            ("bold", Ok(FontWeight::Bold)),
            ("extra-bold", Ok(FontWeight::ExtraBold)),
            ("black", Ok(FontWeight::Black)),
            ("extra-black", Ok(FontWeight::ExtraBlack)),
            ("invalid", Err(Error::InvalidFormat("unknown weight value"))),
        ];

        for (input, expected) in test_cases {
            let result = FontWeight::from_str(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_from_numeric() {
        let test_cases = vec![
            (100, Ok(FontWeight::Thin)),
            (300, Ok(FontWeight::Light)),
            (400, Ok(FontWeight::Normal)),
            (600, Ok(FontWeight::SemiBold)),
            (700, Ok(FontWeight::Bold)),
            (800, Ok(FontWeight::ExtraBold)),
            (900, Ok(FontWeight::Black)),
            (950, Ok(FontWeight::ExtraBlack)),
            (123, Ok(FontWeight::Numeric(123))),
            (0, Err(Error::NumberWithin(1, 1000))),
            (1001, Err(Error::NumberWithin(1, 1000))),
        ];

        for (input, expected) in test_cases {
            let result = FontWeight::try_from(input);
            assert_eq!(result, expected);
        }
    }
}
