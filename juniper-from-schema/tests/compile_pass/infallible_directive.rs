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

    type User implements Entity {
        id: ID! @juniper(infallible: true)
    }

    interface Entity {
        id: ID! @juniper(infallible: true)
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_unowned(&self, _: &Executor<Context>) -> &String {
        unimplemented!()
    }

    fn field_owned(&self, _: &Executor<Context>) -> String {
        unimplemented!()
    }

    fn field_owned_reordered(&self, _: &Executor<Context>) -> String {
        unimplemented!()
    }
}

pub struct User;

impl UserFields for User {
    fn field_id(&self, _: &Executor<Context>) -> &ID {
        unimplemented!()
    }
}
