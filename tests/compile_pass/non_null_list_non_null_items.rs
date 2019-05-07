#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema! {
    type Query {
        field: [Int!]!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_field<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Vec<i32>> {
        unimplemented!()
    }
}
