use juniper::{EmptyMutation, Executor, FieldResult, Variables};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};

pub struct Context;
impl juniper::Context for Context {}

fn main() {}
