//! Translations files parsing
//!
//! Files are parsed as [TranslationData] from a provided [JsonValue].
//! Parsed keys are represented as [TranslationKey].

pub mod group;
pub mod token;
pub mod types;

// use std::collections::{HashMap, HashSet};
//
// use lazy_static::lazy_static;
// use regex::Regex;
// use tinyjson::JsonValue;
//
// use crate::{builder::TokensId, error::ParseError};
//
// /// Data structure containing all translation keys
// ///
// /// This struct should be initialized with the fallback language,
// /// then keys will be populated with other languages using the [`parse_file`] method.
// ///
// /// [`parse_file`]: Self::parse_file
// #[derive(Debug, Clone, PartialEq, Eq, Default)]
// pub(crate) struct TokensData {
//     /// Parsed translation keys
//     pub(crate) keys: HashMap<String, TranslationKey>,
// }
//
// impl TokensData {
//     /// Parse a language file and insert its content into the current [`TranslationData`]
//     pub(crate) fn parse_file(
//         &mut self,
//         group: TokensId,
//         file: JsonValue,
//     ) -> Result<(), ParseError> {
//         let parsed = ParsedFile::parse(file)?;
//
//         for (key, parsed) in parsed.keys {
//             match self.keys.get_mut(&key) {
//                 Some(translation_key) => {
//                     let data = ParsedKeyData {
//                         language: group.clone(),
//                         key: &key,
//                         parsed,
//                     };
//                     translation_key.insert_parsed(data)?
//                 }
//                 None => println!(
//                     "cargo:warning=Key `{}` exists in {} but not in fallback language",
//                     key, group
//                 ),
//             };
//         }
//
//         Ok(())
//     }
// }
//
// /// A parsed group key
// ///
// /// This enum can be constructed by parsing a translation file with [TranslationData].
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum TranslationKey {
//     Simple(SimpleKey),
//     Formatted(FormattedKey),
// }
//
// impl TranslationKey {
//     // /// Initialize a new [TranslationKey] from a [`ParsedKey`]
//     // pub(crate) fn from_parsed(parsed: ParsedKey) -> Self {
//     //     match parsed {
//     //         ParsedKey::Simple(value) => TranslationKey::Simple(SimpleKey {
//     //             fallback: value,
//     //             others: HashMap::new(),
//     //         }),
//     //         ParsedKey::Formatted { value, parameters } => TranslationKey::Formatted(FormattedKey {
//     //             fallback: value,
//     //             others: HashMap::new(),
//     //             parameters,
//     //         }),
//     //     }
//     // }
//
//     /// Inserts a new raw [`ParsedKey`] in this [`TranslationKey`]
//     fn insert_parsed(&mut self, data: ParsedKeyData) -> Result<(), ParseError> {
//         match self {
//             TranslationKey::Simple(inner) => inner.insert_parsed(data),
//             TranslationKey::Formatted(inner) => inner.insert_parsed(data),
//         }
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// /// Simple string key, without any formatting or plurals
// pub struct SimpleKey {
//     /// The key value for the fallback language
//     pub(crate) fallback: String,
//     /// Key values for other languages
//     pub(crate) others: HashMap<TokensId, String>,
// }
//
// impl SimpleKey {
//     /// Inserts a new raw [`ParsedKey`] in this [`SimpleKey`]
//     fn insert_parsed(&mut self, data: ParsedKeyData) -> Result<(), ParseError> {
//         match data.parsed {
//             ParsedKey::Simple(value) => self.others.insert(data.language, value),
//             _ => {
//                 return Err(ParseError::InvalidType {
//                     key: data.key.into(),
//                     expected: "string",
//                 })
//             }
//         };
//
//         Ok(())
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// /// Simple string key with formatting
// pub struct FormattedKey {
//     /// The key value for the fallback language
//     pub(crate) fallback: String,
//     /// Key values for other languages
//     pub(crate) others: HashMap<TokensId, String>,
//     /// List of parameters in the value
//     pub(crate) parameters: HashSet<String>,
// }
//
// impl FormattedKey {
//     /// Inserts a new [`ParsedKey`] in this [`SimpleKey`]
//     fn insert_parsed(&mut self, data: ParsedKeyData) -> Result<(), ParseError> {
//         let (value, parameters) = match data.parsed {
//             ParsedKey::Formatted { value, parameters } => (value, parameters),
//             _ => {
//                 return Err(ParseError::InvalidType {
//                     key: data.key.into(),
//                     expected: "formatted string",
//                 })
//             }
//         };
//
//         if parameters == self.parameters {
//             self.others.insert(data.language, value);
//             Ok(())
//         } else {
//             let missing: Vec<_> = self.parameters.difference(&parameters).cloned().collect();
//             let unknown: Vec<_> = parameters.difference(&self.parameters).cloned().collect();
//
//             Err(ParseError::InvalidParameters {
//                 key: data.key.into(),
//                 missing,
//                 unknown,
//             })
//         }
//     }
// }
//
// /// Raw representation of a parsed file
// #[derive(Debug, Clone, PartialEq, Eq)]
// struct ParsedFile {
//     keys: HashMap<String, ParsedKey>,
// }
//
// impl ParsedFile {
//     /// Parse a JSON [`JsonValue`] as a translations file
//     fn parse(file: JsonValue) -> Result<Self, ParseError> {
//         let input = match file {
//             JsonValue::Object(map) => map,
//             _ => return Err(ParseError::InvalidRoot),
//         };
//
//         let mut keys = HashMap::with_capacity(input.len());
//         for (key, value) in input {
//             let parsed = ParsedKey::parse(&key, value)?;
//             keys.insert(key, parsed);
//         }
//
//         Ok(ParsedFile { keys })
//     }
// }
//
// /// Raw representation of a parsed key
// #[derive(Debug, Clone, PartialEq, Eq)]
// enum ParsedKey {
//     /// Simple string key
//     Simple(String),
//     /// String key with formatted values
//     ///
//     /// Example : `Hello {name}!`
//     Formatted {
//         /// The raw key value
//         value: String,
//         /// List of parameters in the value
//         parameters: HashSet<String>,
//     },
// }
//
// impl ParsedKey {
//     /// Parse a JSON [`Value`] as a key
//     fn parse(key: &str, value: JsonValue) -> Result<Self, ParseError> {
//         match value {
//             JsonValue::String(value) => Ok(Self::parse_string(value)),
//             _ => Err(ParseError::InvalidValue { key: key.into() }),
//         }
//     }
//
//     fn parse_string(value: String) -> Self {
//         lazy_static! {
//             static ref RE: Regex = Regex::new(r"\{([a-z_]+)\}").unwrap();
//         }
//
//         let matches: HashSet<_> = RE
//             .captures_iter(&value)
//             .map(|capture| capture[1].to_string())
//             .collect();
//
//         if matches.is_empty() {
//             Self::Simple(value)
//         } else {
//             Self::Formatted {
//                 value,
//                 parameters: matches,
//             }
//         }
//     }
// }
//
// /// Data associated with a parsed key.
// ///
// /// Used in [`TranslationKey::insert_parsed`].
// #[derive(Debug, Clone, PartialEq, Eq)]
// struct ParsedKeyData<'a> {
//     language: TokensId,
//     key: &'a str,
//     parsed: ParsedKey,
// }
