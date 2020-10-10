#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        single(arg: Int!): Int!
        multiple(one: Int!, two: String, three: [Float]): Int!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_single(&self, executor: &Executor<Context>, arg: i32) -> FieldResult<&i32> {
        unimplemented!()
    }

    fn field_multiple(
        &self,
        executor: &Executor<Context>,
        one: i32,
        two: Option<String>,
        three: Option<Vec<Option<f64>>>,
    ) -> FieldResult<&i32> {
        unimplemented!()
    }
}
