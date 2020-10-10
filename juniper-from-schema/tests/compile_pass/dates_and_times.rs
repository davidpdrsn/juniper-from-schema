#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

use chrono::{naive::NaiveDate, prelude::*};

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        date: Date! @juniper(ownership: "owned")
        dateTime: DateTimeUtc! @juniper(ownership: "owned")
    }

    scalar Date
    scalar DateTimeUtc
}

pub struct Query;

impl QueryFields for Query {
    fn field_date(&self, _: &Executor<Context>) -> FieldResult<NaiveDate> {
        unimplemented!()
    }

    fn field_date_time(&self, _: &Executor<Context>) -> FieldResult<DateTime<Utc>> {
        unimplemented!()
    }
}
