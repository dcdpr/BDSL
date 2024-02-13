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

/// See module level documentation.
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl FontWeight {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "thin" | "hairline" => Some(Self::Thin),
            "extra-light" | "ultra-light" => Some(Self::ExtraLight),
            "light" => Some(Self::Light),
            "normal" | "regular" | "book" => Some(Self::Normal),
            "medium" => Some(Self::Medium),
            "semi-bold" | "demi-bold" => Some(Self::SemiBold),
            "bold" => Some(Self::Bold),
            "extra-bold" | "ultra-bold" => Some(Self::ExtraBold),
            "black" | "heavy" => Some(Self::Black),
            "extra-black" | "ultra-black" => Some(Self::ExtraBlack),
            _ => None,
        }
    }

    pub fn from_numeric(n: u16) -> Option<Self> {
        match n {
            100 => Some(Self::Thin),
            200 => Some(Self::ExtraLight),
            300 => Some(Self::Light),
            400 => Some(Self::Normal),
            500 => Some(Self::Medium),
            600 => Some(Self::SemiBold),
            700 => Some(Self::Bold),
            800 => Some(Self::ExtraBold),
            900 => Some(Self::Black),
            950 => Some(Self::ExtraBlack),
            1..=1000 => Some(Self::Numeric(n)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let test_cases = vec![
            ("thin", Some(FontWeight::Thin)),
            ("extra-light", Some(FontWeight::ExtraLight)),
            ("normal", Some(FontWeight::Normal)),
            ("medium", Some(FontWeight::Medium)),
            ("semi-bold", Some(FontWeight::SemiBold)),
            ("bold", Some(FontWeight::Bold)),
            ("extra-bold", Some(FontWeight::ExtraBold)),
            ("black", Some(FontWeight::Black)),
            ("extra-black", Some(FontWeight::ExtraBlack)),
            ("invalid", None), // Invalid input
        ];

        for (input, expected) in test_cases {
            let result = FontWeight::from_str(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_from_numeric() {
        let test_cases = vec![
            (100, Some(FontWeight::Thin)),
            (300, Some(FontWeight::Light)),
            (400, Some(FontWeight::Normal)),
            (600, Some(FontWeight::SemiBold)),
            (700, Some(FontWeight::Bold)),
            (800, Some(FontWeight::ExtraBold)),
            (900, Some(FontWeight::Black)),
            (950, Some(FontWeight::ExtraBlack)),
            (123, Some(FontWeight::Numeric(123))),
            (0, None),     // Invalid numeric value
            (1001, None),  // Invalid numeric value
            (10000, None), // Invalid numeric value
            (12345, None), // Invalid numeric value
        ];

        for (input, expected) in test_cases {
            let result = FontWeight::from_numeric(input);
            assert_eq!(result, expected);
        }
    }
}
