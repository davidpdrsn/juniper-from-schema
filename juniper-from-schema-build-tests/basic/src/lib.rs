#![allow(unused_braces)]

use juniper::{Executor, FieldResult};

juniper_from_schema::include_schema!();

#[derive(Debug)]
pub struct Context;

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Query;

impl QueryFields for Query {
    fn field_ping(&self, _: &Executor<Context>) -> FieldResult<&bool> {
        todo!()
    }
}
