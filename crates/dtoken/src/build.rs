use std::{collections::HashMap, path::Path};

use crate::error::{BuildError, Error};
use crate::parser::{
    group::Group,
    token::Value,
    types::{DesignTokens, TokenOrGroup},
};
use crate::types::alias::Alias;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use tinyjson::JsonValue;

pub fn build(path: impl AsRef<str>) -> Result<(), BuildError> {
    write(&parse_content(&read_file(path)?)?)
}

pub fn build_merge(paths: &[impl AsRef<str>]) -> Result<(), BuildError> {
    let map = parse_content_merge(paths.iter().map(read_file).collect::<Result<Vec<_>, _>>()?)?;

    write(&map)
}

fn read_file(path: impl AsRef<str>) -> Result<String, BuildError> {
    std::fs::read_to_string(path.as_ref()).map_err(BuildError::Read)
}

fn parse_content(content: &str) -> Result<HashMap<String, JsonValue>, BuildError> {
    #[cfg(all(feature = "ason", feature = "toml"))]
    eprintln!(
        "Warning: any two of `ason`, `toml` or `jsonc` features are enabled. Using `json` parser."
    );

    #[cfg(all(feature = "ason", not(any(feature = "toml", feature = "jsonc"))))]
    {
        let json: ason::ast::AsonNode = ason::parse_from_str(content)?;
        return ason_node_to_json_value(json);
    }

    #[cfg(all(feature = "toml", not(any(feature = "ason", feature = "jsonc"))))]
    {
        let value = toml_span::parse(content)?.take();
        return toml_value_to_json_value(value);
    }

    #[cfg(all(feature = "jsonc", not(any(feature = "ason", feature = "toml"))))]
    {
        let opts = jsonc_parser::ParseOptions::default();
        let value = jsonc_parser::parse_to_value(content, &opts)?;
        jsonc_value_to_json_value(value.ok_or(BuildError::Parse(Error::ExpectedObject))?)
    }

    #[cfg(any(
        not(any(feature = "ason", feature = "toml", feature = "jsonc")),
        all(feature = "ason", feature = "toml", feature = "jsonc")
    ))]
    return content
        .parse::<JsonValue>()?
        .get()
        .cloned()
        .ok_or(BuildError::Parse(Error::ExpectedObject));
}

fn parse_content_merge(contents: Vec<String>) -> Result<HashMap<String, JsonValue>, BuildError> {
    let map = contents
        .into_iter()
        .map(|s| parse_content(&s))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .fold(HashMap::new(), |mut acc, map| {
            deep_merge(&mut acc, map);
            acc
        });

    Ok(map)
}

fn write(map: &HashMap<String, JsonValue>) -> Result<(), BuildError> {
    let tokens = DesignTokens::from_map(map)?;
    let code = generate(&tokens);

    let output = Path::new(&std::env::var("OUT_DIR")?).join("design_tokens.rs");

    std::fs::write(&output, code.to_string()).map_err(BuildError::Write)?;
    rustfmt(&output)?;

    Ok(())
}

fn deep_merge(target: &mut HashMap<String, JsonValue>, source: HashMap<String, JsonValue>) {
    for (key, source_value) in source {
        match target.get_mut(&key) {
            Some(target_value) => {
                // If both values are objects, merge them recursively
                if target_value.is_object() && source_value.is_object() {
                    let mut new_target = target_value
                        .get::<HashMap<_, _>>()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();

                    let source_converted = source_value
                        .get::<HashMap<_, _>>()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();

                    deep_merge(&mut new_target, source_converted);
                    *target_value = JsonValue::Object(new_target);
                } else {
                    // For non-object values, source overwrites target
                    *target_value = source_value.clone();
                }
            }
            None => {
                // If key doesn't exist in target, insert the source value
                target.insert(key, source_value);
            }
        }
    }
}

#[cfg(all(feature = "toml", not(any(feature = "ason", feature = "jsonc"))))]
fn toml_value_to_json_value(
    value: toml_span::value::ValueInner<'_>,
) -> Result<HashMap<String, JsonValue>, BuildError> {
    use toml_span::value::ValueInner;

    match value {
        ValueInner::Table(v) => {
            let mut map = HashMap::new();
            for (key, mut value) in v {
                map.insert(key.name.to_string(), convert_value(value.take())?);
            }
            Ok(map)
        }
        _ => Err(BuildError::Parse(Error::ExpectedObject)),
    }
}

