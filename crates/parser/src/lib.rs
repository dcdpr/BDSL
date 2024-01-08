//! # Bread'n'Butter Parser
//!
//! **A buttery smooth parsing experience.**
//!
//! `bnb-parser` is a crate for parsing textual descriptions of breadboard layouts and components.
//! It provides functionality to convert a string input into structured data representing the
//! different pieces of a breadboard in a software design context.
//!
//! ## Usage
//!
//! Add `bnb-parser` to your `Cargo.toml` dependencies.
//!
//! ```toml
//! [dependencies]
//! bnb-parser = "*"
//! ```
//!
//! Use the crate to parse breadboard descriptions:
//!
//! ```rust
//! use bnb_parser::parse;
//!
//! fn main() {
//!     let input = "place Registration\nUsername\nPassword\nSign Up -> Home";
//!
//!     match parse(input) {
//!         Ok(breadboard) => { /* explore the data! */ },
//!         Err(error) => panic!("{}", error),
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! An example of a textual breadboard description that can be parsed:
//!
//! ```ignore
//! place Registration
//!   include Header
//!
//!   Username
//!   Password
//!   Sign Up -> Home
//!
//! place Home
//!   include Header
//!
//!   Dashboard
//!
//! component Header
//!   Logo
//!   Contact
//! ```
//!
//! After parsing, the [`Breadboard`] structure will contain [`Place`] and [`Component`] instances
//! corresponding to these descriptions, which can then be used programmatically.
//!
//! ## Error Handling
//!
//! If parsing fails, a descriptive [`Error`] enum variant is returned.
//!

use std::{path::PathBuf, str::Chars};

/// The root node of the [Breadboard], containing [`Place`]s and [`Component`]s.
///
/// [Breadboard]: https://basecamp.com/shapeup/1.3-chapter-04
#[derive(Debug, PartialEq, Clone)]
pub struct Breadboard {
    /// A vector of `Place` instances, representing different locations on the breadboard.
    pub places: Vec<Place>,

    /// A vector of `Component` instances, defining the grouped affordances shared across the
    /// breadboard.
    pub components: Vec<Component>,
}

/// Represents a specific place or location on the breadboard.
#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    /// A unique identifier for the place.
    pub name: String,

    /// A list of [`Affordance`] items, representing what can be done at this place.
    pub affordances: Vec<Affordance>,

    /// A list of references to [`Component`]s.
    pub component_references: Vec<String>,

    /// An optional `Sketch` representing a visual layout or design for this place.
    pub sketch: Option<Sketch>,
}

/// Represents a component that can be referenced from [`Place`]s.
#[derive(Debug, PartialEq, Clone)]
pub struct Component {
    /// A unique identifier for the component.
    pub name: String,

    /// Grouped [`Affordance`] items, which can be collectively referenced from one or more places.
    pub affordances: Vec<Affordance>,
}

/// Describes an affordance, detailing an action or capability of a [`Place`].
#[derive(Debug, PartialEq, Clone)]
pub struct Affordance {
    /// A unique identifier for the affordance.
    pub name: String,

    /// A list of [`Connection`] items, specifying how this affordance interacts with other parts
    /// of the breadboard.
    pub connections: Vec<Connection>,
}

/// Represents a connection from an [`Affordance`] to [`Place`]s on the breadboard.
#[derive(Debug, PartialEq, Clone)]
pub struct Connection {
    /// The name of the target [`Place`] for this connection.
    pub target_place: String,

    /// An optional description of the connection.
    pub description: Option<String>,
}

/// Represents a graphical sketch or design associated with a [`Place`].
///
/// # Fields
/// * `connections` - A list of connections, each associated with a specific area of the sketch.
#[derive(Debug, PartialEq, Clone)]
pub struct Sketch {
    /// The file path to the sketch image or file.
    pub path: PathBuf,

    /// A list of connections, each associated with a specific [`Area`] of the sketch.
    pub connections: Vec<(Area, Vec<Connection>)>,
}

/// Defines a specific clickable area within a `Sketch`.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Area {
    /// The top-left coordinates of the area (x, y).
    pub top_left: (u32, u32),

    /// The width of the area.
    pub width: u32,

    /// The height of the area.
    pub height: u32,
}

/// Parses a string input to create a [`Breadboard`] structure.
///
/// # Errors
///
/// Returns an error if parsing of the string fails to produce a valid AST.
///
/// # Examples
///
/// ```
/// use bnb_parser::parse;
///
/// let input = "place ... component ...";
/// let breadboard = parse(input).unwrap();
/// ```
///
pub fn parse(input: &str) -> Result<Breadboard, Error> {
    let mut chars = input.trim().chars();
    let mut places = vec![];
    let mut components = vec![];

    loop {
        match parse_word(&mut chars) {
            "place" => places.push(parse_place(&mut chars)?),
            "component" => components.push(parse_component(&mut chars)?),
            "" => break,
            v => return Err(Error::UnexpectedToken(v.to_owned())),
        }
    }

    Ok(Breadboard { places, components })
}

