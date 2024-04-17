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
/// serialize(&mut buffer, &breadboard);
/// ```
///
#[allow(clippy::missing_panics_doc)]
pub fn serialize(writer: impl Write, breadboard: &Breadboard) {
    serde_json::to_writer(writer, breadboard).expect("Breadboard serialization cannot fail");
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
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the structure expected by
/// `Breadboard`. It can also fail if the structure is correct but something is wrong with the
/// data, for example required struct fields are missing from the JSON map or some number is too
/// big to fit in the expected primitive type.
///
pub fn deserialize(reader: impl Read) -> Result<Breadboard> {
    serde_json::from_reader(reader)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bnb_ast::{
        Affordance, Area, Component, Connection, Coordinate, Pivot, Place, Position, Sketch,
    };

    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_serialize_deserialize() {
        let breadboard = Breadboard {
            places: vec![
                Place {
                    name: "Registration".to_owned(),
                    affordances: vec![
                        Affordance {
                            name: "Username".to_owned(),
                            connections: vec![],
                            description: vec![],
                            level: 0,
                        },
                        Affordance {
                            name: "Password".to_owned(),
                            connections: vec![],
                            description: vec![],
                            level: 0,
                        },
                        Affordance {
                            name: "Sign Up".to_owned(),
                            connections: vec![
                                Connection {
                                    target_place: "Home".to_owned(),
                                    description: Some("success".to_owned()),
                                },
                                Connection {
                                    target_place: "Support".to_owned(),
                                    description: Some("failure".to_owned()),
                                },
                            ],
                            description: vec![],
                            level: 0,
                        },
                    ],
                    component_references: vec!["Header".to_owned()],
                    position: Some(Position {
                        x: Coordinate::Absolute(-10),
                        y: Coordinate::Relative {
                            place: "Support".to_owned(),
                            offset: 20,
                            pivot: Pivot::Left,
                        },
                    }),
                    sketch: Some(Sketch {
                        path: std::path::PathBuf::from("sketches/registration.png"),
                        areas: vec![Area {
                            top_left: (50, 20),
                            width: 110,
                            height: 40,
                            affordance: "Sign Up".to_owned(),
                        }],
                    }),
                    description: vec![],
                },
                Place {
                    name: "Support".to_owned(),
                    affordances: vec![
                        Affordance {
                            name: "Error Message".to_owned(),
                            connections: vec![],
                            description: vec![],
                            level: 0,
                        },
                        Affordance {
                            name: "Try Again".to_owned(),
                            connections: vec![Connection {
                                target_place: "Registration".to_owned(),
                                description: None,
                            }],
                            description: vec![],
                            level: 0,
                        },
                    ],
                    component_references: vec!["Header".to_owned()],
                    position: None,
                    sketch: Some(Sketch {
                        path: PathBuf::from("sketches/support.png"),
                        areas: vec![Area {
                            top_left: (50, 20),
                            width: 110,
                            height: 40,
                            affordance: "Try Again".to_owned(),
                        }],
                    }),
                    description: vec![],
                },
                Place {
                    name: "Home".to_owned(),
                    affordances: vec![Affordance {
                        name: "Dashboard".to_owned(),
                        connections: vec![],
                        description: vec![],
                        level: 0,
                    }],
                    component_references: vec!["Header".to_owned()],
                    position: None,
                    sketch: Some(Sketch {
                        path: PathBuf::from("sketches/home.png"),
                        areas: vec![],
                    }),
                    description: vec![],
                },
            ],
            components: vec![Component::new(Place {
                name: "Header".to_owned(),
                affordances: vec![
                    Affordance {
                        name: "Logo".to_owned(),
                        connections: vec![],
                        description: vec![],
                        level: 0,
                    },
                    Affordance {
                        name: "Contact".to_owned(),
                        connections: vec![],
                        description: vec![],
                        level: 0,
                    },
                ],
                component_references: vec![],
                position: None,
                sketch: None,
                description: vec![],
            })],
        };

        // Serialize the Breadboard
        let mut serialized_data = Vec::new();
        serialize(&mut serialized_data, &breadboard);

        // Deserialize the Breadboard
        let deserialized_breadboard: Breadboard =
            deserialize(&mut serialized_data.as_slice()).expect("Deserialization failed");

        insta::assert_json_snapshot!(deserialized_breadboard);
    }
}
