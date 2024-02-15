use crate::prelude::*;

#[derive(Component, Deref)]
pub(crate) struct Description(String);

impl From<String> for Description {
    fn from(description: String) -> Self {
        Self(description)
    }
}

/// Marker component for affordance title entities.
#[derive(Component, Default)]
pub(super) struct Title;

/// Bundle of required components for affordance title entities.
#[derive(Bundle)]
pub(super) struct TitleBundle {
    name: Name,
    marker: Title,
    visibility: VisibilityBundle,
    transform: TransformBundle,
}

impl TitleBundle {
    pub(super) fn new(name: impl Into<Name>) -> Self {
        Self {
            name: name.into(),
            marker: Title,
            visibility: VisibilityBundle::default(),
            transform: TransformBundle::default(),
        }
    }
}
