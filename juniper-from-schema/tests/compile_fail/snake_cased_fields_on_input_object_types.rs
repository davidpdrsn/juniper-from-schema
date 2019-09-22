#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        field: String!
    }

    schema { query: Query }

    input SomeInput {
        snake_cased: String!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_field<'a>(&self, _: &Executor<'a, Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}
