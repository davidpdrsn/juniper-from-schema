// this file can be used for testing/debugging things before moving them into a trybuild test

#![allow(warnings)]

use juniper::{EmptyMutation, Executor, FieldResult, Variables, ID};

pub struct Context;

impl juniper::Context for Context {}

fn main() {}

juniper_from_schema::graphql_schema! {
    schema {
      query: Query
    }

    type Query {
      helloWorld(name: String!): String! @juniper(ownership: "owned")
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_hello_world(&self, executor: &Executor<Context>, name: String) -> FieldResult<String> {
        todo!()
    }
}
