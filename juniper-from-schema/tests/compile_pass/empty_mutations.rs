#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema! {
    type Query {
        string: String!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_string<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

fn this_should_compile() {
    let _ = juniper::execute(
        "query Foo { string }",
        None,
        &Schema::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &Context,
    )
    .unwrap();
}
