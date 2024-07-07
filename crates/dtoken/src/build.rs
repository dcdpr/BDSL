use std::{collections::HashMap, path::Path};

use crate::error::BuildError;
use crate::parser::{
    group::Group,
    token::Value,
    types::{DesignTokens, TokenOrGroup},
};
use crate::types::{
    alias::Alias, color::Color, dimension::Dimension, font_family::FontFamily, number::Number,
};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn build(path: impl AsRef<str>) -> Result<(), BuildError> {
    use tinyjson::JsonValue;

    let content = std::fs::read_to_string(path.as_ref())?;
    let json: JsonValue = content.parse()?;
    let map = json.get().ok_or(BuildError::Parse)?;

    let tokens = DesignTokens::from_map(map).ok_or(BuildError::Parse)?;
    let code = generate(tokens);

    let output = Path::new(&std::env::var("OUT_DIR")?).join("design_tokens.rs");

    std::fs::write(&output, code.to_string())?;
    rustfmt(&output)?;

    Ok(())
}

fn generate(tokens: DesignTokens) -> TokenStream {
    Generator::new(tokens).generate()
}

struct Generator {
    root: Group,
}

impl Generator {
    fn new(tokens: DesignTokens) -> Self {
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
        field: &String,
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
            #[allow(clippy::module_inception)]
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
            descs.push(desc);
        }

        quote! {
            #[doc = #description]
            #[derive(Debug)]
            pub struct #group_name {
                #(
                #[doc = #descs]
                pub #fields: #types,
                )*
            }

            #(#nested)*
        }
    }

    fn token_or_group_impl(&self, item: &str, token_or_group: &TokenOrGroup) -> TokenStream {
        match token_or_group {
            TokenOrGroup::Token(_) => return quote! {},
            TokenOrGroup::Group(group) => self.module_impl(item, group),
        }
    }

    fn struct_field(&self, field: &String, kind: &TokenOrGroup) -> (Ident, TokenStream) {
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
                        "alias path segment {} points to value, but group was expected.",
                        key
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
                        "alias path segment {} points to value, but group was expected.",
                        key
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
        let kind = self.token_kind(value);
        let value = match value {
            Value::Color(Color { r, g, b, a }) => quote! { { r: #r, g: #g, b: #b, a: #a } },
            Value::Number(Number(v)) => quote! { (#v) },
            Value::Alias(alias) => return self.alias_value(alias).unwrap(),
            Value::FontFamily(FontFamily { primary, fallbacks }) => {
                quote! { {
                    primary: #primary.to_owned(),
                    fallbacks: vec![#( #fallbacks.to_owned(),)*],
                } }
            }
            Value::Dimension(v) => match v {
                Dimension::Pixels(v) => {
                    quote! { ::Pixels(#v) }
                }
                Dimension::Rems(_) => todo!(),
            },
            v => todo!("{:?}", v),
            // Value::FontWeight(v) => quote! { #v },
            // Value::Duration(v) => quote! { #v },
            // Value::CubicBezier(v) => quote! { #v },
            // Value::StrokeStyle(v) => quote! { #v },
            // Value::Border(v) => quote! { #v },
            // Value::Transition(v) => quote! { #v },
            // Value::Shadow(v) => quote! { #v },
            // Value::Gradient(v) => quote! { #v },
            // Value::Typography(v) => quote! { #v },
        };

        quote! {
            #kind #value
        }
    }
}

/// Format a file with rustfmt
#[cfg(feature = "rustfmt")]
fn rustfmt(path: &Path) -> Result<(), BuildError> {
    use std::process::Command;

    Command::new(std::env::var("RUSTFMT").unwrap_or_else(|_| "rustfmt".to_string()))
        .args(&["--emit", "files"])
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

    #[test]
    fn test_examples() {
        let test_cases = vec![indoc! {r#"
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

        for case in test_cases {
            let value: JsonValue = case.parse().unwrap();
            let tokens = DesignTokens::from_map(value.get().unwrap()).unwrap();

            let tokens = generate(tokens);
            let abstract_file: File =
                syn::parse2(tokens.clone()).unwrap_or_else(|err| panic!("{err}:\n\n{}", tokens));
            let code = prettyplease::unparse(&abstract_file);

            insta::assert_snapshot!(code.to_string());
        }
    }
}
