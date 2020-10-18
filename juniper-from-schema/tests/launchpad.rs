// this file can be used for testing/debugging things before moving them into a trybuild test

#![allow(warnings)]

use futures::stream::Stream;
use juniper::{EmptyMutation, Executor, FieldResult, Variables, ID};
use std::pin::Pin;

pub struct Context;

impl juniper::Context for Context {}

fn main() {}
