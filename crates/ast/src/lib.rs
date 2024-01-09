//! # Bread'n'Butter AST
//!
//! **A buttery smooth AST experience.**
//!
//! The `bnb_ast` crate provides the core Abstract Syntax Tree (AST) structures for the
//! Bread'n'Butter project. It defines the data models used to represent the elements of a
//! breadboard in a software development context.
//!
//! This crate is essential for the Bread'n'Butter ecosystem, as it allows for a structured and
//! programmable way to interact with breadboard configurations.
//!
//! ## Overview
//!
//! The crate defines several primary structures:
//!
//! - [`Breadboard`]: The root node representing the entire breadboard configuration.
//! - [`Place`]: Represents a specific location or section on the breadboard.
//! - [`Component`]: Defines a reusable component that can be referenced from places.
//! - [`Affordance`]: Details an action or capability associated with a place.
//! - [`Connection`]: Represents a link from an affordance to places on the breadboard.
//! - [`Sketch`]: A graphical representation associated with a place, including clickable areas.
//! - [`Area`]: A specific clickable area within a `Sketch`.
//!
//! ## Usage
//!
//! The AST structures in this crate are primarily used by the parser and the graphical interface
//! components of the Bread'n'Butter project. They provide the necessary data models for
//! interpreting breadboard descriptions and for rendering visual representations of these
//! configurations.
//!
//! Here's a brief example of how these structures might be used:
//!
//! ```
//! use bnb_ast::{Breadboard, Place, Component, Affordance, Connection, Sketch, Area};
//!
//! // Create a new breadboard instance
//! let breadboard = Breadboard {
//!     places: vec![],
//!     components: vec![],
//! };
//!
//! // Add places and components as needed
//! // ...
//! ```

use serde::{Deserialize, Serialize};

/// The root node of the [Breadboard], containing [`Place`]s and [`Component`]s.
///
/// [Breadboard]: https://basecamp.com/shapeup/1.3-chapter-04
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Breadboard {
    /// A vector of `Place` instances, representing different locations on the breadboard.
    pub places: Vec<Place>,

    /// A vector of `Component` instances, defining the grouped affordances shared across the
    /// breadboard.
    pub components: Vec<Component>,
}

/// Represents a specific place or location on the breadboard.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Place {
    /// A unique identifier for the place.
    pub name: String,

    /// A list of [`Affordance`] items, representing what can be done at this place.
    pub affordances: Vec<Affordance>,

    /// A list of references to [`Component`]s.
    pub component_references: Vec<String>,

    /// An optional `Sketch` representing a visual layout or design for this place.
    pub sketch: Option<Sketch>,
}

/// Represents a component that can be referenced from [`Place`]s.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Component {
    /// A unique identifier for the component.
    pub name: String,

    /// Grouped [`Affordance`] items, which can be collectively referenced from one or more places.
    pub affordances: Vec<Affordance>,
}

/// Describes an affordance, detailing an action or capability of a [`Place`].
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Affordance {
    /// A unique identifier for the affordance.
    pub name: String,

    /// A list of [`Connection`] items, specifying how this affordance interacts with other parts
    /// of the breadboard.
    pub connections: Vec<Connection>,
}

/// Represents a connection from an [`Affordance`] to [`Place`]s on the breadboard.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// The name of the target [`Place`] for this connection.
    pub target_place: String,

    /// An optional description of the connection.
    pub description: Option<String>,
}

/// Represents a graphical sketch or design associated with a [`Place`].
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Sketch {
    /// The file path to the sketch image or file.
    pub path: std::path::PathBuf,

    /// A list of connections, each associated with a specific [`Area`] of the sketch.
    pub connections: Vec<(Area, Vec<Connection>)>,
}

/// Defines a specific clickable area within a `Sketch`.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Area {
    /// The top-left coordinates of the area (x, y).
    pub top_left: (u32, u32),

    /// The width of the area.
    pub width: u32,

    /// The height of the area.
    pub height: u32,
}
