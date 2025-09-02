use crate::prelude::*;

/// Represents the sequential index of an entity among its siblings.
///
/// This component is used to denote the order or position of an entity relative to others of a
/// similar kind, facilitating the organization and sorting of entities based on their defined
/// sequence.
#[derive(Component, Default, Ord, Eq, PartialEq, PartialOrd, Deref, Copy, Clone)]
pub(super) struct Index(pub(super) usize);

/// Identifies entities as headers within the hierarchical structure.
///
/// Used to mark entities that serve as headers, providing a way to distinguish these elements for
/// styling, positioning, or logical grouping purposes within the broader context of their parent
/// entities, such as places or affordances.
#[derive(Component, Default)]
pub(super) struct Header;

/// Bundle of required components for header entities.
#[derive(Bundle)]
pub(super) struct HeaderBundle {
    marker: Header,
    visibility: Visibility,
    transform: Transform,
    size: ComputedSize,
}

impl Default for HeaderBundle {
    fn default() -> Self {
        Self {
            size: ComputedSize::Inherit,
            marker: default(),
            visibility: default(),
            transform: default(),
        }
    }
}

/// Marks entities as the body section within their respective contexts.
///
/// This component distinguishes entities that represent the body sections, typically containing
/// detailed information or additional components related to the parent entity, such as a place or
/// affordance.
#[derive(Component, Default)]
pub(super) struct Body;

#[derive(Bundle)]
pub(super) struct BodyBundle {
    marker: Body,
    visibility: Visibility,
    transform: Transform,
    size: ComputedSize,
}

impl Default for BodyBundle {
    fn default() -> Self {
        Self {
            size: ComputedSize::Inherit,
            marker: default(),
            visibility: default(),
            transform: default(),
        }
    }
}

/// Holds descriptive text for an entity.
///
/// Encapsulates a textual description for an entity, providing a flexible means to attach
/// explanatory or supplementary information directly to entities such as affordances or places.
#[derive(Component, Deref)]
pub(super) struct Description(String);

impl From<String> for Description {
    fn from(description: String) -> Self {
        Self(description)
    }
}

/// Designates entities as titles.
///
/// This component is used to label entities that function as titles, facilitating their
/// identification for styling and positioning.
#[derive(Component, Default)]
pub(super) struct Title;

/// Bundle of required components for affordance title entities.
#[derive(Bundle, Default)]
pub(super) struct TitleBundle {
    name: Name,
    marker: Title,
    visibility: Visibility,
    transform: Transform,
    size: ComputedSize,
}

impl TitleBundle {
    pub fn new(name: impl Into<Name>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    pub(crate) fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// Designates the number span of a [`Title`]
#[derive(Component, Default)]
pub(super) struct TitleNumberSpan;

#[derive(Bundle, Default)]
pub(super) struct TitleNumberSpanBundle {
    text: TextSpan,
    marker: TitleNumberSpan,
    visibility: Visibility,
    transform: Transform,
    size: ComputedSize,
}

impl TitleNumberSpanBundle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: TextSpan::new(text),
            ..default()
        }
    }
}
