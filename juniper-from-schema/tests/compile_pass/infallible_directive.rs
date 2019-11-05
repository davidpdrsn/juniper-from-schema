#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        unowned: String! @juniper(infallible: true)
        owned: String! @juniper(ownership: "owned", infallible: true)
        ownedReordered: String! @juniper(infallible: true, ownership: "owned")
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_unowned(&self, _: &Executor<'_, Context>) -> &String {
        unimplemented!()
    }

    fn field_owned(&self, _: &Executor<'_, Context>) -> String {
        unimplemented!()
    }

    fn field_owned_reordered(&self, _: &Executor<'_, Context>) -> String {
        unimplemented!()
    }
}
