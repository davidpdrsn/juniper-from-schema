// this file can be used for testing/debugging things before moving them into a trybuild test

#![allow(warnings)]

use juniper::{EmptyMutation, Executor, FieldResult, Variables, ID};

pub struct Context;

impl juniper::Context for Context {}

fn main() {}
