#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

use uuid::Uuid;

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        uuid: UUID! @juniper(ownership: "owned")
    }

    scalar UUID
}

pub struct Query;

impl QueryFields for Query {
    fn field_uuid(&self, _: &Executor<Context>) -> FieldResult<Uuid> {
        Ok(Uuid::new_v4())
    }
}
