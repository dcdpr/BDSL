//! Code generation for the Rosetta i18n library.
//!
//! # Usage
//!
//! Code generation works within [build script]. You only need to configure source files. Please
//! read the [Design Tokens specification] for more information.
//!
//! ```no_run
//! dtoken::build("design_tokens.json").unwrap()
//! ```
//!
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [Design Tokens specification]: https://tr.designtokens.org

pub mod error;
pub mod parser;
pub mod types;

#[cfg(feature = "build")]
mod build;

#[cfg(feature = "build")]
pub use build::{build, build_merge, Config};

#[cfg(feature = "bevy")]
pub mod bevy;
