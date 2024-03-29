//! Errors returned when generating code.

use std::{
    error::Error,
    fmt::{self, Display},
    path::PathBuf,
};

use tinyjson::JsonParseError;

/// Error type returned when the configuration passed to [`GroupBuilder`] is invalid.
///
/// [`GroupBuilder`]: crate::GroupBuilder
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// Invalid language identifier
    InvalidGroup(String),
    /// No source provided
    MissingSource,
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidGroup(value) => {
                write!(f, "`{}` is not a valid group identifier", value)
            }
            ConfigError::MissingSource => {
                write!(f, "at least one translations source file is required")
            }
        }
    }
}

/// Error type returned when the code generation failed for some reason.
#[derive(Debug)]
pub enum BuildError {
    Config(ConfigError),
    FileRead {
        file: PathBuf,
        source: std::io::Error,
    },
    FileWrite(std::io::Error),
    JsonParse {
        source: tinyjson::JsonParseError,
    },
    Parse,
    Var(std::env::VarError),
    Fmt(std::io::Error),
}

impl Error for BuildError {}

impl Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Config(error) => write!(f, "invalid configuration: {}", error),
            BuildError::FileRead { file, source } => {
                write!(f, "failed to read `{:?}`: {}", file, source)
            }
            BuildError::FileWrite(error) => write!(f, "failed to write output: {}", error),
            BuildError::JsonParse { source } => {
                write!(f, "failed to load file: {}", source)
            }
            BuildError::Parse => write!(f, "failed to parse file"),
            BuildError::Var(error) => write!(f, "failed to read environment variable: {}", error),
            BuildError::Fmt(error) => write!(f, "failed to run rustfmt: {}", error),
        }
    }
}

impl From<ConfigError> for BuildError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}

impl From<JsonParseError> for BuildError {
    fn from(source: JsonParseError) -> Self {
        Self::JsonParse { source }
    }
}

// impl From<ParseError> for BuildError {
//     fn from(error: ParseError) -> Self {
//         Self::Parse(error)
//     }
// }

impl From<std::io::Error> for BuildError {
    fn from(error: std::io::Error) -> Self {
        Self::FileWrite(error)
    }
}

impl From<std::env::VarError> for BuildError {
    fn from(error: std::env::VarError) -> Self {
        Self::Var(error)
    }
}

/// Error type returned when a parsing error occurs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// File root is not a JSON object
    InvalidRoot,
    /// Invalid key type (raw parsing)
    InvalidValue { key: String },
    /// Invalid key type (doesn't match previous parsed keys)
    InvalidType { key: String, expected: &'static str },
    /// Invalid parameters supplied to interpolated key (missing and/or unknown parameters)
    InvalidParameters {
        key: String,
        missing: Vec<String>,
        unknown: Vec<String>,
    },
    /// Invalid language identifier (not ISO 693-1 compliant)
    InvalidLanguageId { value: String },
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidRoot => write!(f, "file root must be a json object"),
            ParseError::InvalidValue { key } => write!(f, "`{}` has an invalid type", key),
            ParseError::InvalidType { key, expected } => write!(
                f,
                "`{}` doesn't match previous parsed key (expected {})",
                key, expected
            ),
            ParseError::InvalidParameters {
                key,
                missing,
                unknown,
            } => write!(
                f,
                "invalid parameters supplied to `{}` (missing: {:?}, unknown: {:?})",
                key, missing, unknown
            ),
            ParseError::InvalidLanguageId { value } => write!(
                f,
                "`{}` is not a valid ISO 693-1 language identifier",
                value
            ),
        }
    }
}
