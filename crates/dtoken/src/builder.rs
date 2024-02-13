use std::{
    collections::HashMap,
    env,
    fmt::{self, Display},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use tinyjson::JsonValue;

use crate::{
    error::{BuildError, ConfigError},
    gen, parser,
};

/// Helper function that return an default [`TokensBuilder`].
pub fn build() -> DesignTokensBuilder {
    DesignTokensBuilder::default()
}

/// Builder used to configure Rosetta code generation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DesignTokensBuilder {
    files: Vec<PathBuf>,
    raw: HashMap<String, String>,
    name: Option<String>,
    output: Option<PathBuf>,
}

impl DesignTokensBuilder {
    /// Register a new tokens source.
    pub fn source(mut self, path: impl Into<String>) -> Self {
        self.files.push(PathBuf::from(path.into()));
        self
    }

    /// Define a custom name for the output type
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Change the default output of generated files
    pub fn output(mut self, path: impl Into<PathBuf>) -> Self {
        self.output = Some(path.into());
        self
    }

    /// Generate locale files and write them to the output location
    pub fn generate(self) -> Result<(), BuildError> {
        self.build()?.generate()?;
        Ok(())
    }

    /// Validate configuration and build a [`TokensConfig`]
    fn build(self) -> Result<TokensConfig, ConfigError> {
        let groups: HashMap<TokensId, PathBuf> = self
            .files
            .into_iter()
            .map(|path| {
                let id = TokensId::from_path(path.as_path()).ok_or(ConfigError::InvalidGroup(
                    path.to_string_lossy().to_string(),
                ))?;
                Ok((id, path))
            })
            .collect::<Result<_, _>>()?;

        if groups.is_empty() {
            return Err(ConfigError::MissingSource);
        }

        Ok(TokensConfig {
            groups,
            name: self.name.unwrap_or_else(|| "Token".to_string()),
            output: self.output,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TokensId(pub String);

impl TokensId {
    fn from_path(path: &Path) -> Option<Self> {
        Some(Self(path.file_stem()?.to_string_lossy().to_string()))
    }

    pub(crate) fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for TokensId {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ascii_alphabetic = s.chars().all(|c| c.is_ascii_alphabetic());

        if ascii_alphabetic {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(ConfigError::InvalidGroup(s.into()))
        }
    }
}

impl Display for TokensId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Configuration for Tokens code generation
///
/// A [`TokensBuilder`] is provided to construct and validate configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokensConfig {
    pub groups: HashMap<TokensId, PathBuf>,
    pub name: String,
    pub output: Option<PathBuf>,
}

impl TokensConfig {
    /// Returns a list of the group names
    pub fn groups(&self) -> Vec<&TokensId> {
        self.groups.iter().map(|(group, _)| group).collect()
    }

    /// Generate token files and write them to the output location
    pub fn generate(&self) -> Result<(), BuildError> {
        let mut parsed = parser::TokensData::default();

        for (group, path) in &self.groups {
            let content = open_file(path)?;
            parsed.parse_file(group.clone(), content)?;
            println!("cargo:rerun-if-changed={}", path.to_string_lossy());
        }

        let generated = gen::CodeGenerator::new(&parsed, self).generate();

        let output = match &self.output {
            Some(path) => path.clone(),
            None => Path::new(&env::var("OUT_DIR")?).join("design_tokens.rs"),
        };

        let mut file = File::create(&output)?;
        file.write_all(generated.to_string().as_bytes())?;

        #[cfg(feature = "rustfmt")]
        rustfmt(&output)?;

        Ok(())
    }
}

/// Open a file and read its content as a JSON [`JsonValue`]
fn open_file(path: &Path) -> Result<JsonValue, BuildError> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            return Err(BuildError::FileRead {
                file: path.to_path_buf(),
                source: error,
            })
        }
    };

    match content.parse::<JsonValue>() {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(BuildError::JsonParse {
            file: path.to_path_buf(),
            source: error,
        }),
    }
}

/// Format a file with rustfmt
#[cfg(feature = "rustfmt")]
fn rustfmt(path: &Path) -> Result<(), BuildError> {
    use std::process::Command;

    Command::new(env::var("RUSTFMT").unwrap_or_else(|_| "rustfmt".to_string()))
        .args(&["--emit", "files"])
        .arg(path)
        .output()
        .map_err(BuildError::Fmt)?;

    Ok(())
}
