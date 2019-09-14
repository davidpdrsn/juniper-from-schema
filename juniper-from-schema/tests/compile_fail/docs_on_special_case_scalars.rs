#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

use url::Url;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        foo: String! @juniper(ownership: "owned")
    }

    "Url docs"
    scalar Url

    "DateTime docs"
    scalar DateTime

    "Date docs"
    scalar Date

    "Uuid docs"
    scalar Uuid
}

pub struct Query;

impl QueryFields for Query {
    fn field_foo(&self, _: &Executor<'_, Context>) -> FieldResult<String> {
        unimplemented!()
    }
}
