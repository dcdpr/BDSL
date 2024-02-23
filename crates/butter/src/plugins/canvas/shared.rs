use crate::prelude::*;

/// The index/position of an entity, relative to similar entities.
#[derive(Component, Default, Ord, Eq, PartialEq, PartialOrd)]
pub(super) struct Index(pub(super) usize);

/// Marker component for header entities.
#[derive(Component, Default)]
pub(super) struct Header;

/// Bundle of required components for header entities.
#[derive(Bundle)]
pub(super) struct HeaderBundle {
    marker: Header,
    visibility: VisibilityBundle,
    transform: TransformBundle,
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

#[derive(Component, Default)]
pub(super) struct Body;

#[derive(Bundle)]
pub(super) struct BodyBundle {
    marker: Body,
    visibility: VisibilityBundle,
    transform: TransformBundle,
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

#[derive(Component, Deref)]
pub(super) struct Description(String);

impl From<String> for Description {
    fn from(description: String) -> Self {
        Self(description)
    }
}

/// Marker component for affordance title entities.
#[derive(Component, Default)]
pub(super) struct Title;

/// Bundle of required components for affordance title entities.
#[derive(Bundle, Default)]
pub(super) struct TitleBundle {
    name: Name,
    marker: Title,
    visibility: VisibilityBundle,
    transform: TransformBundle,
    size: ComputedSize,
}

impl TitleBundle {
    pub fn new(name: impl Into<Name>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }
}
