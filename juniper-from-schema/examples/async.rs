#![allow(dead_code, unused_variables, unused_imports)]

use async_trait::async_trait;
use juniper::*;
use juniper_from_schema::graphql_schema;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        findTweet(id: ID!): [Tweet!]! @juniper(ownership: "owned", async: true)
    }

    type Tweet {
        id: ID!
        text: String!
    }
}

pub struct Context;

impl juniper::Context for Context {}

pub struct Tweet {
    id: ID,
    text: String,
}

impl TweetFields for Tweet {
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_text(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

pub struct Query;

#[async_trait]
impl QueryFields for Query {
    async fn field_find_tweet<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, Context>,
        trail: &QueryTrail<'r, Tweet, Walked>,
        id: ID,
    ) -> FieldResult<Vec<Tweet>> {
        let tweets = vec![Tweet {
            id,
            text: String::from("Hello, World!"),
        }];

        Ok(tweets)
    }
}

fn main() {}
