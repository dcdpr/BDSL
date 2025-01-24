//! Errors returned when generating code.

use std::{
    fmt::{self, Display},
    num::{ParseFloatError, ParseIntError},
};

use tinyjson::JsonParseError;

/// Error type returned when the code generation failed for some reason.
#[derive(Debug)]
pub enum BuildError {
    Parse(Error),
    Fmt(std::io::Error),
    Read(std::io::Error),
    Write(std::io::Error),
    Var(std::env::VarError),
    JsonParse(tinyjson::JsonParseError),
    #[cfg(feature = "toml")]
    TomlParse(toml_span::Error),
    #[cfg(feature = "ason")]
    AsonParse(ason::AsonError),
    #[cfg(feature = "jsonc")]
    JsoncParse(jsonc_parser::errors::ParseError),
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BuildError::Parse(v) => Some(v),
            BuildError::Var(v) => Some(v),
            BuildError::JsonParse(v) => Some(v),
            #[cfg(feature = "toml")]
            BuildError::TomlParse(v) => Some(v),
            #[cfg(feature = "ason")]
            BuildError::AsonParse(v) => Some(v),
            #[cfg(feature = "jsonc")]
            BuildError::JsoncParse(v) => Some(v),
            BuildError::Fmt(v) | BuildError::Read(v) | BuildError::Write(v) => Some(v),
        }
    }
}

impl Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Parse(error) => write!(f, "failed to parse file: {error}"),
            BuildError::Fmt(error) => write!(f, "failed to run rustfmt: {error}"),
            BuildError::Read(error) => write!(f, "failed to read file: {error}"),
            BuildError::Write(error) => write!(f, "failed to write file: {error}"),
            BuildError::Var(error) => write!(f, "failed to read environment variable: {error}"),
            BuildError::JsonParse(error) => write!(f, "failed to parse json file: {error}"),
            #[cfg(feature = "toml")]
            BuildError::TomlParse(error) => write!(f, "failed to parse toml file: {error}"),
            #[cfg(feature = "ason")]
            BuildError::AsonParse(error) => write!(f, "failed to parse ason file: {error}"),
            #[cfg(feature = "jsonc")]
            BuildError::JsoncParse(error) => write!(f, "failed to parse jsonc file: {error}"),
        }
    }
}

impl From<Error> for BuildError {
    fn from(value: Error) -> Self {
        Self::Parse(value)
    }
}

impl From<JsonParseError> for BuildError {
    fn from(source: JsonParseError) -> Self {
        Self::JsonParse(source)
    }
}

#[cfg(feature = "toml")]
impl From<toml_span::Error> for BuildError {
    fn from(source: toml_span::Error) -> Self {
        Self::TomlParse(source)
    }
}

#[cfg(feature = "ason")]
impl From<ason::AsonError> for BuildError {
    fn from(source: ason::AsonError) -> Self {
        Self::AsonParse(source)
    }
}

#[cfg(feature = "jsonc")]
impl From<jsonc_parser::errors::ParseError> for BuildError {
    fn from(source: jsonc_parser::errors::ParseError) -> Self {
        Self::JsoncParse(source)
    }
}

impl From<std::env::VarError> for BuildError {
    fn from(error: std::env::VarError) -> Self {
        Self::Var(error)
    }
}

/// Error type returned when a parsing error occurs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Property(&'static str, Box<Error>),
    Kind(String, Box<Error>),

    MustExist,

    ExpectedString,
    ExpectedNumber,
    ExpectedArray,
    ExpectedObject,
    UnexpectedType,

    CollectionEmpty,
    CollectionLength(usize),
    ExpectedItemString,
    ExpectedItemNumber,
    ExpectedItemObject,

    NumberMustBePositive,
    NumberWithin(isize, isize),

    InvalidNumber(String),
    InvalidUnit(&'static [&'static str]),
    InvalidFormat(&'static str),
    MissingToken(char),
}

impl Error {
    #[must_use]
    pub fn prop(property: &'static str, error: Self) -> Self {
        Self::Property(property, Box::new(error))
    }

    #[must_use]
    pub fn kind(kind: String, err: Self) -> Self {
        Self::Kind(kind, Box::new(err))
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Self::InvalidNumber(err.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidNumber(err.to_string())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Property(_, source) => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Property(prop, err) => write!(f, "property '{prop}' error: {err}"),
            Self::Kind(kind, err) => write!(f, "value error for $type '{kind}': {err}"),
            Self::MustExist => write!(f, "must exist"),
            Self::ExpectedString
            | Self::ExpectedNumber
            | Self::ExpectedArray
            | Self::ExpectedObject => {
                let ty = match self {
                    Self::ExpectedString => "string",
                    Self::ExpectedNumber => "number",
                    Self::ExpectedArray => "array",
                    _ => unreachable!(),
                };

                write!(f, "must be of type {ty}")
            }
            Self::ExpectedItemString | Self::ExpectedItemNumber | Self::ExpectedItemObject => {
                let ty = match self {
                    Self::ExpectedItemString => "string",
                    Self::ExpectedItemNumber => "number",
                    Self::ExpectedItemObject => "object",
                    _ => unreachable!(),
                };

                write!(f, "collection items must be of type {ty}")
            }
            Self::MissingToken(char) => write!(f, "missing token: {char}"),
            Self::NumberWithin(start, end) => write!(f, "number must be >= {start} <= {end}"),
            Self::UnexpectedType => write!(f, "unexpected type"),
            Self::NumberMustBePositive => write!(f, "must be a positive number"),
            Self::InvalidNumber(err) => write!(f, "invalid number format: {err}"),
            Self::InvalidUnit(units) => {
                write!(f, "invalid unit, must be one of {}", units.join(", "))
            }
            Self::InvalidFormat(str) => write!(f, "invalid format: {str}"),
            Self::CollectionEmpty => write!(f, "collection must not be empty"),
            Self::CollectionLength(len) => write!(f, "collection must contain {len} elements"),
        }
    }
}
