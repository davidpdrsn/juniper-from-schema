#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
use juniper::{EmptyMutation, Executor, FieldResult, Variables};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};

pub struct Context;
impl juniper::Context for Context {}

graphql_schema! {
    schema {
      query: Query
    }

    type Query {
      users(arg: String!): [User!]! @juniper(ownership: "owned")
      // field: String @juniper(ownership: "owned")
    }

    type User {
      id: Int!
      name: String!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_users(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
        _arg: String,
    ) -> FieldResult<Vec<User>> {
        Ok(vec![])
    }
}

pub struct User {
    id: i32,
    name: String,
}

impl UserFields for User {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&i32> {
        Ok(&self.id)
    }

    fn field_name<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
}
