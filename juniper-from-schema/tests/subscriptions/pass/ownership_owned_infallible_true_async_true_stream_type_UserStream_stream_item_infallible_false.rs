#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../subscription_setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
      ping: Boolean!
    }

    type Subscription {
      users: User! @juniper(ownership: "owned", infallible: true, async: true, stream_type: "UserStream", stream_item_infallible: false)
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

#[async_trait]
impl SubscriptionFields for Subscription {
    async fn field_users<'s, 'r, 'a>(
        &'s self,
        _: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, User, Walked>,
    ) -> UserStream {
        todo!()
    }
}

pub struct UserStream;

impl Stream for UserStream {
    type Item = FieldResult<User>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut futures::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {
        todo!()
    }
}