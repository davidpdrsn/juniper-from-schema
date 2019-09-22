#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    scalar Cursor

    type Query {
        field(arg: Cursor!): Cursor!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_field<'a>(
        &self,
        executor: &Executor<'a, Context>,
        arg: Cursor,
    ) -> FieldResult<&Cursor> {
        Cursor::new("123");
        unimplemented!()
    }
}