fn parse_component(chars: &mut Chars) -> Result<Component, Error> {
    skip_whitespace(chars);

    let name = parse_line(chars).to_owned();
    if name.is_empty() {
        return Err(Error::MissingComponentName);
    }

    Ok(Component {
        name,
        affordances: parse_affordances(chars)?,
    })
}

fn parse_place(chars: &mut Chars) -> Result<Place, Error> {
    skip_whitespace(chars);

    let name = parse_line(chars).to_owned();
    if name.is_empty() {
        return Err(Error::MissingPlaceName);
    }

    Ok(Place {
        name,
        component_references: parse_component_references(chars)?,
        affordances: parse_affordances(chars)?,
        sketch: parse_sketch(chars)?,
    })
}

fn parse_component_references(chars: &mut Chars) -> Result<Vec<String>, Error> {
    skip_whitespace(chars);

    let mut references = vec![];

    while chars.clone().next().is_some() {
        skip_whitespace(chars);

        let str = chars.as_str();
        if !str.starts_with("include") {
            return Ok(references);
        }

        // include
        let _ = parse_word(chars);
        skip_whitespace(chars);

        let name = parse_line(chars).to_owned();
        if name.is_empty() {
            return Err(Error::MissingComponentReference);
        }

        references.push(name);
    }

    Ok(references)
}

fn parse_sketch(chars: &mut Chars) -> Result<Option<Sketch>, Error> {
    skip_whitespace(chars);

    if !chars.as_str().starts_with("sketch") {
        return Ok(None);
    }

    // sketch
    let _ = parse_word(chars);
    skip_whitespace(chars);

    let path = PathBuf::from(parse_line(chars));
    skip_whitespace(chars);

    let mut connections = vec![];
    while chars.clone().next() == Some('[') {
        let area = parse_area(chars)?;
        skip_whitespace(chars);

        let conn = parse_connections(chars)?;
        if conn.is_empty() {
            return Err(Error::SketchAreaMissingConnection);
        }

        connections.push((area, conn));
    }

    Ok(Some(Sketch { path, connections }))
}

fn parse_area(chars: &mut Chars) -> Result<Area, Error> {
    if chars.next() != Some('[') {
        return Err(Error::ExpectedSketchArea);
    }

    let parse_coordinate =
        |chars: &mut Chars, expected_delimiter: Option<char>| -> Result<u32, Error> {
            let coord = parse_int(chars)?;
            skip_whitespace(chars);
            if let Some(delimiter) = expected_delimiter {
                if chars.next() != Some(delimiter) {
                    return Err(Error::InvalidAreaCoordinates);
                }
            }
            Ok(coord)
        };

    let top = parse_coordinate(chars, Some(','))?;
    let left = parse_coordinate(chars, None)?;
    let bottom = parse_coordinate(chars, Some(','))?;
    let right = parse_coordinate(chars, None)?;

    let width = right.saturating_sub(left);
    let height = bottom.saturating_sub(top);

    if width == 0 {
        return Err(Error::InvalidAreaWidth);
    }

    if height == 0 {
        return Err(Error::InvalidAreaHeight);
    }

    if chars.next() != Some(']') {
        return Err(Error::UnterminatedSketchArea);
    }

    Ok(Area {
        top_left: (top, left),
        width,
        height,
    })
}

fn parse_int(chars: &mut Chars<'_>) -> Result<u32, Error> {
    let str = chars.as_str();
    while chars.clone().next().map_or(false, |c| c.is_digit(10)) {
        chars.next();
    }

    str[..str.len() - chars.as_str().len()]
        .parse::<u32>()
        .map_err(|e| Error::InvalidInteger(e.to_string()))
}

fn parse_affordances(chars: &mut Chars) -> Result<Vec<Affordance>, Error> {
    skip_whitespace(chars);

    let mut affordances = vec![];

    while chars.clone().next().is_some() {
        skip_whitespace(chars);

        let str = chars.as_str();
        if str.is_empty()
            || str.starts_with("place")
            || str.starts_with("component")
            || str.starts_with("sketch")
        {
            return Ok(affordances);
        }

        let name = parse_affordance_or_target_name(chars)?.to_owned();
        if name.is_empty() {
            return Ok(affordances);
        }

        affordances.push(Affordance {
            name,
            connections: parse_connections(chars)?,
        });
    }

    Ok(affordances)
}

