use proc_macro2::Span;
use std::{collections::HashMap, fmt::Write, path::PathBuf};
use syn::{
    self,
    parse::{Parse, ParseStream},
    Ident, Token, Type,
};

#[derive(Debug)]
pub struct GraphqlSchemaFromFileInput {
    pub schema_path: PathBuf,
    pub error_type: Type,
    pub context_type: Type,
}

impl Parse for GraphqlSchemaFromFileInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file = input.parse::<syn::LitStr>()?.value();
        let cargo_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Env var `CARGO_MANIFEST_DIR` was missing");
        let pwd = PathBuf::from(cargo_dir);
        let schema_path = pwd.join(file);

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let mut configs = input
            .parse_terminated::<_, Token![,]>(TypeConfig::parse)?
            .into_pairs()
            .map(|pair| {
                let config = pair.into_value();
                (
                    config.ident.to_string(),
                    (config.type_, config.ident.span()),
                )
            })
            .collect::<HashMap<String, (Type, Span)>>();

        let error_type = configs
            .remove("error_type")
            .map(|(t, _)| t)
            .unwrap_or_else(default_error_type);

        let context_type = configs
            .remove("context_type")
            .map(|(t, _)| t)
            .unwrap_or_else(default_context_type);

        #[allow(clippy::never_loop)]
        for (name, (_, span)) in configs {
            let mut msg = String::new();
            writeln!(msg, "Unknown `graphql_schema_from_file` config `{}`", name).unwrap();
            writeln!(msg, "Supported configs are `error_type` and `context_type`").unwrap();
            return Err(syn::parse::Error::new(span, msg));
        }

        Ok(GraphqlSchemaFromFileInput {
            schema_path,
            error_type,
            context_type,
        })
    }
}

pub fn default_error_type() -> Type {
    syn::parse_str("juniper::FieldError").expect("Failed to parse default error type")
}

pub fn default_context_type() -> Type {
    syn::parse_str("Context").expect("Failed to parse default context type")
}

struct TypeConfig {
    ident: Ident,
    type_: Type,
}

impl Parse for TypeConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        input.parse::<Token![:]>()?;
        let type_ = input.parse::<Type>()?;
        Ok(TypeConfig { ident, type_ })
    }
}