#[cfg(all(feature = "toml", not(any(feature = "ason", feature = "jsonc"))))]
fn convert_value(value: toml_span::value::ValueInner<'_>) -> Result<JsonValue, BuildError> {
    use toml_span::value::ValueInner;

    match value {
        ValueInner::String(v) => Ok(JsonValue::String(v.to_string())),
        #[allow(clippy::cast_precision_loss)]
        ValueInner::Integer(v) => Ok(JsonValue::Number(v as f64)),
        ValueInner::Float(v) => Ok(JsonValue::Number(v)),
        ValueInner::Boolean(v) => Ok(JsonValue::Boolean(v)),
        ValueInner::Array(v) => {
            let mut arr = Vec::new();
            for mut item in v {
                arr.push(convert_value(item.take())?);
            }
            Ok(JsonValue::Array(arr))
        }
        ValueInner::Table(v) => {
            let mut map = HashMap::new();
            for (key, mut value) in v {
                map.insert(key.name.to_string(), convert_value(value.take())?);
            }
            Ok(JsonValue::Object(map))
        }
    }
}

#[cfg(all(feature = "ason", not(any(feature = "toml", feature = "jsonc"))))]
pub fn ason_node_to_json_value(
    node: ason::ast::AsonNode,
) -> Result<HashMap<String, JsonValue>, BuildError> {
    use ason::ast::AsonNode;

    match node {
        AsonNode::Object(v) => {
            let mut map = HashMap::new();
            for pair in v {
                map.insert(pair.key, convert_node(*pair.value)?);
            }
            Ok(map)
        }
        AsonNode::Map(v) => {
            let mut map = HashMap::new();
            for pair in v {
                match convert_node(*pair.name)? {
                    JsonValue::String(key) => {
                        map.insert(key, convert_node(*pair.value)?);
                    }
                    _ => return Err(BuildError::Parse(Error::ExpectedString)),
                }
            }
            Ok(map)
        }
        _ => Err(BuildError::Parse(Error::ExpectedObject)),
    }
}

#[cfg(all(feature = "ason", not(any(feature = "toml", feature = "jsonc"))))]
fn convert_node(node: ason::ast::AsonNode) -> Result<JsonValue, BuildError> {
    use ason::ast::AsonNode;

    match node {
        AsonNode::Number(v) => Ok(JsonValue::Number(convert_number(v))),
        AsonNode::Boolean(v) => Ok(JsonValue::Boolean(v)),
        AsonNode::String(v) => Ok(JsonValue::String(v)),
        AsonNode::List(v) => {
            let mut arr = Vec::new();
            for item in v {
                arr.push(convert_node(item)?);
            }
            Ok(JsonValue::Array(arr))
        }
        AsonNode::Object(v) => {
            let mut map = HashMap::new();
            for pair in v {
                map.insert(pair.key, convert_node(*pair.value)?);
            }
            Ok(JsonValue::Object(map))
        }
        AsonNode::Map(v) => {
            let mut map = HashMap::new();
            for pair in v {
                match convert_node(*pair.name)? {
                    JsonValue::String(key) => {
                        map.insert(key, convert_node(*pair.value)?);
                    }
                    _ => return Err(BuildError::Parse(Error::ExpectedString)),
                }
            }
            Ok(JsonValue::Object(map))
        }

        _ => Err(BuildError::Parse(Error::UnexpectedType)),
    }
}

#[cfg(all(feature = "ason", not(any(feature = "toml", feature = "jsonc"))))]
fn convert_number(num: ason::ast::Number) -> f64 {
    use ason::ast::Number;

    match num {
        Number::I8(v) => v as f64,
        Number::U8(v) => v as f64,
        Number::I16(v) => v as f64,
        Number::U16(v) => v as f64,
        Number::I32(v) => v as f64,
        Number::U32(v) => v as f64,
        #[allow(clippy::cast_precision_loss)]
        Number::I64(v) => v as f64,
        #[allow(clippy::cast_precision_loss)]
        Number::U64(v) => v as f64,
        Number::F32(v) => v as f64,
        Number::F64(v) => v,
    }
}

