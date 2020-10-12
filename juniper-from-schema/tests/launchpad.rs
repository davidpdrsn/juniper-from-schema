// this file can be used for testing/debugging things before moving them into a trybuild test

#![allow(warnings)]

use juniper::{EmptyMutation, Executor, FieldResult, Variables, ID};

pub struct Context;

impl juniper::Context for Context {}

fn main() {}

// juniper_from_schema::graphql_schema! {
//     schema {
//       query: Query
//     }

//     type Query {
//         enumeration(arg: Unit = METER): Unit! @juniper(ownership: "owned")
//     }

//     enum Unit { METER FOOT }
// }

pub struct Query;

#[juniper::graphql_object]
impl Query {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// impl QueryFields for Query {
//     fn field_enumeration(
//         &self,
//         _: &Executor<Context>,
//         arg: Unit,
//     ) -> FieldResult<Unit> {
//         Ok(arg)
//     }
// }
