#![allow(dead_code, unused_variables, unused_imports)]

use juniper::*;
use juniper_from_schema::graphql_schema;

graphql_schema! {
    schema {
        query: Query
    }

    enum Status {
        PUBLISHED
        UNPUBLISHED
    }

    type Query {
        allPosts(status: Status!): [Post!]! @juniper(ownership: "owned")
    }

    type Post {
        id: ID!
    }
}

fn main() {}

pub struct Context;

impl juniper::Context for Context {}

pub struct Post {
    id: ID,
}

impl PostFields for Post {
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&ID> {
        Ok(&self.id)
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_all_posts(
        &self,
        executor: &Executor<Context>,
        trail: &QueryTrail<Post, Walked>,
        status: Status,
    ) -> FieldResult<Vec<Post>> {
        match status {
            Status::Published => unimplemented!("find published posts"),
            Status::Unpublished => unimplemented!("find unpublished posts"),
        }
    }
}