#[cfg(all(feature = "jsonc", not(any(feature = "ason", feature = "toml"))))]
fn jsonc_value_to_json_value(
    value: jsonc_parser::JsonValue<'_>,
) -> Result<HashMap<String, JsonValue>, BuildError> {
    match value {
        jsonc_parser::JsonValue::Object(v) => {
            let mut map = HashMap::new();
            for (key, value) in v {
                map.insert(key, convert_jsonc_value(value)?);
            }
            Ok(map)
        }

        _ => Err(BuildError::Parse(Error::ExpectedObject)),
    }
}

#[cfg(all(feature = "jsonc", not(any(feature = "ason", feature = "toml"))))]
fn convert_jsonc_value(value: jsonc_parser::JsonValue<'_>) -> Result<JsonValue, BuildError> {
    match value {
        jsonc_parser::JsonValue::String(v) => Ok(JsonValue::String(v.into_owned())),
        jsonc_parser::JsonValue::Number(v) => convert_number(v),
        jsonc_parser::JsonValue::Boolean(v) => Ok(JsonValue::Boolean(v)),
        jsonc_parser::JsonValue::Object(v) => {
            let mut map = HashMap::new();
            for (key, value) in v {
                map.insert(key, convert_jsonc_value(value)?);
            }
            Ok(JsonValue::Object(map))
        }
        jsonc_parser::JsonValue::Array(v) => {
            let mut arr = Vec::new();
            for item in v {
                arr.push(convert_jsonc_value(item)?);
            }
            Ok(JsonValue::Array(arr))
        }
        jsonc_parser::JsonValue::Null => Ok(JsonValue::Null),
    }
}

#[cfg(all(feature = "jsonc", not(any(feature = "ason", feature = "toml"))))]
fn convert_number(n: &str) -> Result<JsonValue, BuildError> {
    if let Ok(num) = n.parse::<i64>() {
        #[allow(clippy::cast_precision_loss)]
        return Ok(JsonValue::Number(num as f64));
    }

    if let Ok(num) = n.parse::<f64>() {
        return Ok(JsonValue::Number(num));
    }

    Err(BuildError::Parse(Error::ExpectedNumber))
}

fn generate(tokens: &DesignTokens) -> TokenStream {
    Generator::new(tokens).generate()
}

struct Generator {
    root: Group,
}

impl Generator {
    fn new(tokens: &DesignTokens) -> Self {
        let root = Group {
            items: tokens.items.clone(),
            description: Some("Root-level Design Tokens type".to_owned()),
            default_type: None,
            extensions: HashMap::new(),
        };

        Self { root }
    }

    fn generate(&self) -> TokenStream {
        let module = self.module_impl("DesignTokens", &self.root);
        let instance = self.group_instance("DesignTokens", &self.root, vec![]);

        quote! {
            #[allow(clippy::allow_attributes, clippy::too_many_lines)]
            pub fn design_tokens() -> design_tokens::DesignTokens {
                #instance
            }

            #module
        }
    }

