#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        string: String!
    }

    schema { query: Query }

    directive @juniper(
        ownership: String = "borrowed",
        infallible: Boolean = false,
        with_time_zone: Boolean = true,
        async: Boolean = false,
        stream_item_infallible: Boolean = true,
        stream_type: String = null
    ) on FIELD_DEFINITION | SCALAR
}

pub struct Query;

impl QueryFields for Query {
    fn field_string(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}
