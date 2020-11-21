#![allow(dead_code, unused_variables, unused_imports)]

use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use juniper::*;
use juniper_from_schema::graphql_schema;
use std::pin::Pin;
use tokio::sync::broadcast::Sender;

graphql_schema! {
    schema {
        query: Query
        subscription: Subscription
    }

    type Query {
        // Query must have at least one field
        ping: Boolean!
    }

    type Subscription {
        tweets: Tweet! @juniper(ownership: "owned", infallible: true, stream_item_infallible: false)
    }

    type Tweet {
        id: ID!
    }
}

pub struct Context {
    tx: Sender<Tweet>,
}

impl juniper::Context for Context {}

#[derive(Clone)]
pub struct Tweet {
    id: ID,
    text: String,
}

impl TweetFields for Tweet {
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&ID> {
        unimplemented!()
    }
}

pub struct Subscription;

impl SubscriptionFields for Subscription {
    fn field_tweets(
        &self,
        executor: &Executor<Context>,
        _: &QueryTrail<Tweet, Walked>,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<Tweet>> + Send>> {
        let ctx = executor.context();
        let receiver = ctx.tx.subscribe();
        let stream = receiver.into_stream().map(|item| item.map_err(From::from));
        Box::pin(stream)
    }
}

pub struct Query;

#[async_trait]
impl QueryFields for Query {
    fn field_ping(&self, executor: &Executor<Context>) -> FieldResult<&bool> {
        todo!()
    }
}

fn main() {}
