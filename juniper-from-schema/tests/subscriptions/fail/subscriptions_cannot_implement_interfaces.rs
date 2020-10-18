#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
      ping: Boolean!
    }

    type Subscription implements Entity {
      users: User! @juniper(infallible: true, ownership: "owned")
    }

    interface Entity {
        id: ID!
    }

    type User {
      id: ID!
      name: String!
    }

    schema {
      query: Query
      subscription: Subscription
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_ping(&self, _: &Executor<Context>) -> FieldResult<&bool> {
        todo!()
    }
}

pub struct Subscription;

impl SubscriptionFields for Subscription {
    fn field_users<'r, 'a>(
        &self,
        _: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, User, Walked>,
    ) -> Box<dyn juniper_from_schema::futures::Stream<Item = User> + Send + Unpin> {
        Box::new(juniper_from_schema::futures::stream::iter(vec![]))
    }
}

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
