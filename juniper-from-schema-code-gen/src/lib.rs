//! See the docs for "juniper-from-schema" for more info about this.

#![deny(
    unused_imports,
    mutable_borrow_reservation_conflict,
    dead_code,
    unused_variables,
    unused_must_use
)]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/juniper-from-schema-code-gen/0.5.2")]

mod ast_pass;

use ast_pass::{code_gen_pass::CodeGenPass, error, AstData};

use graphql_parser::parse_schema;
use proc_macro2::Span;
use quote::quote;
use std::{
    fmt,
    path::{Path, PathBuf},
};

const DATE_TIME_SCALAR_NAME: &str = "DateTimeUtc";
const DATE_SCALAR_NAME: &str = "Date";
const UUID_SCALAR_NAME: &str = "Uuid";
const URL_SCALAR_NAME: &str = "Url";

#[derive(Debug)]
pub struct CodeGen {
    schema: SchemaLocation,
    context_type: syn::Type,
    error_type: syn::Type,
}

impl CodeGen {
    pub fn build_from_schema_file(path: PathBuf) -> CodeGenBuilder {
        CodeGenBuilder {
            schema: SchemaLocation::File(path),
            context_type: None,
            error_type: None,
        }
    }

    pub fn build_from_schema_literal(schema: String) -> CodeGenBuilder {
        CodeGenBuilder {
            schema: SchemaLocation::Literal(schema),
            context_type: None,
            error_type: None,
        }
    }

    pub fn generate_code(self) -> Result<proc_macro2::TokenStream, Error> {
        let (schema, schema_path) = match self.schema {
            SchemaLocation::File(path) => (
                std::fs::read_to_string(&path).map_err(Error::Io)?,
                Some(path),
            ),
            SchemaLocation::Literal(schema) => (schema, None),
        };

        let doc = match parse_schema(&schema) {
            Ok(doc) => doc,
            Err(parse_error) => return Err(Error::SchemaParseError(parse_error)),
        };

        let ast_data = match AstData::new_from_doc(&doc) {
            Ok(x) => x,
            Err(code_gen_errors) => {
                let errors = Error::CodeGenErrors {
                    errors: code_gen_errors.into_iter().collect(),
                    schema,
                };
                return Err(errors);
            }
        };

        let output = CodeGenPass::new(&schema, &self.error_type, &self.context_type, ast_data);

        match output.gen_juniper_code(&doc) {
            Ok(mut tokens) => {
                if debugging_enabled() {
                    eprintln!("{}", tokens);
                }

                if let Some(path) = schema_path {
                    include_literal_schema(&mut tokens, path.as_path());
                }

                Ok(tokens)
            }
            Err(code_gen_errors) => {
                let errors = Error::CodeGenErrors {
                    errors: code_gen_errors.into_iter().collect(),
                    schema,
                };
                Err(errors)
            }
        }
    }
}

// This should cause the Rust schema to be rebuild even if the user only changes the GraphQL schema
// file.
fn include_literal_schema(tokens: &mut proc_macro2::TokenStream, schema_path: &Path) {
    let schema_path = syn::LitStr::new(
        schema_path
            .to_str()
            .expect("Invalid UTF-8 characters in file name"),
        Span::call_site(),
    );

    tokens.extend(quote! {
        const _: &str = std::include_str!(#schema_path);
    });
}

#[derive(Debug)]
pub enum Error {
    SchemaParseError(graphql_parser::schema::ParseError),
    CodeGenErrors {
        errors: Vec<error::Error>,
        schema: String,
    },
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SchemaParseError(inner) => write!(f, "{}", inner),
            Error::CodeGenErrors { errors, schema } => {
                assert!(
                    !errors.is_empty(),
                    "`print_and_panic_if_errors` called without any errors"
                );

                let count = errors.len();

                let out = errors
                    .iter()
                    .map(|error| error.display(&schema).to_string())
                    .collect::<Vec<_>>()
                    .join("\n\n");

                if count == 1 {
                    write!(f, "\n\n{}\n\naborting due to previous error\n", out)
                } else {
                    write!(f, "\n\n{}\n\naborting due to {} errors\n", out, count)
                }
            }
            Error::Io(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct CodeGenBuilder {
    schema: SchemaLocation,
    context_type: Option<syn::Type>,
    error_type: Option<syn::Type>,
}

impl CodeGenBuilder {
    pub fn context_type(mut self, context_type: syn::Type) -> Self {
        self.context_type = Some(context_type);
        self
    }

    pub fn error_type(mut self, error_type: syn::Type) -> Self {
        self.error_type = Some(error_type);
        self
    }

    pub fn finish(self) -> CodeGen {
        CodeGen {
            schema: self.schema,
            context_type: self.context_type.unwrap_or_else(default_context_type),
            error_type: self.error_type.unwrap_or_else(default_error_type),
        }
    }
}

#[derive(Debug)]
enum SchemaLocation {
    File(PathBuf),
    Literal(String),
}

pub fn default_error_type() -> syn::Type {
    syn::parse_str("juniper::FieldError").expect("Failed to parse default error type")
}

pub fn default_context_type() -> syn::Type {
    syn::parse_str("Context").expect("Failed to parse default context type")
}

fn debugging_enabled() -> bool {
    std::env::var("JUNIPER_FROM_SCHEMA_DEBUG")
        .map(|val| val == "1")
        .unwrap_or(false)
}
