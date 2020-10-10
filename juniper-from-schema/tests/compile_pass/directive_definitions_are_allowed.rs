#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        string: String!
    }

    schema { query: Query }

    directive @foo(bar: [String!]!) on FIELD
}

pub struct Query;

impl QueryFields for Query {
    fn field_string(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}
