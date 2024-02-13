use std::collections::HashMap;

use tinyjson::JsonValue;

use super::types::TokenOrGroup;

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub items: HashMap<String, TokenOrGroup>,
    pub description: Option<String>,
    pub default_type: Option<String>,
    pub extensions: HashMap<String, JsonValue>,
}

impl Group {
    pub fn from_value(
        map: &HashMap<String, JsonValue>,
        mut default_type: Option<String>,
    ) -> Option<Self> {
        let mut items = HashMap::new();
        let mut description = None;
        let mut extensions = HashMap::new();

        if let Some(kind) = map.get("$type").and_then(|v| v.get::<String>()) {
            if !Self::is_valid_type(&kind) {
                return None; // Invalid type value
            }

            default_type = Some(kind.clone());
        }

        for (key, val) in map {
            match (key.as_str(), val) {
                ("$description", JsonValue::String(desc)) => {
                    description = Some(desc.clone());
                }
                ("$extensions", JsonValue::Object(exts)) => {
                    extensions.extend(exts.clone());
                }
                ("$type", JsonValue::String(_)) => { /* already covered */ }
                (_, JsonValue::Object(map)) => {
                    let item = TokenOrGroup::from_map(map, default_type.clone())?;
                    items.insert(key.clone(), item);
                }
                _ => return None,
            }
        }

        Some(Group {
            items,
            description,
            default_type,
            extensions,
        })
    }

    fn is_valid_type(type_str: &str) -> bool {
        let valid_types = vec![
            "border",
            "color",
            "cubicBezier",
            "dimension",
            "duration",
            "fontFamily",
            "fontWeight",
            "gradient",
            "number",
            "shadow",
            "strokeStyle",
            "transition",
            "typography",
        ];
        valid_types.contains(&type_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::token::{Token, Value},
        types::{color::Color, dimension::Dimension},
    };

    use super::*;
    use tinyjson::JsonValue::{Number, Object, String};

    #[test]
    fn test_group_from_value() {
        let test_cases = vec![(
            HashMap::from([
                (
                    "group1".to_string(),
                    Object(HashMap::from([
                        ("$type".to_string(), String("color".to_owned())),
                        ("$value".to_string(), String("#FF5733".to_owned())),
                    ])),
                ),
                (
                    "group2".to_string(),
                    Object(HashMap::from([
                        ("$type".to_string(), String("dimension".to_owned())),
                        ("$value".to_string(), String("16px".to_owned())),
                    ])),
                ),
                (
                    "nested_group".to_string(),
                    Object(HashMap::from([
                        (
                            "nested_group1".to_string(),
                            Object(HashMap::from([
                                ("$type".to_string(), String("color".to_owned())),
                                ("$value".to_string(), String("#00FF00".to_owned())),
                            ])),
                        ),
                        (
                            "nested_group2".to_string(),
                            Object(HashMap::from([(
                                "$value".to_string(),
                                String("32px".to_owned()),
                            )])),
                        ),
                        ("$type".to_string(), String("dimension".to_owned())),
                    ])),
                ),
                (
                    "$description".to_string(),
                    String("A group of tokens".to_owned()),
                ),
                (
                    "$extensions".to_string(),
                    Object(HashMap::from([("key1".to_string(), Number(42.0))])),
                ),
            ]),
            Some(Group {
                items: vec![
                    (
                        "group1".to_owned(),
                        TokenOrGroup::Token(Token {
                            value: Value::Color(Color {
                                r: 255,
                                g: 87,
                                b: 51,
                                a: 255,
                            }),
                            description: None,
                        }),
                    ),
                    (
                        "group2".to_owned(),
                        TokenOrGroup::Token(Token {
                            value: Value::Dimension(Dimension::Pixels(16.0)),
                            description: None,
                        }),
                    ),
                    (
                        "nested_group".to_owned(),
                        TokenOrGroup::Group(Group {
                            items: vec![
                                (
                                    "nested_group1".to_owned(),
                                    TokenOrGroup::Token(Token {
                                        value: Value::Color(Color {
                                            r: 0,
                                            g: 255,
                                            b: 0,
                                            a: 255,
                                        }),
                                        description: None,
                                    }),
                                ),
                                (
                                    "nested_group2".to_owned(),
                                    TokenOrGroup::Token(Token {
                                        value: Value::Dimension(Dimension::Pixels(32.0)),
                                        description: None,
                                    }),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                            description: None,
                            default_type: Some("dimension".to_owned()),
                            extensions: HashMap::new(),
                        }),
                    ),
                ]
                .into_iter()
                .collect(),
                description: Some("A group of tokens".to_owned()),
                default_type: None,
                extensions: HashMap::from([("key1".to_owned(), Number(42.0))]),
            }),
        )];

        for (input, expected) in test_cases {
            let result = Group::from_value(&input, None);
            similar_asserts::assert_eq!(result, expected);
        }
    }
}
