use std::ops::Deref;

use tinyjson::JsonValue;

use crate::error::Error;

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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Number(pub f64);

impl Number {
    #[must_use]
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

impl TryFrom<&JsonValue> for Number {
    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        value
            .get::<f64>()
            .ok_or(Error::ExpectedNumber)
            .map(|v| Number(*v))
    }
}

#[cfg(feature = "build")]
impl quote::ToTokens for Number {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let num = self.0;
        tokens.extend(quote::quote! { dtoken::types::number::Number(#num) });
    }
}
