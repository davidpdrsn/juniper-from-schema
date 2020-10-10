#![allow(dead_code, unused_variables, unused_imports)]

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
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_title(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_noop(&self, executor: &Executor<Context>) -> FieldResult<&bool> {
        unimplemented!()
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_create_post(
        &self,
        executor: &Executor<Context>,
        trail: &QueryTrail<Post, Walked>,
        input: CreatePost,
    ) -> FieldResult<Option<Post>> {
        let title: String = input.title;

        unimplemented!()
    }
}
