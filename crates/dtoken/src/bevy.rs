use bevy_ui::Val;

use crate::types::{color::Color, dimension::Dimension};

impl From<Dimension> for Val {
    fn from(value: Dimension) -> Self {
        match value {
            Dimension::Pixels(v) => Self::Px(v as f32),
            Dimension::Rems(_) => unimplemented!("Bevy does not currently support Rem units"),
        }
    }
}

impl From<Color> for bevy_render::color::Color {
    fn from(value: Color) -> Self {
        Self::rgba_from_array(value.to_rgba())
    }
}
