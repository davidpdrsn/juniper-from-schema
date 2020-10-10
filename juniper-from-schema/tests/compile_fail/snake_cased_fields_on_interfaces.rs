#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        field: SomeInterface!
    }

    schema { query: Query }

    interface SomeInterface {
        snake_cased: String!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_field(
        &self,
        _: &Executor<Context>,
        _: &QueryTrail<SomeInterface, Walked>,
    ) -> FieldResult<&SomeInterface> {
        unimplemented!()
    }
}
