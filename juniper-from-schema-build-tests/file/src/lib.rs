#![allow(unused_braces)]

use juniper::{Executor, FieldError, FieldResult, IntoFieldError};

juniper_from_schema::include_schema!();

#[derive(Debug)]
pub struct Context;

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Query;

impl QueryFields for Query {
    fn field_ping(&self, _: &Executor<()>) -> Result<&bool, MyError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct MyError;

impl<S> IntoFieldError<S> for MyError {
    fn into_field_error(self) -> FieldError<S> {
        todo!()
    }
}
