#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

use uuid::Uuid;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        uuid: Uuid! @juniper(ownership: "owned")
    }

    scalar Uuid
}

pub struct Query;

impl QueryFields for Query {
    fn field_uuid(&self, _: &Executor<'_, Context>) -> FieldResult<Uuid> {
        Ok(Uuid::new_v4())
    }
}
