#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        type: String!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_type(&self, _: &Executor<'_, Context>, _: String) -> FieldResult<&String> {
        unimplemented!()
    }
}
