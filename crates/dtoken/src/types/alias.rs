//! Instead of having explicit values, tokens can reference the value of another token. To put it
//! another way, a token can be an alias for another token. This spec considers the terms "alias"
//! and "reference" to be synonyms and uses them interchangeably.
//!
//! Aliases are useful for:
//!
//! - Expressing design choices
//! - Eliminating repetition of values in token files (DRYing up the code)
//!
//! For a design token to reference another, its value MUST be a string containing the
//! period-separated (.) path to the token it's referencing enclosed in curly brackets.
//!
//! For example:
//! Example 15
//!
//! ```json,ignore
//! {
//!   "group name": {
//!     "token name": {
//!       "$value": 1234,
//!       "$type": "number"
//!     }
//!   },
//!   "alias name": {
//!     "$value": "{group name.token name}"
//!   }
//! }
//! ```
//!
//! When a tool needs the actual value of a token it MUST resolve the reference - i.e. lookup the
//! token being referenced and fetch its value. In the above example, the "alias name" token's
//! value would resolve to 1234 because it references the token whose path is {group name.token
//! name} which has the value 1234.
//!
//! Tools SHOULD preserve references and therefore only resolve them whenever the actual value
//! needs to be retrieved. For instance, in a design tool, changes to the value of a token being
//! referenced by aliases SHOULD be reflected wherever those aliases are being used.
//!
//! Aliases MAY reference other aliases. In this case, tools MUST follow each reference until they
//! find a token with an explicit value. Circular references are not allowed. If a design token
//! file contains circular references, then the value of all tokens in that chain is unknown and an
//! appropriate error or warning message SHOULD be displayed to the user.
#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    pub path_segments: Vec<String>,
}

impl Alias {
    pub fn from_str(s: &str) -> Option<Self> {
        if s.starts_with('{') && s.ends_with('}') {
            let path = &s[1..s.len() - 1]; // Remove the curly braces
            let path_segments = path.split('.').map(String::from).collect();
            Some(Self { path_segments })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_from_str() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("{foo.bar}", Some(Alias { path_segments: vec!["foo".to_string(), "bar".to_string()] })),
            ("{abc.xyz}", Some(Alias { path_segments: vec!["abc".to_string(), "xyz".to_string()] })),
            ("{token}", Some(Alias { path_segments: vec!["token".to_string()] })),
            ("not_an_alias", None),
            ("{}valid{}", Some(Alias { path_segments: vec!["}valid{".to_string()] })),
            ("{foo.bar", None),
            ("foo.bar}", None),
        ];

        for (input, expected) in test_cases {
            let result = Alias::from_str(input);
            assert_eq!(result, expected);
        }
    }
}
