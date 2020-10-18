include!("../compile_pass/setup.rs");

use futures::stream::Stream;
use async_trait::async_trait;

type BoxStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

pub struct User {
    id: ID,
    name: String,
}

impl UserFields for User {
    fn field_id(&self, _: &Executor<Context>) -> FieldResult<&ID> {
        todo!()
    }

    fn field_name(&self, _: &Executor<Context>) -> FieldResult<&String> {
        todo!()
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_ping(&self, _: &Executor<Context>) -> FieldResult<&bool> {
        todo!()
    }
}

pub struct Subscription;
