# juniper-from-schema

**Important notice**: This project is still in a very experimental state. You should probably not use it in serious projects.

---

Wouldn't it be neat if you could parse your GraphQL schema at compile and generate all the juniper boilerplate? While of course ensuring your Rust types match the schema.

That is the idea of this crate.

## Basic example

Schema:

```graphql
type Query {
  currentUser: User
}

type User {
  id: Int!
}

schema {
  query: Query
}
```

Rust code:

```rust
use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema;

graphql_schema_from_file!("schema.graphql")

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_current_user<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, User, Walked>,
    ) -> FieldResult<Option<User>> {
        unimplemented!("code for finding current user")
    }
}

struct User {
    id: i32,
}

impl UserFields for User {
    fn field_id<'a>(
        &self,
        executor: &Executor<'a, Context>,
    ) -> FieldResult<&i32> {
        Ok(&self.id)
    }
}
```

For each type in your GraphQL schema `graphql_schema_from_file!` will generate traits named `{Type}Fields` with methods for each field. It also generates calls to `graphql_object!` that just calls the methods from the trait.

The `QueryTrail` is another generated type which provides compile look aheads check by the compiler. So rather than calling `select_child("edges")?.select_child("nodes")` you instead call `trail.edges().nodes().walk()?` where the method names are generated from the schema, so you cannot get them wrong.
