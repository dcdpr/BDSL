use crate::prelude::*;

use super::computed_size;

pub(crate) struct ErrorHandlerPlugin;

impl Plugin for ErrorHandlerPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    ComputedSize(#[from] computed_size::Error),
}

pub(crate) fn err(result: Result<(), Error>) {
    if let Err(error) = result {
        error!(?error, "System error.");
    }
}
