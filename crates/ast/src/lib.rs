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

use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// The root node of the [Breadboard], containing [`Place`]s and [`Component`]s.
///
/// [Breadboard]: https://basecamp.com/shapeup/1.3-chapter-04
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
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

    /// An optional description added to the place.
    pub description: Vec<String>,

    /// A list of [`Item]` elements contained in the place.
    pub items: Vec<Item>,

    /// The desired position of the place, as x/y coordinates.
    ///
    /// Note that relative position [`Coordinate`]s are expected to be correct for their given
    /// axis. Meaning, for the `x` axis, the relative offset can be to the left or right of the
    /// target, but *not* top or bottom, those are used for the `y` axis.
    pub position: Option<Position>,

    /// An optional `Sketch` representing a visual layout or design for this place.
    pub sketch: Option<Sketch>,
}

/// Represents the desired position for a given place.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: Coordinate,
    pub y: Coordinate,
}

/// Represents one coordinate of a desired position for a given place.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Coordinate {
    /// An absolute position within the [`Breadboard`] canvas.
    ///
    /// `0` is the center of the canvas, positive numbers to to the right and downwards.
    Absolute(i32),

    /// A relative position calculated from the given *other* [`Place`].
    ///
    /// A relative position with `offset` set to `0` and `pivot` set to [`Pivot::Center`] means
    /// this place is positioned exactly on top of the reference place.
    ///
    /// Note that these are marked as *desired* positions, any implementation such as a GUI will
    /// likely not render two places on top of each other, to avoid any confusion or visual
    /// artifacts.
    ///
    /// Any other pivot variant moves the current place to one of the four sides of the target. For
    /// example, `Right` aligns the left side of the current place with the right side of the
    /// target place, essentially aligning the current place to the right of the target.
    ///
    /// Again, libraries might add their own interpretation. For example, a GUI might always add
    /// some padding between two aligned places, even if `offset` is set to `0`.
    Relative {
        place: String,
        offset: i32,
        pivot: Pivot,
    },
}

/// The relative position from which an offset is calculated.
#[derive(Debug, PartialEq, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Pivot {
    #[default]
    Center,
    Top,
    Right,
    Bottom,
    Left,
}

/// Represents a component that can be referenced from [`Place`]s.
///
/// Internally, a component is the same as a [`Place`].
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Component(Place);

impl Component {
    /// Creates a new [`Component`] from the given [`Place`].
    pub fn new(place: Place) -> Self {
        Self(place)
    }
}

impl Deref for Component {
    type Target = Place;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Describes an item within a [`Place`].
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Affordance(Affordance),
    Reference(Reference),
}

/// Describes an affordance, detailing an action or capability of a [`Place`].
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Affordance {
    /// A unique identifier for the affordance.
    pub name: String,

    /// An optional description added to the affordance.
    pub description: Vec<String>,

    /// A list of [`Connection`] items, specifying how this affordance interacts with other parts
    /// of the breadboard.
    pub connections: Vec<Connection>,

    /// The nesting level of the affordance.
    ///
    /// By default this is set to 0.
    pub level: usize,
}

/// Describes a reference to a [`Component`] embedded in a [`Place`].
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// A unique identifier for the referenced [`Component`].
    pub name: String,

    /// The nesting level of the reference.
    ///
    /// By default this is set to 0.
    pub level: usize,
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

    /// A list of clickable areas.
    pub areas: Vec<Area>,
}

/// Defines a specific clickable area within a `Sketch`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Area {
    /// The top-left coordinates of the area (x, y).
    pub top_left: (u32, u32),

    /// The width of the area.
    pub width: u32,

    /// The height of the area.
    pub height: u32,

    /// The name of the [`Affordance`] within the [`Place`] of the sketch, this area belongs to.
    pub affordance: String,
}
