#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

use chrono::prelude::*;

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        dateTime: DateTime! @juniper(ownership: "owned")
    }

    scalar DateTime @juniper(with_time_zone: "foobar")
}

pub struct Query;

impl QueryFields for Query {
    fn field_date_time(&self, _: &Executor<'_, Context>) -> FieldResult<NaiveDateTime> {
        unimplemented!()
    }
}
