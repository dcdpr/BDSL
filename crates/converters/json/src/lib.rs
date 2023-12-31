//! # Bread'n'Butter JSON Converter
//!
//! **A buttery smooth JSON conversion experience.**
//!
//! The `bnb_converter_json` crate provides utility functions for serializing and deserializing
//! `Breadboard` structures to and from JSON format.
//!
//! ## Overview
//!
//! The crate offers two primary functionalities:
//!
//! - [`serialize`]: Converts a `Breadboard` instance into a JSON representation.
//! - [`deserialize`]: Constructs a `Breadboard` instance from JSON data.
//!
//! ## Usage
//!
//! The crate is particularly useful in scenarios where breadboard configurations need to be saved
//! as JSON files or sent over a network.
//!
//! ## Examples
//!
//! See the function-level documentation for examples.

use std::io::{Read, Write};

use bnb_ast::Breadboard;
use serde_json::Result;

/// Serializes a `Breadboard` structure into JSON format.
///
/// # Examples
///
/// ```
/// use bnb_ast::Breadboard;
/// use bnb_converter_json::serialize;
///
/// let breadboard = Breadboard { places: vec![], components: vec![] };
/// let mut buffer = vec![];
/// serialize(&mut buffer, &breadboard).unwrap();
/// ```
///
pub fn serialize(writer: impl Write, breadboard: &Breadboard) -> Result<()> {
    serde_json::to_writer(writer, breadboard)
}

/// Deserializes JSON data into a `Breadboard` structure.
///
/// # Examples
///
/// ```
/// use bnb_converter_json::deserialize;
///
/// let json = r#"{"places": [], "components": []}"#;
/// let breadboard = deserialize(json.as_bytes()).unwrap();
/// ```
///
pub fn deserialize(reader: impl Read) -> Result<Breadboard> {
    serde_json::from_reader(reader)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bnb_ast::{Affordance, Area, Component, Connection, Place, Sketch};

    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let breadboard = Breadboard {
            places: vec![
                Place {
                    name: "Registration".to_string(),
                    affordances: vec![
                        Affordance {
                            name: "Username".to_string(),
                            connections: vec![],
                        },
                        Affordance {
                            name: "Password".to_string(),
                            connections: vec![],
                        },
                        Affordance {
                            name: "Sign Up".to_string(),
                            connections: vec![
                                Connection {
                                    target_place: "Home".to_string(),
                                    description: Some("success".to_string()),
                                },
                                Connection {
                                    target_place: "Support".to_string(),
                                    description: Some("failure".to_string()),
                                },
                            ],
                        },
                    ],
                    component_references: vec!["Header".to_string()],
                    sketch: Some(Sketch {
                        path: std::path::PathBuf::from("sketches/registration.png"),
                        connections: vec![(
                            Area {
                                top_left: (50, 20),
                                width: 110,
                                height: 40,
                            },
                            vec![
                                Connection {
                                    target_place: "Home".to_string(),
                                    description: Some("success".to_string()),
                                },
                                Connection {
                                    target_place: "Support".to_string(),
                                    description: Some("failure".to_string()),
                                },
                            ],
                        )],
                    }),
                },
                Place {
                    name: "Support".to_string(),
                    affordances: vec![
                        Affordance {
                            name: "Error Message".to_string(),
                            connections: vec![],
                        },
                        Affordance {
                            name: "Try Again".to_string(),
                            connections: vec![Connection {
                                target_place: "Registration".to_string(),
                                description: None,
                            }],
                        },
                    ],
                    component_references: vec!["Header".to_string()],
                    sketch: Some(Sketch {
                        path: PathBuf::from("sketches/support.png"),
                        connections: vec![(
                            Area {
                                top_left: (50, 20),
                                width: 110,
                                height: 40,
                            },
                            vec![Connection {
                                target_place: "Registration".to_string(),
                                description: None,
                            }],
                        )],
                    }),
                },
                Place {
                    name: "Home".to_string(),
                    affordances: vec![Affordance {
                        name: "Dashboard".to_string(),
                        connections: vec![],
                    }],
                    component_references: vec!["Header".to_string()],
                    sketch: Some(Sketch {
                        path: PathBuf::from("sketches/home.png"),
                        connections: vec![],
                    }),
                },
            ],
            components: vec![Component {
                name: "Header".to_string(),
                affordances: vec![
                    Affordance {
                        name: "Logo".to_string(),
                        connections: vec![],
                    },
                    Affordance {
                        name: "Contact".to_string(),
                        connections: vec![],
                    },
                ],
            }],
        };

        // Serialize the Breadboard
        let mut serialized_data = Vec::new();
        serialize(&mut serialized_data, &breadboard).expect("Serialization failed");

        // Deserialize the Breadboard
        let deserialized_breadboard: Breadboard =
            deserialize(&mut serialized_data.as_slice()).expect("Deserialization failed");

        insta::assert_json_snapshot!(deserialized_breadboard);
    }
}
