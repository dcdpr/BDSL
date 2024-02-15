//! Code generation
//!
//! # Generated code
//! The generated code consists of a single enum (called by default `Lang`),
//! which expose pub(crate)lic method for each of the translation keys. These
//! methods returns a `&'static str` where possible, otherwise a `String`.
//!
//! # Usage
//! The code generator is contained within the [`CodeGenerator`] struct.
//! Calling [`generate`](CodeGenerator::generate) will produce a [TokenStream]
//! with the generated code. Internal methods used to generate the output are not exposed.

use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{
    builder::{TokensConfig, TokensId},
    parser::{FormattedKey, SimpleKey, TokensData, TranslationKey},
};

/// Type storing state and configuration for the code generator
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CodeGenerator<'a> {
    keys: &'a HashMap<String, TranslationKey>,
    groups: Vec<&'a TokensId>,
    name: Ident,
}

impl<'a> CodeGenerator<'a> {
    /// Initialize a new [`CodeGenerator`]
    pub(crate) fn new(data: &'a TokensData, config: &'a TokensConfig) -> Self {
        let name = Ident::new(&config.name, Span::call_site());

        CodeGenerator {
            keys: &data.keys,
            groups: config.groups(),
            name,
        }
    }

    /// Generate code as a [`TokenStream`]
    pub(crate) fn generate(&self) -> TokenStream {
        // Transform as PascalCase strings
        let languages: Vec<_> = self
            .groups
            .iter()
            .map(|lang| lang.value().to_case(Case::Pascal))
            .collect();

        let name = &self.name;
        let fields = languages
            .iter()
            .map(|lang| Ident::new(lang, Span::call_site()));

        let language_impl = self.impl_language();
        let methods = self.keys.iter().map(|(key, value)| match value {
            TranslationKey::Simple(inner) => self.method_simple(key, inner),
            TranslationKey::Formatted(inner) => self.method_formatted(key, inner),
        });

        quote! {
            /// Language type generated by the [rosetta](https://github.com/baptiste0928/rosetta) i18n library.
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
            pub enum #name {
                #(#fields),*
            }

            impl #name {
                #(#methods)*
            }

            #language_impl
        }
    }

    /// Generate method for [`TranslationKey::Simple`]
    fn method_simple(&self, key: &str, data: &SimpleKey) -> TokenStream {
        let name = Ident::new(&key.to_case(Case::Snake), Span::call_site());
        let fallback = &data.fallback;
        let arms = data
            .others
            .iter()
            .map(|(language, value)| self.match_arm_simple(language, value));

        quote! {
            #[allow(clippy::all)]
            pub fn #name(&self) -> &'static str {
                match self {
                    #(#arms,)*
                    _ => #fallback
                }
            }
        }
    }

    /// Generate match arm for [`TranslationKey::Simple`]
    fn match_arm_simple(&self, language: &TokensId, value: &str) -> TokenStream {
        let name = &self.name;
        let lang = Ident::new(&language.value().to_case(Case::Pascal), Span::call_site());

        quote! { #name::#lang => #value }
    }

    /// Generate method for [`TranslationKey::Formatted`]
    fn method_formatted(&self, key: &str, data: &FormattedKey) -> TokenStream {
        let name = Ident::new(&key.to_case(Case::Snake), Span::call_site());

        // Sort parameters alphabetically to have consistent ordering
        let mut sorted = Vec::from_iter(&data.parameters);
        sorted.sort_by_key(|s| s.to_lowercase());
        let params = sorted
            .iter()
            .map(|param| Ident::new(param, Span::call_site()))
            .map(|param| quote!(#param: impl ::std::fmt::Display));

        let arms = data
            .others
            .iter()
            .map(|(language, value)| self.match_arm_formatted(language, value, &data.parameters));
        let fallback = self.format_formatted(&data.fallback, &data.parameters);

        quote! {
            #[allow(clippy::all)]
            pub fn #name(&self, #(#params),*) -> ::std::string::String {
                match self {
                    #(#arms,)*
                    _ => #fallback
                }
            }
        }
    }

    /// Generate match arm for [`TranslationKey::Formatted`]
    fn match_arm_formatted(
        &self,
        language: &TokensId,
        value: &str,
        parameters: &HashSet<String>,
    ) -> TokenStream {
        let name = &self.name;
        let format_value = self.format_formatted(value, parameters);
        let lang = Ident::new(&language.value().to_case(Case::Pascal), Span::call_site());

        quote! { #name::#lang => #format_value }
    }

    /// Generate `format!` for [`TranslationKey::Formatted`]
    fn format_formatted(&self, value: &str, parameters: &HashSet<String>) -> TokenStream {
        let params = parameters
            .iter()
            .map(|param| Ident::new(param, Span::call_site()))
            .map(|param| quote!(#param = #param));

        quote!(format!(#value, #(#params),*))
    }

    /// Generate implementation for `rosetta_i18n::Language` trait.
    fn impl_language(&self) -> TokenStream {
        let name = &self.name;

        let language_id_idents = self.groups.iter().map(|lang| lang.value()).map(|lang| {
            (
                lang,
                Ident::new(&lang.to_case(Case::Pascal), Span::call_site()),
            )
        });

        let from_language_id_arms = language_id_idents
            .clone()
            .map(|(lang, ident)| quote!(#lang => ::core::option::Option::Some(Self::#ident)));

        let to_language_id_arms = language_id_idents
            .map(|(lang, ident)| quote!(Self::#ident => ::rosetta_i18n::LanguageId::new(#lang)));

        quote! {
            impl ::rosetta_i18n::Language for #name {
                fn from_language_id(language_id: &::rosetta_i18n::LanguageId) -> ::core::option::Option<Self> {
                    match language_id.value() {
                        #(#from_language_id_arms,)*
                        _ => ::core::option::Option::None
                    }
                }

                fn language_id(&self) -> ::rosetta_i18n::LanguageId {
                    match self {
                        #(#to_language_id_arms,)*
                    }
                }
            }
        }
    }
}