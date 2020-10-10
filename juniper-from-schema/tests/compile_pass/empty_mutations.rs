#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        string: String!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_string(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

fn this_should_compile() {
    let _ = juniper::execute_sync(
        "query Foo { string }",
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &Context,
    )
    .unwrap();
}
