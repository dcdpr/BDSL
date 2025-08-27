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
//! let input = "place Registration\nUsername\nPassword\nSign Up -> Home";
//!
//! match parse(input) {
//!     Ok(breadboard) => { /* explore the data! */ },
//!     Err(error) => panic!("{}", error),
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

use std::{
    path::PathBuf,
    str::{Chars, FromStr},
};

use bnb_ast::{
    Affordance, Area, Breadboard, Component, Connection, Coordinate, Item, Pivot, Place, Position,
    Reference, Sketch,
};
use tracing::instrument;

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
#[instrument(skip_all)]
pub fn parse(input: &str) -> Result<Breadboard, Error> {
    let mut chars = input.trim().chars();
    let mut places = vec![];
    let mut components = vec![];

    loop {
        let description = parse_comment(&mut chars);

        match parse_word(&mut chars) {
            "place" => places.push(parse_place(&mut chars, description)?),
            "component" => components.push(parse_component(&mut chars, description)?),
            "" => break,
            v => return Err(Error::UnexpectedToken(v.to_owned())),
        }
    }

    Ok(Breadboard { places, components })
}

#[instrument(level = "trace", skip_all)]
fn parse_comment(chars: &mut Chars<'_>) -> Vec<String> {
    let mut comment = vec![];

    // Continuously parse consecutive comment lines (even if the comments are interleaved with
    // empty lines).
    loop {
        skip_whitespace(chars);

        if !chars.as_str().starts_with("//") {
            break;
        }

        let line = &parse_line(chars)[2..];

        // This is not a description-style comment, so we'll ignore it.
        if !line.starts_with('/') {
            continue;
        }

        // Remove exactly *one* leading space, if present.
        let n: usize = line.starts_with("/ ").into();
        comment.push(line[n + 1..].to_owned());
    }

    comment
}

#[instrument(skip_all)]
fn parse_component(chars: &mut Chars<'_>, description: Vec<String>) -> Result<Component, Error> {
    let place = parse_place(chars, description)?;

    Ok(Component::new(place))
}

#[instrument(skip_all)]
fn parse_place(chars: &mut Chars<'_>, description: Vec<String>) -> Result<Place, Error> {
    skip_whitespace(chars);

    let name = parse_line(chars).to_owned();
    if name.is_empty() {
        return Err(Error::MissingPlaceName);
    }

    Ok(Place {
        name,
        description,
        items: parse_items(chars)?,
        position: parse_position(chars)?,
        sketch: parse_sketch(chars)?,
    })
}

#[instrument(level = "debug", skip_all)]
fn parse_position(chars: &mut Chars<'_>) -> Result<Option<Position>, Error> {
    skip_whitespace(chars);

    if !chars.as_str().starts_with("position") {
        return Ok(None);
    }

    // Consume the 'position' word
    let _ = parse_word(chars);
    parse_while(chars, |c| c.is_whitespace() && c != '\n');

    let mut x = parse_coordinate(chars)?.ok_or(Error::MissingCoordinate)?;

    parse_until(chars, ",\n");
    if chars.clone().next() == Some(',') {
        chars.next();
    }

    let y = parse_coordinate(chars)?;
    let y_missing = y.is_none();

    let mut y = y.unwrap_or_else(|| match &x {
        Coordinate::Absolute(_) => Coordinate::Absolute(0),
        Coordinate::Relative { place, .. } => Coordinate::Relative {
            place: place.to_owned(),
            offset: 0,
            pivot: Pivot::Center,
        },
    });

    // Swap x and y if x is a relative coordinate with 'top' or 'bottom' pivot
    if let &Coordinate::Relative { pivot, .. } = &x {
        if y_missing && (pivot == Pivot::Top || pivot == Pivot::Bottom) {
            (x, y) = (y, x);
        }
    }

    // Validate pivot points
    if let &Coordinate::Relative { pivot, .. } = &x {
        if pivot == Pivot::Top || pivot == Pivot::Bottom {
            return Err(Error::InvalidCoordinatePivot);
        }
    }
    if let &Coordinate::Relative { pivot, .. } = &y {
        if pivot == Pivot::Left || pivot == Pivot::Right {
            return Err(Error::InvalidCoordinatePivot);
        }
    }

    Ok(Some(Position { x, y }))
}

