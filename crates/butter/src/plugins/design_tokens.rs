use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/design_tokens.rs"));

/// Application Design Tokens.
pub(crate) struct DesignTokensPlugin;

#[derive(Resource, Deref)]
pub(crate) struct DesignTokens(design_tokens::DesignTokens);

impl Plugin for DesignTokensPlugin {
    fn build(&self, app: &mut App) {
        app.world.insert_resource(DesignTokens(design_tokens()))
    }
}
