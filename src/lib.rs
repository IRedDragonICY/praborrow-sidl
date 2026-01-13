mod error;
mod lexer;
mod parser;

use crate::lexer::Lexer;
use crate::parser::{Def, Parser};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use std::path::PathBuf;

/// Macro to include a .sidl file and generate Rust code.
#[proc_macro]
pub fn include_sidl(input: TokenStream) -> TokenStream {
    let input_path_str = input.to_string().trim_matches('"').to_string();

    // Resolve path relative to CARGO_MANIFEST_DIR
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let path = PathBuf::from(manifest_dir).join(&input_path_str);

    let sidl_content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to read SIDL file {:?}: {}", path, e),
            )
            .to_compile_error()
            .into();
        }
    };

    let lexer = Lexer::new(&sidl_content);
    let mut parser = match Parser::new(lexer) {
        Ok(p) => p,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("SIDL Parser Init Error: {}", e),
            )
            .to_compile_error()
            .into();
        }
    };

    let defs = match parser.parse() {
        Ok(d) => d,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("SIDL Parse Error: {}", e),
            )
            .to_compile_error()
            .into();
        }
    };

    let mut expanded_tokens = proc_macro2::TokenStream::new();

    for def in defs {
        match def {
            Def::Struct(s) => {
                let name = format_ident!("{}", s.name);
                let fields = s.fields.iter().map(|(n, t)| {
                    let field_name = format_ident!("{}", n);
                    let field_type = format_ident!("{}", t);
                    quote! { pub #field_name: #field_type, }
                });

                expanded_tokens.extend(quote! {
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Diplomat)]
                    pub struct #name {
                        #(#fields)*
                    }
                });
            }
            Def::Service(s) => {
                let name = format_ident!("{}", s.name);
                let methods = s.methods.iter().map(|m| {
                    let method_name = format_ident!("{}", m.name);
                    let arg_name = format_ident!("{}", m.arg_name);
                    let arg_type = format_ident!("{}", m.arg_type);
                    let ret_type = format_ident!("{}", m.ret_type);

                    quote! {
                        async fn #method_name(&self, #arg_name: #arg_type) -> #ret_type;
                    }
                });

                expanded_tokens.extend(quote! {
                    #[async_trait::async_trait]
                    pub trait #name {
                        #(#methods)*
                    }
                });
            }
        }
    }

    TokenStream::from(expanded_tokens)
}

/// Derive macro for the `Diplomat` trait.
#[proc_macro_derive(Diplomat)]
pub fn derive_diplomat(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl praborrow_diplomacy::Diplomat for #name {}
    };
    TokenStream::from(expanded)
}