#[instrument(level = "debug", skip_all)]
fn parse_coordinate(chars: &mut Chars<'_>) -> Result<Option<Coordinate>, Error> {
    parse_while(chars, |c| c.is_whitespace() && c != '\n');

    // If we start with a newline char or there are no more characters, there's no coordinate
    if chars.clone().next().is_none_or(|c| c == '\n') {
        return Ok(None);
    }

    let pivot = match chars.clone().next() {
        Some('^') => Pivot::Top,
        Some('>') => Pivot::Right,
        Some('_') => Pivot::Bottom,
        Some('<') => Pivot::Left,
        _ => Pivot::Center,
    };

    // Consume the pivot character, if any.
    if !matches!(pivot, Pivot::Center) {
        chars.next();
    }

    parse_while(chars, |c| c.is_whitespace() && c != '\n');

    // After the optional pivot, more characters should follow.
    let c = match chars.clone().next() {
        None | Some('\n') => return Err(Error::InvalidPosition),
        Some(c) => c,
    };

    // If the next non-whitespace character is a quote, we need to parse every character until the
    // next unescaped quote as a quoted string.
    //
    // If not, we check if there's any valid "unquoted string" character (e.g. anything except `+`,
    // `-`, a newline, or a digit character), and take those as being an unquoted string.
    let place = (c == '"')
        .then(|| parse_quoted_string(chars).map(ToOwned::to_owned))
        .transpose()?
        .or_else(|| {
            (c != '+' && c != '-' && c != '\n' && c != ',' && !c.is_ascii_digit())
                .then(|| parse_until(chars, "+-\n,").trim().to_owned())
        });

    parse_while(chars, |c| c.is_whitespace() && c != '\n');

    // If there is a non-whitespace character next, we continue parsing the offset, but if not, we
    // have an invalid coordinate, *unless* we parsed a "place" before, which is valid.
    let c = match chars.clone().next() {
        None | Some('\n') => {
            let place = place.ok_or(Error::InvalidPosition)?;

            return Ok(Some(Coordinate::Relative {
                place,
                offset: 0,
                pivot,
            }));
        }
        Some(c) => c,
    };

    let offset = if c == '+' || c == '-' || c.is_ascii_digit() {
        parse_int(chars)?
    } else {
        0
    };

    let position = place.map_or(Coordinate::Absolute(offset), |place| Coordinate::Relative {
        place,
        offset,
        pivot,
    });

    Ok(Some(position))
}

#[instrument(level = "debug", skip_all)]
fn parse_sketch(chars: &mut Chars<'_>) -> Result<Option<Sketch>, Error> {
    skip_whitespace(chars);

    if !chars.as_str().starts_with("sketch") {
        return Ok(None);
    }

    // sketch
    let _ = parse_word(chars);
    skip_whitespace(chars);

    let path = PathBuf::from(parse_line(chars));
    skip_whitespace(chars);

    let mut areas = vec![];
    while chars.clone().next() == Some('[') {
        let mut area = parse_area(chars)?;

        area.affordance = parse_line(chars).trim().to_owned();
        if area.affordance.is_empty() {
            return Err(Error::SketchAreaMissingAffordance);
        }

        areas.push(area);
        skip_whitespace(chars);
    }

    Ok(Some(Sketch { path, areas }))
}

#[instrument(level = "debug", skip_all)]
fn parse_area(chars: &mut Chars<'_>) -> Result<Area, Error> {
    if chars.next() != Some('[') {
        return Err(Error::ExpectedSketchArea);
    }

    let parse_coordinate =
        |chars: &mut Chars<'_>, expected_delimiter: Option<char>| -> Result<u32, Error> {
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
        affordance: String::new(),
    })
}

#[instrument(level = "trace", skip_all)]
fn parse_int<E: ToString, T: FromStr<Err = E>>(chars: &mut Chars<'_>) -> Result<T, Error> {
    let mut sign = '+';
    if let Some(c) = chars.clone().next() {
        if c == '+' || c == '-' {
            chars.next();
        }
        if c == '-' {
            sign = c;
        }
    }

    skip_whitespace(chars);
    let str = chars.as_str();
    while chars.clone().next().is_some_and(|c| c.is_ascii_digit()) {
        chars.next();
    }

    format!("{sign}{}", &str[..str.len() - chars.as_str().len()])
        .parse::<T>()
        .map_err(|e| Error::InvalidInteger(e.to_string()))
}

