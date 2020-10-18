#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../subscription_setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
      ping: Boolean!
    }

    type Subscription {
      users: User! @juniper(ownership: "owned", infallible: false, async: false, stream_item_infallible: true)
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

impl SubscriptionFields for Subscription {
    fn field_users<'s, 'r, 'a>(
        &'s self,
        _: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, User, Walked>,
    ) -> FieldResult<BoxStream<User>> {
        todo!()
    }
}

