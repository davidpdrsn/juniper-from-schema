# [juniper-from-schema](https://crates.io/crates/juniper-from-schema)

This library contains a procedural macro that reads a GraphQL schema file, and generates the
corresponding [Juniper](https://crates.io/crates/juniper) [macro calls]. This means you can
have a real schema file and be guaranteed that it matches your Rust implementation. It also
removes most of the boilerplate involved in using Juniper.

[macro calls]: https://graphql-rust.github.io/types/objects/complex_fields.html

## Table of contents

- [Example](#example)
- [Customizing ownership](#customizing-ownership)
- [GraphQL features](#graphql-features)
    - [The `ID` type](#the-id-type)
    - [Custom scalar types](#custom-scalar-types)
    - [Interfaces](#interfaces)
    - [Union types](#union-types)
    - [Input objects](#input-objects)
    - [Enumeration types](#enumeration-types)
- [GraphQL to Rust types](#graphql-to-rust-types)
- [Query trails](#query-trails)
- [Customizing the error type](#customizing-the-error-type)
- [Inspecting the generated code](#inspecting-the-generated-code)

## Example

Schema:

```graphql
schema {
  query: Query
  mutation: Mutation
}

type Query {
  // this makes the return value `FieldResult<String>`
  // rather than the default `FieldResult<&String>`
  "#[ownership(owned)]"
  helloWorld(name: String!): String!
}

type Mutation {
  noop: Boolean!
}
```

How you could implement that schema:

```rust
#[macro_use]
extern crate juniper;

use juniper_from_schema::graphql_schema_from_file;

// This is the important line
graphql_schema_from_file!("tests/schemas/doc_schema.graphql");

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_hello_world(
        &self,
        executor: &juniper::Executor<'_, Context>,
        name: String,
    ) -> juniper::FieldResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_noop(&self, executor: &juniper::Executor<'_, Context>) -> juniper::FieldResult<&bool> {
        Ok(&true)
    }
}

fn main() {
    let ctx = Context;

    let query = "query { helloWorld(name: \"Ferris\") }";

    let (result, errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, Mutation),
        &juniper::Variables::new(),
        &ctx,
    )
    .unwrap();

    assert_eq!(errors.len(), 0);
    assert_eq!(
        result
            .as_object_value()
            .unwrap()
            .get_field_value("helloWorld")
            .unwrap()
            .as_scalar_value::<String>()
            .unwrap(),
        "Hello, Ferris!",
    );
}
```

And with `graphql_schema_from_file!` expanded your code would look something like this:

```rust
#[macro_use]
extern crate juniper;

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;

juniper::graphql_object!(Query: Context |&self| {
    field hello_world(&executor, name: String) -> juniper::FieldResult<String> {
        <Self as QueryFields>::field_hello_world(&self, &executor, name)
    }
});

trait QueryFields {
    fn field_hello_world(
        &self,
        executor: &juniper::Executor<'_, Context>,
        name: String,
    ) -> juniper::FieldResult<String>;
}

impl QueryFields for Query {
    fn field_hello_world(
        &self,
        executor: &juniper::Executor<'_, Context>,
        name: String,
    ) -> juniper::FieldResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}

pub struct Mutation;

juniper::graphql_object!(Mutation: Context |&self| {
    field noop(&executor) -> juniper::FieldResult<&bool> {
        <Self as MutationFields>::field_noop(&self, &executor)
    }
});

trait MutationFields {
    fn field_noop(&self, executor: &juniper::Executor<'_, Context>) -> juniper::FieldResult<&bool>;
}

impl MutationFields for Mutation {
    fn field_noop(&self, executor: &juniper::Executor<'_, Context>) -> juniper::FieldResult<&bool> {
        Ok(&true)
    }
}

type Schema = juniper::RootNode<'static, Query, Mutation>;

fn main() {
    let ctx = Context;

    let query = "query { helloWorld(name: \"Ferris\") }";

    let (result, errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, Mutation),
        &juniper::Variables::new(),
        &ctx,
    )
    .unwrap();

    assert_eq!(errors.len(), 0);
    assert_eq!(
        result
            .as_object_value()
            .unwrap()
            .get_field_value("helloWorld")
            .unwrap()
            .as_scalar_value::<String>()
            .unwrap(),
        "Hello, Ferris!",
    );
}
```

## Customizing ownership

By default all fields return borrowed values. Specifically the type is
`juniper::FieldResult<&'a T>` where `'a` is the lifetime of `self`. This works well for
returning data owned by `self` and avoids needless `.clone()` calls you would need if fields
returned owned values.

However if you need to return owned values (such as values queried from a database) you have to
annotate the field in the schema with `#[ownership(owned)]`.

All field arguments will be owned.

## GraphQL features

The goal of this library is to support as much of GraphQL as Juniper does.

Here is the complete list of features:

Supported:
- Object types including converting lists and non-nulls to Rust types
- Custom scalar types including the `ID` type
- Interfaces
- Unions
- Input objects
- Enumeration types

Not supported yet:
- Default values for arguments
- Subscriptions (currently not supported by Juniper so we're unsure when or if this will happen)

### The `ID` type

The `ID` GraphQL type will be generated as a newtype wrapper around a `String` using
[`juniper::graphql_scalar!`](https://docs.rs/juniper/0.11.1/juniper/macro.graphql_scalar.html). The Rust type will be called `Id`.

Example:

```rust
pub struct Id(pub String);

impl Id {
    // A generated convenience initializer
    pub fn new<T: Into<String>>(id: T) -> Self {
        Id(id.into())
    }
}
```

### Custom scalar types

Similarly to `ID`, custom scalar types get converted into newtype wrappers around `String`s. For example:

```graphql
scalar Cursor
```

Would result in

```rust
pub struct Cursor(pub String);
```

`Date` and `DateTime` are the two exceptions to this. `Date` gets converted into
[`chrono::naive::NaiveDate`](https://docs.rs/chrono/0.4.6/chrono/naive/struct.NaiveDate.html)
and `DateTime` into
[`chrono::DateTime<chrono::offset::Utc>`](https://docs.rs/chrono/0.4.6/chrono/struct.DateTime.html).

### Interfaces

Juniper has several ways of representing GraphQL interfaces in Rust. They are listed
[here](https://graphql-rust.github.io/types/interfaces.html#enums) along with their advantages
and disadvantages.

For the generated code we use the `enum` pattern because we found it to be the most flexible.

Abbreviated example (find [complete example here](https://github.com/davidpdrsn/juniper-from-schema/blob/examples/examples/interface.rs)):

```rust
#
graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        "#[ownership(owned)]"
        search(query: String!): [SearchResult!]!
    }

    interface SearchResult {
        id: ID!
        text: String!
    }

    type Article implements SearchResult {
        id: ID!
        text: String!
    }

    type Tweet implements SearchResult {
        id: ID!
        text: String!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_search(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, SearchResult, Walked>,
        query: String,
    ) -> FieldResult<Vec<SearchResult>> {
        let article: Article = Article { id: Id::new("1"), text: "Business".to_string() };
        let tweet: Tweet = Tweet { id: Id::new("2"), text: "1 weird tip".to_string() };

        let posts = vec![
            SearchResult::from(article),
            SearchResult::from(tweet),
        ];

        Ok(posts)
    }
}
```

The enum that gets generated has variants for each type that implements the interface and also
implements `From<T>` for each type.

### Union types

Union types are basically just interfaces so they work in very much the same way.

Abbreviated example (find [complete example here](https://github.com/davidpdrsn/juniper-from-schema/blob/examples/examples/union_types.rs)):

```rust
#
graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        "#[ownership(owned)]"
        search(query: String!): [SearchResult!]!
    }

    union SearchResult = Article | Tweet

    type Article {
        id: ID!
        text: String!
    }

    type Tweet {
        id: ID!
        text: String!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_search(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, SearchResult, Walked>,
        query: String,
    ) -> FieldResult<Vec<SearchResult>> {
        let article: Article = Article { id: Id::new("1"), text: "Business".to_string() };
        let tweet: Tweet = Tweet { id: Id::new("2"), text: "1 weird tip".to_string() };

        let posts = vec![
            SearchResult::from(article),
            SearchResult::from(tweet),
        ];

        Ok(posts)
    }
}
```

### Input objects

Input objects will be converted into Rust structs with public fields.

Abbreviated example (find [complete example here](https://github.com/davidpdrsn/juniper-from-schema/blob/examples/examples/input_types.rs)):

```rust
graphql_schema! {
    schema {
        query: Query
        mutation: Mutation
    }

    type Mutation {
        "#[ownership(owned)]"
        createPost(input: CreatePost!): Post
    }

    input CreatePost {
        title: String!
    }

    type Post {
        id: ID!
        title: String!
    }

    type Query { noop: Boolean! }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_create_post(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
        input: CreatePost,
    ) -> FieldResult<Option<Post>> {
        let title: String = input.title;

        unimplemented!()
    }
}
```

From that example `CreatePost` will be defined as

```rust
pub struct CreatePost {
    pub title: String,
}
```

### Enumeration types

GraphQL enumeration types will be converted into normal Rust enums. The name of each variant
will be camel cased.

Abbreviated example (find [complete example here](https://github.com/davidpdrsn/juniper-from-schema/blob/examples/examples/enumeration_types.rs)):

```rust
#
graphql_schema! {
    schema {
        query: Query
    }

    enum Status {
        PUBLISHED
        UNPUBLISHED
    }

    type Query {
        "#[ownership(owned)]"
        allPosts(status: STATUS!): [Post!]!
    }

    type Post {
        id: ID!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_all_posts(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
        status: Status,
    ) -> FieldResult<Vec<Post>> {
        match status {
            Status::Published => unimplemented!("find published posts"),
            Status::Unpublished => unimplemented!("find unpublished posts"),
        }
    }
}
```

## GraphQL to Rust types

This is how the standard GraphQL types will be mapped to Rust:

- `Int` -> `i32`
- `Float` -> `f64`
- `String` -> `String`
- `Boolean` -> `bool`
- `ID` -> `pub struct Id(pub String)`

## Query trails

If you're not careful about preloading associations for deeply nested queries you risk getting
lots of [N+1 query bugs][]. Juniper provides a [look ahead api][] which lets you inspect things
coming up further down a query. However the API is string based, so you risk making typos and
checking for fields that don't exist.

`QueryTrail` is a thin wrapper around Juniper look aheads with generated methods for each field
on all your types. This means the compiler will reject your code if you're checking for invalid
fields.

Fields that return objects types (non scalar values) will also get a `QueryTrail` argument
besides the executor.

[N+1 query bugs]: https://secure.phabricator.com/book/phabcontrib/article/n_plus_one/
[look ahead api]: https://docs.rs/juniper/0.11.1/juniper/struct.LookAheadSelection.html

### Abbreviated example

Find [complete example here](https://github.com/davidpdrsn/juniper-from-schema/blob/examples/examples/query_trails.rs)

```rust
#
graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        "#[ownership(owned)]"
        allPosts: [Post!]!
    }

    type Post {
        id: Int!
        author: User!
    }

    type User {
        id: Int!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_all_posts(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
    ) -> FieldResult<Vec<Post>> {
        // Check if the query includes the author
        if let Some(_) = trail.author().walk() {
            // Somehow preload the users to avoid N+1 query bugs
            // Exactly how to do this depends on your setup
        }

        // Normally this would come from the database
        let post = Post {
            id: 1,
            author: User { id: 1 },
        };

        Ok(vec![post])
    }
}

pub struct Post {
    id: i32,
    author: User,
}

impl PostFields for Post {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.id)
    }

    fn field_author(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<&User> {
        Ok(&self.author)
    }
}

pub struct User {
    id: i32,
}

impl UserFields for User {
    fn field_id(
        &self,
        executor: &Executor<'_, Context>,
    ) -> FieldResult<&i32> {
        Ok(&self.id)
    }
}
```

### Types

A query trial has two generic parameters: `QueryTrail<'a, T, K>`. `T` is the type the current
field returns and `K` is either `Walked` or `NotWalked`.

The lifetime `'a` comes from Juniper and is the lifetime of the incoming query.

#### `T`

The `T` allows us to implement different methods for different types. For example in the
example above we implement `id` and `author` for `QueryTrail<'_, Post, K>` but only `id` for
`QueryTrail<'_, User, K>`.

If your field returns a `Vec<T>` or `Option<T>` the given query trail will be `QueryTrail<'_,
T, _>`. So `Vec` or `Option` will be removed and you'll only be given the inner most type.
That is because in the GraphQL query syntax it doesn't matter if you're querying a `User`
or `[User]`. The fields you have access to are the same.

#### `K`

The `Walked` and `NotWalked` types are used to check if a given trail has been checked to
actually be part of a query. Calling any method on a `QueryTrail<'_, T, K>` will return
`QueryTrail<'_, T, NotWalked>`, and to check if the trail is actually part of the query you have
to call `.walk()` which returns `Option<QueryTrail<'_, T, Walked>>`. If that is a `Some(_)`
you'll know the trail is part of the query and you can do whatever preloading is necessary.

Example:

```rust
if let Some(walked_trail) = trail
    .some_field()
    .some_other_field()
    .third_field()
    .walk()
{
    // preload stuff
}
```

You can always run `cargo doc` and inspect all the methods on `QueryTrail` and in which
contexts you can call them.

## Customizing the error type

By default the return type of the generated field methods will be [`juniper::FieldResult<T>`].
That is just a type alias for `std::result::Result<T, juniper::FieldError>`. Should you want to
use a different error type than [`juniper::FieldError`] that can be done by passing `,
error_type: YourType` to [`graphql_schema_from_file!`].

Just keep in that your custom error type must implement [`juniper::IntoFieldError`] to
type check.

Example:

```rust
graphql_schema_from_file!("tests/schemas/doc_schema.graphql", error_type: MyError);

pub struct MyError(String);

impl juniper::IntoFieldError for MyError {
    fn into_field_error(self) -> juniper::FieldError {
        // Perform custom error handling
        juniper::FieldError::from(self.0)
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_hello_world(
        &self,
        executor: &Executor<'_, Context>,
        name: String,
    ) -> Result<String, MyError> {
        Ok(format!("Hello, {}!", name))
    }
}
```

[`graphql_schema!`] does not support changing the error type.

[`graphql_schema!`]: macro.graphql_schema.html
[`graphql_schema_from_file!`]: macro.graphql_schema_from_file.html
[`juniper::IntoFieldError`]: https://docs.rs/juniper/0.11.1/juniper/trait.IntoFieldError.html
[`juniper::FieldError`]: https://docs.rs/juniper/0.11.1/juniper/struct.FieldError.html
[`juniper::FieldResult<T>`]: https://docs.rs/juniper/0.11.1/juniper/type.FieldResult.html

## Inspecting the generated code

If you wish to see exactly what code gets generated you can set the env var
`JUNIPER_FROM_SCHEMA_DEBUG` to `1` when compiling. For example:

```bash
JUNIPER_FROM_SCHEMA_DEBUG=1 cargo build
```

The code will not be formatted so it might be tricky to read. The easiest way to fix this is to
copy the printed code to a file and run it through [rustfmt].

Alternatively you can include the [feature] called `"format-debug-output"`. This will run the
output through [rustfmt] before printing it. That way you don't have to do that manually.
Example `Cargo.toml`:

```toml
[dependencies]
juniper-from-schema = { version = "x.y.z", features = ["format-debug-output"] }
```

Unfortunately this requires that you're using nightly, because [rustfmt requires
nightly](https://github.com/rust-lang/rustfmt#installing-from-source). It might also break your
build because [rustfmt] doesn't always compile for some reason ¯\\\_(ツ)\_/¯. If you experience
this just remove the `"format-debug-output"` feature and format the output manually.

[feature]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features
[rustfmt]: https://github.com/rust-lang/rustfmt

---

License: MIT