fn parse_connections(chars: &mut Chars) -> Result<Vec<Connection>, Error> {
    let mut connections = vec![];
    while chars.clone().next().is_some() {
        skip_whitespace(chars);

        if !chars.as_str().starts_with("->") {
            break;
        }

        chars.next();
        chars.next();
        skip_whitespace(chars);

        // description
        let description = (chars.clone().next() == Some('('))
            .then(|| parse_connection_description(chars))
            .transpose()?;
        let target_place = parse_affordance_or_target_name(chars)?.to_owned();

        connections.push(Connection {
            target_place,
            description,
        });
    }

    Ok(connections)
}

fn parse_affordance_or_target_name<'a>(chars: &'a mut Chars) -> Result<&'a str, Error> {
    let str = chars.as_str();

    match chars.clone().next() {
        Some('"') => parse_quoted_string(chars),
        _ => {
            while chars
                .clone()
                .next()
                .map_or(false, |c| c != '\n' && c != '(')
            {
                if chars.as_str().starts_with("->") {
                    break;
                }

                chars.next();
            }

            Ok(str[..str.len() - chars.as_str().len()].trim())
        }
    }
}

fn parse_connection_description<'a>(chars: &'a mut Chars) -> Result<String, Error> {
    if chars.next() != Some('(') {
        return Err(Error::ExpectedConnectionDescription);
    }

    let start = chars.as_str();
    let desc = match chars.clone().next() {
        Some('"') => parse_quoted_string(chars)?.to_owned(),
        _ => {
            while chars
                .clone()
                .next()
                .map_or(false, |c| c != '\n' && c != ')')
            {
                chars.next();
            }

            let end = chars.as_str();
            start[..start.len() - end.len()].to_owned()
        }
    };

    if chars.next() != Some(')') {
        return Err(Error::UnterminatedConnectionDescription);
    }

    Ok(desc)
}

fn parse_quoted_string<'a>(chars: &'a mut Chars) -> Result<&'a str, Error> {
    match chars.next() {
        Some('"') => (),
        _ => return Err(Error::ExpectedQuotedString),
    }

    let start = chars.as_str();

    let mut escape = false;
    for c in chars.clone() {
        if c == '"' && !escape {
            let end = chars.as_str();
            chars.next(); // Consume the closing quote
            return Ok(&start[..start.len() - end.len()]);
        }
        escape = c == '\\' && !escape;
        chars.next();
    }

    Err(Error::UnterminatedQuotedString)
}

fn parse_word<'a>(chars: &'a mut Chars) -> &'a str {
    let str = chars.as_str();

    while chars.clone().next().map_or(false, |c| !c.is_whitespace()) {
        chars.next();
    }

    &str[..str.len() - chars.as_str().len()]
}

fn parse_line<'a>(chars: &'a mut Chars) -> &'a str {
    let str = chars.as_str();

    while chars.clone().next().map_or(false, |c| c != '\n') {
        chars.next();
    }

    &str[..str.len() - chars.as_str().len()]
}

