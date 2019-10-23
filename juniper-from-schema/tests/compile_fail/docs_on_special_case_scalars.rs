#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

use url::Url;

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        foo: String! @juniper(ownership: "owned")
    }

    "Url docs"
    scalar Url

    "DateTimeUtc docs"
    scalar DateTimeUtc

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