#[instrument(level = "debug", skip_all)]
fn parse_items(chars: &mut Chars<'_>) -> Result<Vec<Item>, Error> {
    skip_whitespace(chars);

    let mut items = vec![];

    while chars.clone().next().is_some() {
        if let Some(reference) = parse_reference(chars)? {
            items.push(Item::Reference(reference));
        } else if let Some(affordance) = parse_affordance(chars)? {
            items.push(Item::Affordance(affordance));
        } else {
            return Ok(items);
        }
    }

    Ok(items)
}

#[instrument(level = "debug", skip_all)]
fn parse_reference(chars: &mut Chars<'_>) -> Result<Option<Reference>, Error> {
    skip_whitespace(chars);

    // Ensure we're dealing with a (potentially nested) reference.
    let mut ch = chars.clone();
    let _ = parse_level(&mut ch);
    if !ch.as_str().starts_with("include") {
        return Ok(None);
    }

    let level = parse_level(chars);

    // include
    let _ = parse_word(chars);
    skip_whitespace(chars);

    let name = parse_line(chars).to_owned();
    if name.is_empty() {
        return Err(Error::MissingComponentReference);
    }

    Ok(Some(Reference { name, level }))
}

#[instrument(level = "debug", skip_all)]
fn parse_affordance(chars: &mut Chars<'_>) -> Result<Option<Affordance>, Error> {
    skip_whitespace(chars);

    let mut ch = chars.clone();
    drop(parse_comment(&mut ch));
    let str = ch.as_str();
    if str.is_empty()
        || str.starts_with("place")
        || str.starts_with("component")
        || str.starts_with("sketch")
        || str.starts_with("position")
    {
        return Ok(None);
    }

    let description = parse_comment(chars);

    let level = parse_level(chars);

    let name = parse_affordance_or_target_name(chars)?.to_owned();

    // If there is no name, it means we've reached the end of the board.
    //
    // We might still have captured a comment and one or more level characters, but we'll
    // ignore them for now, instead of (more correctly) raising a syntax error.
    if name.is_empty() {
        return Ok(None);
    }

    Ok(Some(Affordance {
        name,
        description,
        connections: parse_connections(chars)?,
        level,
    }))
}

