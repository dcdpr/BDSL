use std::ops::Deref;

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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Number(pub f64);

impl Number {
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

impl Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
