#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate juniper;

use juniper::*;
use juniper_from_schema::graphql_schema;

graphql_schema! {
    schema {
        query: Query
        mutation: Mutation
    }

    type Mutation {
        createPost(input: CreatePost!): Post @juniper(ownership: "owned")
    }

    input CreatePost {
        title: String!
    }

    type Post {
        id: ID!
        title: String!
    }

    type Query { noop: Boolean! }
}

fn main() {}

pub struct Context;

impl juniper::Context for Context {}

pub struct Post {
    id: ID,
}

impl PostFields for Post {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_title(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_noop(&self, executor: &Executor<'_, Context>) -> FieldResult<&bool> {
        unimplemented!()
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_create_post(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
        input: CreatePost,
    ) -> FieldResult<Option<Post>> {
        let title: String = input.title;

        unimplemented!()
    }
}