fn skip_whitespace(chars: &mut Chars) {
    while chars.clone().next().map_or(false, char::is_whitespace) {
        chars.next();
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("missing place name")]
    MissingPlaceName,

    #[error("missing component name")]
    MissingComponentName,

    #[error("missing component reference")]
    MissingComponentReference,

    #[error("expected quoted string")]
    ExpectedQuotedString,

    #[error("unterminated quoted string")]
    UnterminatedQuotedString,

    #[error("expected connection description")]
    ExpectedConnectionDescription,

    #[error("unterminated connection description")]
    UnterminatedConnectionDescription,

    #[error("invalid sketch path: {0}")]
    InvalidSketchPath(String),

    #[error("sketch area must have at least one connection")]
    SketchAreaMissingConnection,

    #[error("expected sketch area")]
    ExpectedSketchArea,

    #[error("unterminated sketch area")]
    UnterminatedSketchArea,

    #[error("invalid area coordinates")]
    InvalidAreaCoordinates,

    #[error("area width must be a positive number")]
    InvalidAreaWidth,

    #[error("area height must be a positive number")]
    InvalidAreaHeight,

    #[error("invalid integer: {0}")]
    InvalidInteger(String),

    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_snapshots() {
        let test_cases = vec![
            indoc! {r#"
                place Home
            "#},
            indoc! {r#"
                place Registration
                  include Header

                  Username
                  Password
                  Sign Up -> Home

                place Home
                  include Header

                  Dashboard

                component Header
                  Logo
                  Contact
            "#},
            indoc! {r#"
                place invoice
                  Turn on autopay -> Set up autopay -> Foo bar -> (test) test 2
                place two
                place three and more!
                  "free -> form!" -> Not -> "(test)"
                  another one!
                  sketch foo/bar.png
                    [0,0 10,10] -> (if foo) Pay Invoice -> Another one
                    [20,20 30,30] -> Pay Twice
                place four!
            "#},
        ];

        for case in test_cases {
            insta::assert_debug_snapshot!(parse(case));
        }
    }

    #[test]
    fn test_parse_connection_description() {
        let test_cases = vec![
            ("(simple description)", Ok("simple description".to_owned())),
            (
                "(description with newline\n)",
                Err(Error::UnterminatedConnectionDescription),
            ),
            ("(\"quoted string\")", Ok("quoted string".to_owned())),
            (
                "(\"escaped \\\"quote\\\"\")",
                Ok("escaped \\\"quote\\\"".to_owned()),
            ),
            (
                "(multi\nline\ndescription)",
                Err(Error::UnterminatedConnectionDescription),
            ),
            (
                "(description with special!@#)",
                Ok("description with special!@#".to_owned()),
            ),
            (
                "(unterminated",
                Err(Error::UnterminatedConnectionDescription),
            ),
            ("no parenthesis", Err(Error::ExpectedConnectionDescription)),
            (
                "(unterminated \"quoted string)",
                Ok("unterminated \"quoted string".to_owned()),
            ),
            ("()", Ok("".to_owned())),
            (
                "(description with ) inside)",
                Ok("description with ".to_owned()),
            ),
            (
                "(\"quoted with ) inside\")",
                Ok("quoted with ) inside".to_owned()),
            ),
        ];

        for (input, expected) in test_cases {
            let mut chars = input.chars();
            let result = parse_connection_description(&mut chars);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_quoted_string() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("\"simple string\"", Ok("simple string")),
            ("\"string with \\\"escaped quotes\\\"\"", Ok("string with \\\"escaped quotes\\\"")),
            ("\"\"", Ok("")),
            ("\"string with spaces\"", Ok("string with spaces")),
            ("\"string with newline\\n\"", Ok("string with newline\\n")),
            ("\"string with tab\\t\"", Ok("string with tab\\t")),
            ("\"string with various \\\"special\\\" characters!@#\"", Ok("string with various \\\"special\\\" characters!@#")),
            ("\"unterminated string", Err(Error::UnterminatedQuotedString)),
            ("no quotes", Err(Error::ExpectedQuotedString)),
            ("\"escaped backslash \\\\\"", Ok("escaped backslash \\\\")),
            ("\"multi\nline\"", Ok("multi\nline")),
            ("\"string with \\\\\\\"escaped quote\"", Ok("string with \\\\\\\"escaped quote")),
        ];

        for (input, expected) in test_cases {
            let mut chars = input.chars();
            let result = parse_quoted_string(&mut chars);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_line() {
        let test_cases = vec![
            ("first line\nsecond line", "first line"),
            ("single_line", "single_line"),
            ("", ""),
            ("line with space ", "line with space "),
            ("line with \t tab", "line with \t tab"),
            ("multi\nline\nstring", "multi"),
            ("line with\nnewline", "line with"),
            ("\nstarts with newline", ""),
            ("ends with newline\n", "ends with newline"),
            ("line with special!@#\nchars", "line with special!@#"),
        ];

        for (input, expected) in test_cases {
            let mut chars = input.chars();
            let result = parse_line(&mut chars);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_word() {
        let test_cases = vec![
            ("hello world", "hello"),
            ("  leading spaces", ""),
            ("", ""),
            ("no_spaces", "no_spaces"),
            ("word ", "word"),
            ("multiple words", "multiple"),
            ("word\nnextline", "word"),
            ("\t\tword\ttabs", ""),
            ("1234 number", "1234"),
            ("special_chars!@#", "special_chars!@#"),
            ("mixed 1234 words", "mixed"),
            ("end", "end"),
        ];

        for (input, expected) in test_cases {
            let mut chars = input.chars();
            let result = parse_word(&mut chars);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_skip_whitespace() {
        let test_cases = vec![
            ("  hello", "hello"),
            ("\t\n world", "world"),
            ("", ""),
            ("no_spaces", "no_spaces"),
            (" \t\n ", ""),
            (" a b ", "a b "),
            ("\n\n\nthree_newlines", "three_newlines"),
            ("single space", "single space"),
        ];

        for (input, expected) in test_cases {
            let mut chars = input.chars();
            skip_whitespace(&mut chars);
            let result: String = chars.collect();
            assert_eq!(result, expected);
        }
    }
}
