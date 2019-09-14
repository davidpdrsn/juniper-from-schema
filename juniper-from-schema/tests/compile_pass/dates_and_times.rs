#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

use chrono::{naive::NaiveDate, prelude::*};

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        date: Date! @juniper(ownership: "owned")
        dateTime: DateTime! @juniper(ownership: "owned")
    }

    scalar Date
    scalar DateTime
}

pub struct Query;

impl QueryFields for Query {
    fn field_date(&self, _: &Executor<'_, Context>) -> FieldResult<NaiveDate> {
        unimplemented!()
    }

    fn field_date_time(&self, _: &Executor<'_, Context>) -> FieldResult<DateTime<Utc>> {
        unimplemented!()
    }
}
