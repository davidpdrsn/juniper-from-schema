# juniper-from-schema

**Important notice**: This project is just an idea and shouldn't be used for anything serious.

---

Wouldn't it be neat if you could parse your GraphQL schema at compile and generate all the boilerplate? While of course ensuring your types match the schema.

That is the idea of this crate.

Write something like:

```rust
use juniper::{Executor, FieldResult};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};

graphql_schema! {
    type User {
        id: Int!
    }

    type Query {
        user(id: Int!): User!
        allUsers: [User!]!
    }

    type Mutation {
        newUser(id: Int!): User!
    }

    schema {
        query: Query
        mutation: Mutation
    }
}

struct User {
    id: i32,
}

impl User {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<i32> {
        Ok(self.id)
    }
}

fn find_user(id: i32) -> User {
    User { id: id }
}

struct Query;

impl Query {
    fn field_user<'a>(&self, executor: &Executor<'a, Context>, id: i32) -> FieldResult<User> {
        Ok(find_user(id))
    }

    fn field_all_users<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<Vec<User>> {
        Ok(vec![find_user(1)])
    }
}

struct Mutation;

impl Mutation {
    fn field_new_user<'a>(&self, executor: &Executor<'a, Context>, id: i32) -> FieldResult<User> {
        // TODO: Actually create a new user
        Ok(find_user(1))
    }
}

struct Context {}
impl juniper::Context for Context {}

fn main() {
    // TODO
}
```

That will generate `juniper::graphql_object!` calls for each of the types and delegate to methods named `field_{field name}`. That is all it does so far, but might evolve, who knows?