#[instrument(level = "trace", skip_all)]
fn parse_connections(chars: &mut Chars<'_>) -> Result<Vec<Connection>, Error> {
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

#[instrument(level = "trace", skip_all)]
fn parse_level(chars: &mut Chars<'_>) -> usize {
    // Don't do any implicit trimming, the first character should be a "level" character.
    if !chars.as_str().starts_with('>') {
        return 0;
    }

    // Parse one or more levels, and allow spaces between and after `>`.
    let str = parse_while(chars, |c| c == '>' || (c.is_whitespace() && c != '\n'));

    str.matches('>').count()
}

#[instrument(level = "trace", skip_all)]
fn parse_affordance_or_target_name<'a>(chars: &'a mut Chars<'_>) -> Result<&'a str, Error> {
    let str = chars.as_str();

    if let Some('"') = chars.clone().next() {
        return parse_quoted_string(chars);
    }

    while chars.clone().next().is_some_and(|c| c != '\n') {
        if chars.as_str().starts_with("->") {
            break;
        }

        chars.next();
    }

    Ok(str[..str.len() - chars.as_str().len()].trim())
}

#[instrument(level = "trace", skip_all)]
fn parse_connection_description(chars: &mut Chars<'_>) -> Result<String, Error> {
    if chars.next() != Some('(') {
        return Err(Error::ExpectedConnectionDescription);
    }

    let start = chars.as_str();
    let desc = if let Some('"') = chars.clone().next() {
        parse_quoted_string(chars)?.to_owned()
    } else {
        while chars.clone().next().is_some_and(|c| c != '\n' && c != ')') {
            chars.next();
        }

        let end = chars.as_str();
        start[..start.len() - end.len()].to_owned()
    };

    if chars.next() != Some(')') {
        return Err(Error::UnterminatedConnectionDescription);
    }

    Ok(desc)
}

#[instrument(level = "trace", skip_all)]
fn parse_quoted_string<'a>(chars: &'a mut Chars<'_>) -> Result<&'a str, Error> {
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

#[instrument(level = "trace", skip_all)]
fn parse_while<'a>(chars: &'a mut Chars<'_>, fun: impl Fn(char) -> bool) -> &'a str {
    let str = chars.as_str();

    while chars.clone().next().is_some_and(&fun) {
        chars.next();
    }

    &str[..str.len() - chars.as_str().len()]
}

#[instrument(level = "trace", skip_all)]
fn parse_until<'a>(chars: &'a mut Chars<'_>, until: &str) -> &'a str {
    let str = chars.as_str();

    while chars.clone().next().is_some_and(|c| !until.contains(c)) {
        chars.next();
    }

    &str[..str.len() - chars.as_str().len()]
}

#[instrument(level = "trace", skip_all)]
fn parse_word<'a>(chars: &'a mut Chars<'_>) -> &'a str {
    let str = chars.as_str();

    while chars.clone().next().is_some_and(|c| !c.is_whitespace()) {
        chars.next();
    }

    &str[..str.len() - chars.as_str().len()]
}

#[instrument(level = "trace", skip_all)]
fn parse_line<'a>(chars: &'a mut Chars<'_>) -> &'a str {
    let str = chars.as_str();

    while chars.clone().next().is_some_and(|c| c != '\n') {
        chars.next();
    }

    &str[..str.len() - chars.as_str().len()]
}

#[instrument(level = "trace", skip_all)]
fn skip_whitespace(chars: &mut Chars<'_>) {
    while chars.clone().next().is_some_and(char::is_whitespace) {
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

    #[error("missing position coordinate")]
    MissingCoordinate,

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

    #[error("sketch area must reference an affordance")]
    SketchAreaMissingAffordance,

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

    #[error("invalid position")]
    InvalidPosition,

    #[error("invalid coordinate pivot")]
    InvalidCoordinatePivot,

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
            indoc! {"
                place Home
            "},
            indoc! {"
                place Registration
                  include Header

                  Username
                  Password
                  Sign Up -> (success) Home
                          -> (failure) Support

                  sketch sketches/registration.png
                    [50,20 110,40] Sign Up

                place Support
                  include Header

                  Error Message
                  Try Again -> Registration

                  position > Registration
                  sketch sketches/registration.png
                    [50,20 110,40] Try Again

                place Home
                  include Header

                  Dashboard

                  position 0, ^ Registration - 12
                  sketch sketches/home.png

                component Header
                  Logo
                  Contact
            "},
            indoc! {r#"
                place invoice
                  Turn on autopay -> Set up autopay -> Foo bar -> (test) test 2
                place two
                place three and more!
                  "free -> form!" -> Not -> "(test)"
                  another one!
                  sketch foo/bar.png
                    [0,0 10,10] free -> form!
                    [20,20 30,30] another one!
                place four!
            "#},
        ];

        for case in test_cases {
            insta::assert_debug_snapshot!(parse(case));
        }
    }

    #[test]
    fn test_parse_level() {
        let test_cases = vec![
            indoc! {"
                place NoLevel
                  No Level
            "},
            indoc! {"
                place OneLevel
                  > One Level
            "},
            indoc! {"
                place MultipleLevels
                  > One Level
                  > Two Level
                  > Three Level
            "},
            indoc! {"
                place NestedLevels
                  > One Level
                  >> Two Level
                  >> Three Level
                  >> > Four Level
            "},
            indoc! {"
                component MixedLevels
                  > One Level
                  >> Two Level
                  >> > Three Level
                  > Four Level
                  Five Level
            "},
        ];

        for case in test_cases {
            insta::assert_debug_snapshot!(parse(case));
        }
    }

    #[test]
    fn test_parse_comment() {
        let test_cases = vec![
            indoc! {"
                /// A comment as a description for the place.
                place PlaceComment
            "},
            indoc! {"
                place AffordanceComment
                  Not Here
                  /// An affordance comment.
                  But Here
                  And No Longer Here
            "},
            indoc! {"
                /// Both a place comment,
                place MultipleComments
                  /// and
                  One
                  /// affordance
                  >> Two
                  /// comments!
                  >>> Three
            "},
            indoc! {"
                /// Multi-level
                /// comments are supported.
                component MultiLineComment
                  /// For components,
                  /// and affordances.
                  Affordance
            "},
            indoc! {"
                ///Starting whitespace is
                /// optional.
                ///   and more than one whitespace
                ///  is preserved.
                ///   As is trailing whitespace  
                place WhiteSpace
                  ///  > Here as well < 
                  Affordance
            "},
            indoc! {"
                /// Comments for multiple places.
                place MultiplePlaces
                  /// And affordances.
                  Affordance

                /// Also works!
                place Works
                  /// And affordances.
                  Affordance
            "},
            indoc! {"
                // Non-description comments are also supported, but stripped.
                place MultiplePlaces
                  // Everywhere.
                  Affordance
                  // Always.

                // Forever.
                place Works
                  /// Unless three `/`'s are used!
                  Affordance
            "},
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
            ("()", Ok(String::new())),
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

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_parse_position() {
        let test_cases = vec![
            (
                r#"position > "0,0",0"#,
                Ok(Some(Position {
                    x: Coordinate::Relative {
                        place: "0,0".to_owned(),
                        offset: 0,
                        pivot: Pivot::Right,
                    },
                    y: Coordinate::Absolute(0),
                })),
            ),
            (
                r#"position "foo",",bar""#,
                Ok(Some(Position {
                    x: Coordinate::Relative {
                        place: "foo".to_owned(),
                        offset: 0,
                        pivot: Pivot::Center,
                    },
                    y: Coordinate::Relative {
                        place: ",bar".to_owned(),
                        offset: 0,
                        pivot: Pivot::Center,
                    },
                })),
            ),
            (
                "position < foo bar - 10,^bar baz | qux ! + 12",
                Ok(Some(Position {
                    x: Coordinate::Relative {
                        place: "foo bar".to_owned(),
                        offset: -10,
                        pivot: Pivot::Left,
                    },
                    y: Coordinate::Relative {
                        place: "bar baz | qux !".to_owned(),
                        offset: 12,
                        pivot: Pivot::Top,
                    },
                })),
            ),
            ("position _ foo,^bar", Err(Error::InvalidCoordinatePivot)),
            (
                "position -10,23",
                Ok(Some(Position {
                    x: Coordinate::Absolute(-10),
                    y: Coordinate::Absolute(23),
                })),
            ),
            (
                "position > foo + 10, 0",
                Ok(Some(Position {
                    x: Coordinate::Relative {
                        place: "foo".to_owned(),
                        offset: 10,
                        pivot: Pivot::Right,
                    },
                    y: Coordinate::Absolute(0),
                })),
            ),
            (
                "position foo-10,^foo+20",
                Ok(Some(Position {
                    x: Coordinate::Relative {
                        place: "foo".to_owned(),
                        offset: -10,
                        pivot: Pivot::Center,
                    },
                    y: Coordinate::Relative {
                        place: "foo".to_owned(),
                        offset: 20,
                        pivot: Pivot::Top,
                    },
                })),
            ),
            (
                "position ^foo",
                Ok(Some(Position {
                    x: Coordinate::Relative {
                        place: "foo".to_owned(),
                        offset: 0,
                        pivot: Pivot::Center,
                    },
                    y: Coordinate::Relative {
                        place: "foo".to_owned(),
                        offset: 0,
                        pivot: Pivot::Top,
                    },
                })),
            ),
            (
                r#"position >"foo+" + 12"#,
                Ok(Some(Position {
                    x: Coordinate::Relative {
                        place: "foo+".to_owned(),
                        offset: 12,
                        pivot: Pivot::Right,
                    },
                    y: Coordinate::Relative {
                        place: "foo+".to_owned(),
                        offset: 0,
                        pivot: Pivot::Center,
                    },
                })),
            ),
        ];

        for (input, expected) in test_cases {
            let mut chars = input.chars();
            let result = parse_position(&mut chars);
            assert_eq!(result, expected);
        }
    }
}
