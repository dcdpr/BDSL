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

use crate::error::Error;

/// See module docs.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct FontFamily {
    pub primary: String,
    pub fallbacks: Vec<String>,
}
impl FontFamily {
    #[must_use]
    pub fn primary(s: &str) -> Self {
        Self {
            primary: s.to_owned(),
            fallbacks: vec![],
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.primary.as_str()
    }
}

impl TryFrom<&JsonValue> for FontFamily {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(v) => Ok(Self::primary(v)),
            JsonValue::Array(v) => Self::try_from(v.as_slice()),
            _ => Err(Error::UnexpectedType),
        }
    }
}

impl TryFrom<&[JsonValue]> for FontFamily {
    type Error = Error;

    fn try_from(value: &[JsonValue]) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|val| {
                val.get::<String>()
                    .ok_or(Error::ExpectedItemString)
                    .map(ToOwned::to_owned)
            })
            .collect::<Result<Vec<_>, Error>>()?
            .split_first()
            .map(|(primary, fallbacks)| FontFamily {
                primary: primary.to_owned(),
                fallbacks: fallbacks.to_vec(),
            })
            .ok_or(Error::ExpectedArray)
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for FontFamily {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let FontFamily { primary, fallbacks } = self;

        let new = quote::quote! { dtoken::types::font_family::FontFamily {
            primary: #primary.to_owned(),
            fallbacks: vec![#( #fallbacks.to_owned(),)*],
        } };

        tokens.extend(new);
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
            let result = FontFamily::primary(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_from_slice() {
        use JsonValue::{Number, String};

        let test_cases = vec![
            (
                vec![String("Arial".to_owned())],
                Ok(FontFamily {
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
                Ok(FontFamily {
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
                Ok(FontFamily {
                    primary: "Roboto".to_owned(),
                    fallbacks: vec!["'Noto Sans'".to_owned(), "sans-serif".to_owned()],
                }),
            ),
            (vec![Number(12.)], Err(Error::ExpectedItemString)),
        ];

        for (input, expected) in test_cases {
            let result = FontFamily::try_from(input.as_slice());
            assert_eq!(result, expected);
        }
    }
}
