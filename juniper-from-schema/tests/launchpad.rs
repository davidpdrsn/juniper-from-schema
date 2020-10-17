// this file can be used for testing/debugging things before moving them into a trybuild test

#![allow(warnings)]

use juniper::{EmptyMutation, Executor, FieldResult, Variables, ID};
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

pub struct Context;

impl juniper::Context for Context {}

fn main() {}

// juniper_from_schema::graphql_schema! {
//     schema {
//       query: Query
//     }

//     type Query {
//         ping: Boolean! @juniper(infallible: true, ownership: "owned", async: true)
//     }

//     directive @juniper(
//         ownership: String = "borrowed",
//         infallible: Boolean = false,
//         with_time_zone: Boolean = true,
//         async: Boolean = false
//     ) on FIELD_DEFINITION | SCALAR
// }

// pub struct Query;

// #[juniper_from_schema::juniper::async_trait]
// impl QueryFields for Query {
//     async fn field_ping<'r, 'a>(
//         &self,
//         _: &Executor<'r, 'a, Context>,
//     ) -> bool {
//         todo!()
//     }
// }
