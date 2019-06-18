//! See the docs for "juniper-from-schema" for more info about this.

#![deny(unused_imports, dead_code, unused_variables, unused_must_use)]
#![recursion_limit = "128"]
#![doc(html_root_url = "https://docs.rs/juniper-from-schema-code-gen/0.3.0")]

extern crate proc_macro;
extern crate proc_macro2;

mod ast_pass;
mod nullable_type;
mod parse_input;
mod pretty_print;

use self::{
    ast_pass::{AstData, CodeGenPass},
    parse_input::{default_error_type, parse_input},
};
use graphql_parser::parse_schema;
use proc_macro2::TokenStream;
use syn::Type;

/// Read a GraphQL schema file and generate corresponding Juniper macro calls.
///
/// See [the crate level docs](index.html) for an example.
#[proc_macro]
pub fn graphql_schema_from_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let parsed =
        parse_input(&input.to_string()).expect("Failed to parse input to graphql_schema_from_file");

    match std::fs::read_to_string(&parsed.schema_path) {
        Ok(schema) => parse_and_gen_schema(&schema, parsed.error_type),
        Err(err) => panic!("{}", err),
    }
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
    parse_and_gen_schema(&schema, default_error_type())
}

fn parse_and_gen_schema(schema: &str, error_type: Type) -> proc_macro::TokenStream {
    let doc = match parse_schema(&schema) {
        Ok(doc) => doc,
        Err(parse_error) => panic!("{}", parse_error),
    };

    let ast_data = AstData::new(&doc);
    let output = CodeGenPass::new(schema, error_type, ast_data);

    match output.gen_juniper_code(&doc) {
        Ok(tokens) => {
            let out: proc_macro::TokenStream = tokens.into();

            if debugging_enabled() {
                self::pretty_print::code_gen_debug(out.to_string());
            }

            out
        }
        Err(errors) => {
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
