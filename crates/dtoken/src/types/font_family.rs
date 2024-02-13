//! Represents a font name or an array of font names (ordered from most to least preferred). The
//! $type property MUST be set to the string fontFamily. The value MUST either be a string value
//! containing a single font name or an array of strings, each being a single font name.
//!
//! For example:
//!
//! EXAMPLE 19
//! ```json,ignore
//! {
//!   "Primary font": {
//!     "$value": "Comic Sans MS",
//!     "$type": "fontFamily"
//!   },
//!   "Body font": {
//!     "$value": ["Helvetica", "Arial", "sans-serif"],
//!     "$type": "fontFamily"
//!   }
//! }
//! ```
//!
//! See: <https://tr.designtokens.org/format/#font-family>.

use tinyjson::JsonValue;

/// See module docs.
#[derive(Debug, Clone, PartialEq)]
pub struct FontFamily {
    pub primary: String,
    pub fallbacks: Vec<String>,
}
impl FontFamily {
    pub fn from_str(s: &str) -> Self {
        Self {
            primary: s.to_owned(),
            fallbacks: vec![],
        }
    }
    pub fn from_slice(value: &[JsonValue]) -> Option<Self> {
        value
            .into_iter()
            .map(|val| val.get::<String>().map(ToOwned::to_owned))
            .collect::<Option<Vec<_>>>()?
            .split_first()
            .map(|(primary, fallbacks)| FontFamily {
                primary: primary.to_owned(),
                fallbacks: fallbacks.to_vec(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let test_cases = vec![
            (
                "Arial",
                FontFamily {
                    primary: "Arial".to_owned(),
                    fallbacks: vec![],
                },
            ),
            (
                "Helvetica, Arial, sans-serif",
                FontFamily {
                    primary: "Helvetica, Arial, sans-serif".to_owned(),
                    fallbacks: vec![],
                },
            ),
            (
                "Roboto, 'Noto Sans', sans-serif",
                FontFamily {
                    primary: "Roboto, 'Noto Sans', sans-serif".to_owned(),
                    fallbacks: vec![],
                },
            ),
        ];

        for (input, expected) in test_cases {
            let result = FontFamily::from_str(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_from_slice() {
        use JsonValue::{Number, String};

        let test_cases = vec![
            (
                vec![String("Arial".to_owned())],
                Some(FontFamily {
                    primary: "Arial".to_owned(),
                    fallbacks: vec![],
                }),
            ),
            (
                vec![
                    String("Helvetica".to_owned()),
                    String("Arial".to_owned()),
                    String("sans-serif".to_owned()),
                ],
                Some(FontFamily {
                    primary: "Helvetica".to_owned(),
                    fallbacks: vec!["Arial".to_owned(), "sans-serif".to_owned()],
                }),
            ),
            (
                vec![
                    String("Roboto".to_owned()),
                    String("'Noto Sans'".to_owned()),
                    String("sans-serif".to_owned()),
                ],
                Some(FontFamily {
                    primary: "Roboto".to_owned(),
                    fallbacks: vec!["'Noto Sans'".to_owned(), "sans-serif".to_owned()],
                }),
            ),
            (
                vec![Number(12.)],
                None, // Invalid JSON value
            ),
        ];

        for (input, expected) in test_cases {
            let result = FontFamily::from_slice(&input);
            assert_eq!(result, expected);
        }
    }
}
