/// Represents a number. Numbers can be positive, negative and have fractions. Example uses for
/// number tokens are gradient stop positions or unitless line heights. The $type property MUST be
/// set to the string number. The value MUST be a JSON number value.
/// Example 23
///
/// ```json,ignore
/// {
///   "line-height-large": {
///     "$value": 2.3,
///     "$type": "number"
///   }
/// }
/// ```
///
/// See: <https://tr.designtokens.org/format/#number>.

/// See module docs.
#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub f64);
