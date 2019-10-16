//! See the docs for "juniper-from-schema" for more info about this.

#![deny(
    unused_imports,
    mutable_borrow_reservation_conflict,
    dead_code,
    unused_variables,
    unused_must_use
)]
#![recursion_limit = "128"]
#![doc(html_root_url = "https://docs.rs/juniper-from-schema-code-gen/0.4.1")]

extern crate proc_macro;
extern crate proc_macro2;

mod ast_pass;
mod nullable_type;
mod parse_input;
mod pretty_print;

use self::{
    ast_pass::{ast_data_pass::AstData, error::Error, CodeGenPass},
    parse_input::{default_context_type, default_error_type, GraphqlSchemaFromFileInput},
};
use graphql_parser::parse_schema;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{collections::BTreeSet, path::Path};
use syn::Type;

/// Read a GraphQL schema file and generate corresponding Juniper macro calls.
///
/// See [the crate level docs](index.html) for an example.
#[proc_macro]
pub fn graphql_schema_from_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = match syn::parse::<GraphqlSchemaFromFileInput>(input) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    match std::fs::read_to_string(&parsed.schema_path) {
        Ok(schema) => {
            let mut tokens = parse_and_gen_schema(&schema, parsed.error_type, parsed.context_type);
            include_literal_schema(&mut tokens, &parsed.schema_path);
            tokens
        }
        Err(err) => panic!("{}", err),
    }
}

// This should cause the Rust schema to be rebuild even if the user only changes the GraphQL schema
// file.
fn include_literal_schema(tokens: &mut proc_macro::TokenStream, schema_path: &Path) {
    let schema_path = syn::LitStr::new(
        schema_path
            .to_str()
            .expect("Invalid UTF-8 characters in file name"),
        Span::call_site(),
    );

    tokens.extend(proc_macro::TokenStream::from(quote! {
        const _: &str = std::include_str!(#schema_path);
    }));
}

/// Write your GraphQL schema directly in your Rust code.
///
/// This is mostly useful for testing. Prefer using [`graphql_schema_from_file`][] for larger
/// schemas.
///
/// [`graphql_schema_from_file`]: macro.graphql_schema_from_file.html
///
/// # Example
///
/// ```ignore
/// graphql_schema! {
///     schema {
///         query: Query
///     }
///
///     type Query {
///         helloWorld: String! @juniper(ownership: "owned")
///     }
/// }
///
/// pub struct Query;
///
/// impl QueryFields for Query {
///     fn field_hello_world(
///         &self,
///         executor: &Executor<'_, Context>,
///     ) -> FieldResult<String> {
///         Ok("Hello, World!".to_string())
///     }
/// }
/// ```
#[proc_macro]
pub fn graphql_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let schema = input.to_string();
    parse_and_gen_schema(&schema, default_error_type(), default_context_type())
}

fn parse_and_gen_schema(
    schema: &str,
    error_type: Type,
    context_type: Type,
) -> proc_macro::TokenStream {
    let doc = match parse_schema(&schema) {
        Ok(doc) => doc,
        Err(parse_error) => panic!("{}", parse_error),
    };

    let ast_data = match AstData::new_from_schema_and_doc(schema, &doc) {
        Ok(x) => x,
        Err(errors) => print_and_panic_if_errors(errors),
    };

    let output = CodeGenPass::new(schema, error_type, context_type, ast_data);

    match output.gen_juniper_code(&doc) {
        Ok(tokens) => {
            let out: proc_macro::TokenStream = tokens.into();

            if debugging_enabled() {
                self::pretty_print::code_gen_debug(out.to_string());
            }

            out
        }
        Err(errors) => print_and_panic_if_errors(errors),
    }
}

fn print_and_panic_if_errors<T>(errors: BTreeSet<Error>) -> T {
    let count = errors.len();

    let out = errors
        .into_iter()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n\n");

    if count == 1 {
        panic!("\n\n{}\n\naborting due to previous error\n", out)
    } else {
        panic!("\n\n{}\n\naborting due to {} errors\n", out, count)
    }
}

fn debugging_enabled() -> bool {
    if let Ok(val) = std::env::var("JUNIPER_FROM_SCHEMA_DEBUG") {
        if &val == "1" {
            return true;
        }
    }

    false
}
