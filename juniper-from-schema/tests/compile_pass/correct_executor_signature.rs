#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        field: Int!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_field(&self, executor: &Executor<Context>) -> FieldResult<&i32> {
        unimplemented!()
    }
}