    fn group_instance(&self, item: &str, group: &Group, mut parents: Vec<Ident>) -> TokenStream {
        let module_name = Ident::new(&item.to_case(Case::Snake), Span::call_site());
        let group_name = Ident::new(&item.to_case(Case::Pascal), Span::call_site());
        parents.push(module_name.clone());

        let mut items: Vec<_> = group.items.iter().collect();
        items.sort_by_key(|(k, _)| k.to_owned());

        let mut fields = vec![];
        let mut values = vec![];
        for (name, token_or_group) in &items {
            let (field, value) = self.field_instance(name, token_or_group, parents.clone());

            fields.push(field);
            values.push(value);
        }

        quote! {
            #(#parents::)* #group_name {
                #( #fields: #values,)*
            }
        }
    }

    fn field_instance(
        &self,
        field: &str,
        kind: &TokenOrGroup,
        parents: Vec<Ident>,
    ) -> (Ident, TokenStream) {
        let key = self.field_ident(field);
        let value = match kind {
            TokenOrGroup::Token(token) => self.token_value(&token.value),
            TokenOrGroup::Group(group) => self.group_instance(field, group, parents),
        };

        (key, value)
    }

    fn module_impl(&self, item: &str, group: &Group) -> TokenStream {
        let module = Ident::new(&item.to_case(Case::Snake), Span::call_site());
        let group = self.group_impl(item, group);

        quote! {
            #[allow(clippy::allow_attributes, clippy::module_inception)]
            pub mod #module {
                #group
            }
        }
    }

    fn group_impl(&self, item: &str, group: &Group) -> TokenStream {
        let group_name = Ident::new(&item.to_case(Case::Pascal), Span::call_site());
        let description = group.description.clone().unwrap_or_default();

        let mut items: Vec<_> = group.items.iter().collect();
        items.sort_by_key(|(k, _)| k.to_owned());

        let mut nested = vec![];
        for (name, group_item) in &items {
            let group = self.token_or_group_impl(name, group_item);
            nested.push(group);
        }

        let mut fields = vec![];
        let mut types = vec![];
        let mut descs = vec![];
        for (name, token_or_group) in &items {
            let (field, kind) = self.struct_field(name, token_or_group);
            let desc = token_or_group.description().unwrap_or_default();

            fields.push(field);
            types.push(kind);
            descs.push(if desc.is_empty() {
                quote! {}
            } else {
                quote! { #[doc = #desc] }
            });
        }

        let desc = if description.is_empty() {
            quote! {}
        } else {
            quote! { #[doc = #description] }
        };

        let bevy_reflect = if cfg!(feature = "bevy") {
            quote! { #[derive(bevy_reflect::Reflect)] }
        } else {
            quote! {}
        };

        quote! {
            #desc
            #bevy_reflect
            #[derive(Debug)]
            pub struct #group_name {
                #(
                #descs
                pub #fields: #types,
                )*
            }

            #(#nested)*
        }
    }

    fn token_or_group_impl(&self, item: &str, token_or_group: &TokenOrGroup) -> TokenStream {
        match token_or_group {
            TokenOrGroup::Token(_) => quote! {},
            TokenOrGroup::Group(group) => self.module_impl(item, group),
        }
    }

    fn struct_field(&self, field: &str, kind: &TokenOrGroup) -> (Ident, TokenStream) {
        let key = self.field_ident(field);
        let value = match kind {
            TokenOrGroup::Token(token) => self.token_kind(&token.value),
            TokenOrGroup::Group(_) => {
                let module = Ident::new(&field.to_case(Case::Snake), Span::call_site());
                let tail = Ident::new(&field.to_case(Case::Pascal), Span::call_site());
                quote! { #module :: #tail }
            }
        };

        (key, value)
    }

    #[allow(clippy::unused_self)]
    fn field_ident(&self, field: &str) -> Ident {
        let key = if field.starts_with('_') {
            format!("_{}", field.to_case(Case::Snake))
        } else {
            field.to_case(Case::Snake)
        };
        Ident::new(&key, Span::call_site())
    }

    fn alias_type(&self, alias: &Alias) -> Result<TokenStream, String> {
        let mut reference = &TokenOrGroup::Group(self.root.clone());

        for key in &alias.path_segments {
            reference = match reference {
                TokenOrGroup::Token(_) => {
                    return Err(format!(
                        "alias path segment {key} points to value, but group was expected."
                    ));
                }
                TokenOrGroup::Group(group) => match group.items.get(key) {
                    Some(token_or_group) => token_or_group,
                    None => {
                        return Err(format!(
                            "alias target missing: {{{}}}",
                            alias.path_segments.join(".")
                        ));
                    }
                },
            };
        }

        match reference {
            TokenOrGroup::Token(token) => Ok(self.token_kind(&token.value)),
            TokenOrGroup::Group(_) => Err(format!(
                "alias {{{}}} must point to a value, but instead points to a group.",
                alias.path_segments.join(".")
            )),
        }
    }

    fn token_kind(&self, value: &Value) -> TokenStream {
        let kind = match value {
            Value::Color(_) => "Color",
            Value::Dimension(_) => "Dimension",
            Value::FontFamily(_) => "FontFamily",
            Value::FontWeight(_) => "FontWeight",
            Value::Duration(_) => "Duration",
            Value::CubicBezier(_) => "CubicBezier",
            Value::Number(_) => "Number",
            Value::StrokeStyle(_) => "StrokeStyle",
            Value::Border(_) => "Border",
            Value::Transition(_) => "Transition",
            Value::Shadow(_) => "Shadow",
            Value::Gradient(_) => "Gradient",
            Value::Typography(_) => "Typography",
            Value::Alias(alias) => return self.alias_type(alias).unwrap(),
        };

        let module = Ident::new(&kind.to_case(Case::Snake), Span::call_site());
        let kind = Ident::new(kind, Span::call_site());

        quote! {
            dtoken::types::#module::#kind
        }
    }

    fn alias_value(&self, alias: &Alias) -> Result<TokenStream, String> {
        let mut reference = &TokenOrGroup::Group(self.root.clone());

        for key in &alias.path_segments {
            reference = match reference {
                TokenOrGroup::Token(_) => {
                    return Err(format!(
                        "alias path segment {key} points to value, but group was expected."
                    ));
                }
                TokenOrGroup::Group(group) => match group.items.get(key) {
                    Some(token_or_group) => token_or_group,
                    None => {
                        return Err(format!(
                            "alias target missing: {{{}}}",
                            alias.path_segments.join(".")
                        ));
                    }
                },
            };
        }

        match reference {
            TokenOrGroup::Token(token) => Ok(self.token_value(&token.value)),
            TokenOrGroup::Group(_) => Err(format!(
                "alias {{{}}} must point to a value, but instead points to a group.",
                alias.path_segments.join(".")
            )),
        }
    }

    fn token_value(&self, value: &Value) -> TokenStream {
        match value {
            Value::Alias(alias) => self.alias_value(alias).unwrap(),
            Value::Border(v) => v.to_token_stream(),
            Value::Color(v) => v.to_token_stream(),
            Value::CubicBezier(v) => v.to_token_stream(),
            Value::Dimension(v) => v.to_token_stream(),
            Value::Duration(v) => v.to_token_stream(),
            Value::FontFamily(v) => v.to_token_stream(),
            Value::FontWeight(v) => v.to_token_stream(),
            Value::Gradient(v) => v.to_token_stream(),
            Value::Number(v) => v.to_token_stream(),
            Value::Shadow(v) => v.to_token_stream(),
            Value::StrokeStyle(v) => v.to_token_stream(),
            Value::Transition(v) => v.to_token_stream(),
            Value::Typography(v) => v.to_token_stream(),
        }
    }
}

/// Format a file with rustfmt
#[cfg(feature = "rustfmt")]
fn rustfmt(path: &Path) -> Result<(), BuildError> {
    use std::process::Command;

    Command::new(std::env::var("RUSTFMT").unwrap_or_else(|_| "rustfmt".to_string()))
        .args(["--emit", "files"])
        // .args(["--config", "format_strings=true,edition=2024,struct_lit_width=0,struct_lit_single_line=false,struct_variant_width=false"])
        .args(["--config", "format_strings=true"])
        .arg(path)
        .output()
        .map_err(BuildError::Fmt)?;

    Ok(())
}

#[cfg(not(feature = "rustfmt"))]
fn rustfmt(_path: &Path) -> Result<(), BuildError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use syn::File;
    use tinyjson::JsonValue;

    use super::*;

    #[cfg(any(
        not(any(feature = "ason", feature = "toml", feature = "jsonc")),
        all(feature = "ason", feature = "toml", feature = "jsonc")
    ))]
    #[test]
    fn test_json() {
        let test_cases = [indoc! {r#"
                {
                  "group name": {
                    "token name": {
                      "$value": 1234,
                      "$type": "number"
                    }
                  },
                  "alias name": {
                    "$value": "{group name.token name}"
                  }
                }
            "#}];

        for (i, case) in test_cases.iter().enumerate() {
            let map: HashMap<String, JsonValue> = parse_content(case).unwrap();
            let tokens = DesignTokens::from_map(&map).unwrap();

            let tokens = generate(&tokens);
            let abstract_file: File =
                syn::parse2(tokens.clone()).unwrap_or_else(|err| panic!("{err}:\n\n{tokens}"));
            let code = prettyplease::unparse(&abstract_file);

            insta::assert_snapshot!(format!("json case {i}"), code.to_string());
        }
    }

    #[cfg(all(feature = "toml", not(any(feature = "ason", feature = "jsonc"))))]
    #[test]
    fn test_toml() {
        let test_cases = [indoc! {r#"
                ["group name"]

                ["group name"."token name"]
                "$value" = 1234
                "$type" = "number"

                ["alias name"]
                "$value" = "{group name.token name}"
            "#}];

        for (i, case) in test_cases.iter().enumerate() {
            let map: HashMap<String, JsonValue> = parse_content(case).unwrap();
            let tokens = DesignTokens::from_map(&map).unwrap();

            let tokens = generate(&tokens);
            let abstract_file: File =
                syn::parse2(tokens.clone()).unwrap_or_else(|err| panic!("{err}:\n\n{tokens}"));
            let code = prettyplease::unparse(&abstract_file);

            insta::assert_snapshot!(format!("toml case {i}"), code.to_string());
        }
    }

    #[cfg(all(feature = "ason", not(any(feature = "toml", feature = "jsonc"))))]
    #[test]
    fn test_ason() {
        let test_cases = [indoc! {r#"
                [
                  "group name": [
                    "token name": [
                      "$value": 1234
                      "$type": "number"
                    ]
                  ]
                  "alias name": [
                    "$value": "{group name.token name}"
                  ]
                ]
            "#}];

        for (i, case) in test_cases.iter().enumerate() {
            let map: HashMap<String, JsonValue> = parse_content(case).unwrap();
            let tokens = DesignTokens::from_map(&map).unwrap();

            let tokens = generate(&tokens);
            let abstract_file: File =
                syn::parse2(tokens.clone()).unwrap_or_else(|err| panic!("{err}:\n\n{tokens}"));
            let code = prettyplease::unparse(&abstract_file);

            insta::assert_snapshot!(format!("ason case {i}"), code.to_string());
        }
    }

    #[cfg(all(feature = "jsonc", not(any(feature = "ason", feature = "toml"))))]
    #[test]
    fn test_jsonc() {
        let test_cases = [indoc! {r#"
                {
                  "group name": {
                    "token name": {
                      "$value": 1234,
                      "$type": "number",
                    },
                  },
                  // A comment
                  "alias name": { // Another comment
                    "$value": "{group name.token name}",
                  },
                }
            "#}];

        for (i, case) in test_cases.iter().enumerate() {
            let map: HashMap<String, JsonValue> = parse_content(case).unwrap();
            let tokens = DesignTokens::from_map(&map).unwrap();

            let tokens = generate(&tokens);
            let abstract_file: File =
                syn::parse2(tokens.clone()).unwrap_or_else(|err| panic!("{err}:\n\n{tokens}"));
            let code = prettyplease::unparse(&abstract_file);

            insta::assert_snapshot!(format!("json case {i}"), code.to_string());
        }
    }

    #[cfg(any(
        not(any(feature = "ason", feature = "toml", feature = "jsonc")),
        all(feature = "ason", feature = "toml", feature = "jsonc")
    ))]
    #[test]
    fn test_merged_content() {
        let contents = [
            indoc! {r#"
                {
                  "group name": {
                    "token name": {
                      "$value": 1234,
                      "$type": "number"
                    }
                  },
                  "alias name": {
                    "$value": "{group name.token name}"
                  }
                }
            "#},
            indoc! {r##"
                {
                  "group name": {
                    "token name": {
                      "$value": 5678,
                      "$type": "number"
                    }
                  },
                  "alias name": {
                    "$type": "color",
                    "$value": "#ff0000"
                  },
                  "new token": {
                    "$value": "{alias name}"
                  }
                }
            "##},
        ];

        let map = parse_content_merge(contents.iter().map(ToString::to_string).collect()).unwrap();
        let tokens = DesignTokens::from_map(&map).unwrap();

        let tokens = generate(&tokens);
        let abstract_file: File =
            syn::parse2(tokens.clone()).unwrap_or_else(|err| panic!("{err}:\n\n{tokens}"));
        let code = prettyplease::unparse(&abstract_file);

        insta::assert_snapshot!("merged content", code.to_string());
    }
}
