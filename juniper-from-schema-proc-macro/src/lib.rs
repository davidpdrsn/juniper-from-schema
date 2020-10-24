//! See the docs for "juniper-from-schema" for more info about this.

#![deny(
    unused_imports,
    mutable_borrow_reservation_conflict,
    dead_code,
    unused_variables,
    unused_must_use
)]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/juniper-from-schema-proc-macro/0.5.2")]

mod parse_input;

use juniper_from_schema_code_gen::CodeGen;
use parse_input::GraphqlSchemaFromFileInput;

/// Read a GraphQL schema file and generate corresponding Juniper macro calls.
///
/// See [the crate level docs](index.html) for an example.
#[proc_macro]
pub fn graphql_schema_from_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let GraphqlSchemaFromFileInput {
        schema_path,
        context_type,
        error_type,
    } = match syn::parse::<GraphqlSchemaFromFileInput>(input) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut builder = CodeGen::build_from_schema_file(schema_path);
    if let Some(context_type) = context_type {
        builder = builder.context_type(context_type);
    }
    if let Some(error_type) = error_type {
        builder = builder.error_type(error_type);
    }
    let code_gen = builder.finish();

    match code_gen.generate_code() {
        Ok(tokens) => tokens.into(),
        Err(errors) => panic!("{}", errors),
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
///         executor: &Executor<Context>,
///     ) -> FieldResult<String> {
///         Ok("Hello, World!".to_string())
///     }
/// }
/// ```
#[proc_macro]
pub fn graphql_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let schema = input.to_string();

    let code_gen = CodeGen::build_from_schema_literal(schema).finish();

    match code_gen.generate_code() {
        Ok(tokens) => tokens.into(),
        Err(errors) => panic!("{}", errors),
    }
}
