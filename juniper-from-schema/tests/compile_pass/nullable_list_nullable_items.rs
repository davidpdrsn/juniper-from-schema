#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        field: [Int]
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_field<'a>(
        &self,
        executor: &Executor<'a, Context>,
    ) -> FieldResult<&Option<Vec<Option<i32>>>> {
        unimplemented!()
    }
}
