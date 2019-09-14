#![allow(dead_code, unused_variables, unused_imports)]

#[macro_use]
extern crate juniper;

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
        allPosts(status: Status = PUBLISHED): [Post!]! @juniper(ownership: "owned")
    }

    type Post {
        id: ID!
    }
}

pub struct Context;

impl juniper::Context for Context {}

pub struct Post {
    id: ID,
}

impl PostFields for Post {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&ID> {
        Ok(&self.id)
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_all_posts(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
        status: Status,
    ) -> FieldResult<Vec<Post>> {
        // `status` will be `Status::Published` if not given in the query

        match status {
            Status::Published => unimplemented!("find published posts"),
            Status::Unpublished => unimplemented!("find unpublished posts"),
        }
    }
}

fn main() {}
