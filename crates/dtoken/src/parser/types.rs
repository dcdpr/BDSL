use std::collections::HashMap;

use tinyjson::JsonValue;

use super::{group::Group, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct DesignTokens {
    pub items: HashMap<String, TokenOrGroup>,
}

impl DesignTokens {
    pub fn from_map(map: &HashMap<String, JsonValue>) -> Option<Self> {
        let items = map
            .iter()
            .filter_map(|(k, v)| Some((k.to_owned(), TokenOrGroup::from_map(v.get()?, None)?)))
            .collect();

        Some(Self { items })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenOrGroup {
    Token(Token),
    Group(Group),
}
impl TokenOrGroup {
    pub fn from_map(
        map: &HashMap<String, JsonValue>,
        default_type: Option<String>,
    ) -> Option<Self> {
        if map.contains_key("$value") {
            Token::from_map(map, default_type).map(TokenOrGroup::Token)
        } else {
            Group::from_value(map, default_type).map(TokenOrGroup::Group)
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            TokenOrGroup::Token(v) => v.description.as_deref(),
            TokenOrGroup::Group(v) => v.description.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::token::Value,
        types::{color::Color, dimension::Dimension},
    };

    use super::*;
    use tinyjson::JsonValue::{Object, String};

    #[test]
    fn test_design_tokens_from_map() {
        let test_cases = vec![
            (
                HashMap::from([
                    (
                        "color".to_string(),
                        Object(HashMap::from([
                            ("$type".to_string(), String("color".to_owned())),
                            ("$value".to_string(), String("#FF5733".to_owned())),
                            ("$description".to_string(), String("Red color".to_owned())),
                        ])),
                    ),
                    (
                        "dimension".to_string(),
                        Object(HashMap::from([
                            ("$type".to_string(), String("dimension".to_owned())),
                            ("$value".to_string(), String("16px".to_owned())),
                        ])),
                    ),
                ]),
                Some(DesignTokens {
                    items: vec![
                        (
                            "color".to_owned(),
                            TokenOrGroup::Token(Token {
                                value: Value::Color(Color {
                                    r: 255,
                                    g: 87,
                                    b: 51,
                                    a: 255,
                                }),
                                description: Some("Red color".to_owned()),
                            }),
                        ),
                        (
                            "dimension".to_owned(),
                            TokenOrGroup::Token(Token {
                                value: Value::Dimension(Dimension::Pixels(16.0)),
                                description: None,
                            }),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                HashMap::from([(
                    "group".to_string(),
                    Object(HashMap::from([
                        (
                            "subgroup".to_string(),
                            Object(HashMap::from([
                                ("$type".to_string(), String("color".to_owned())),
                                ("$value".to_string(), String("#00FF00".to_owned())),
                            ])),
                        ),
                        ("$type".to_string(), String("dimension".to_owned())),
                    ])),
                )]),
                Some(DesignTokens {
                    items: vec![(
                        "group".to_owned(),
                        TokenOrGroup::Group(Group {
                            items: vec![(
                                "subgroup".to_owned(),
                                TokenOrGroup::Token(Token {
                                    value: Value::Color(Color {
                                        r: 0,
                                        g: 255,
                                        b: 0,
                                        a: 255,
                                    }),
                                    description: None,
                                }),
                            )]
                            .into_iter()
                            .collect(),
                            description: None,
                            default_type: Some("dimension".to_owned()),
                            extensions: HashMap::new(),
                        }),
                    )]
                    .into_iter()
                    .collect(),
                }),
            ),
        ];

        for (input, expected) in test_cases {
            let result = DesignTokens::from_map(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_token_or_group_from_map() {
        // Test cases for Token variant
        let token_test_cases = vec![
            (
                HashMap::from([
                    ("$type".to_string(), String("color".to_owned())),
                    ("$value".to_string(), String("#FF5733".to_owned())),
                ]),
                None,
                Some(TokenOrGroup::Token(Token {
                    value: Value::Color(Color {
                        r: 255,
                        g: 87,
                        b: 51,
                        a: 255,
                    }),
                    description: None,
                })),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("dimension".to_owned())),
                    ("$value".to_string(), String("16px".to_owned())),
                ]),
                None,
                Some(TokenOrGroup::Token(Token {
                    value: Value::Dimension(Dimension::Pixels(16.0)),
                    description: None,
                })),
            ),
        ];

        for (input, default_type, expected) in token_test_cases {
            let result = TokenOrGroup::from_map(&input, default_type);
            similar_asserts::assert_eq!(result, expected);
        }

        // Test cases for Group variant
        let group_test_cases = vec![(
            HashMap::from([
                ("group".to_string(), Object(HashMap::new())),
                ("$type".to_string(), String("dimension".to_owned())),
            ]),
            None,
            Some(TokenOrGroup::Group(Group {
                items: HashMap::from([(
                    "group".to_string(),
                    TokenOrGroup::Group(Group {
                        items: HashMap::new(),
                        description: None,
                        default_type: Some("dimension".to_owned()),
                        extensions: HashMap::new(),
                    }),
                )]),
                description: None,
                default_type: Some("dimension".to_owned()),
                extensions: HashMap::new(),
            })),
        )];

        for (input, default_type, expected) in group_test_cases {
            let result = TokenOrGroup::from_map(&input, default_type);
            similar_asserts::assert_eq!(result, expected);
        }
    }
}
