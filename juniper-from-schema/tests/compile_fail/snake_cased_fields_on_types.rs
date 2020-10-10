#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        snake_cased: String!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_snake_cased(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}
