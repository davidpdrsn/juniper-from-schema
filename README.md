# juniper-from-schema

**Important notice**: This project is still in a very experimental state. You should probably not use it in serious projects.

---

Wouldn't it be neat if you could parse your GraphQL schema at compile and generate all the juniper boilerplate? While of course ensuring your Rust types match the schema.

That would in theory give you some nice benefits:

- Actually have a GraphQL schema file you can share with the clients. [I don't believe juniper currently supports this.](https://github.com/graphql-rust/juniper#features)
- You app only compiles if its types match the schema.

Also, if you know all the queries your clients will make, you could technically verify that the queries match the schema. And if you know your backend implements the schema then you know queries should work. The circle is complete ♻️

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
// This is needed to import internal juniper macros
#[macro_use]
extern crate juniper;

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

## Debugging

Setting `JUNIPER_FROM_SCHEMA_DEBUG` to `1` will make the generated code be printed to your terminal. The recommended way to do that is with `JUNIPER_FROM_SCHEMA_DEBUG=1 cargo build`. The code will be pretty printed using [rustfmt][https://crates.io/crates/rustfmt], which sadly doesn't support macros so `graphql_object!` calls wont be very legible.
