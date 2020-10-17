#![allow(
    dead_code,
    unused_mut,
    unused_variables,
    unused_must_use,
    unused_imports
)]
include!("setup.rs");

use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

juniper_from_schema::graphql_schema! {
    schema {
      query: Query
    }

    type Query {
        asyncPing: Boolean! @juniper(infallible: true, ownership: "owned", async: true)
        syncPing: Boolean! @juniper(infallible: true, ownership: "owned", async: false)
    }

    type User implements Entity {
        id: ID! @juniper(infallible: true, ownership: "owned", async: true)
    }

    interface Entity {
        id: ID! @juniper(infallible: true, ownership: "owned", async: true)
    }
}

pub struct Query;

#[juniper_from_schema::juniper::async_trait]
impl QueryFields for Query {
    async fn field_async_ping<'r, 'a>(&self, _: &Executor<'r, 'a, Context>) -> bool {
        ready(true).await
    }

    fn field_sync_ping(&self, _: &Executor<Context>) -> bool {
        true
    }
}

pub struct User;

#[juniper_from_schema::juniper::async_trait]
impl UserFields for User {
    async fn field_id<'r, 'a>(&self, _: &Executor<'r, 'a, Context>) -> ID {
        todo!()
    }
}

// copied from std because it isn't stable yet
pub struct Ready<T>(Option<T>);

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<T> {
        todo!()
    }
}

pub fn ready<T>(t: T) -> Ready<T> {
    todo!()
}
