#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        string: String
        float: Float
        int: Int
        boolean: Boolean
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_string<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<String>> {
        unimplemented!()
    }

    fn field_float<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<f64>> {
        unimplemented!()
    }

    fn field_int<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<i32>> {
        unimplemented!()
    }

    fn field_boolean<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<bool>> {
        unimplemented!()
    }
}
