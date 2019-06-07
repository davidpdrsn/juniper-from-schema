#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema! {
    type Query {
      borrowed_string: String! @juniper(ownership: "borrowed")
      owned_string: String! @juniper(ownership: "owned")
      as_ref_string: String @juniper(ownership: "as_ref")
    }

    schema {
      query: Query
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_borrowed_string<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&String> {
        unimplemented!()
    }

    fn field_owned_string<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<String> {
        unimplemented!()
    }

    fn field_as_ref_string<'a>(
        &self,
        executor: &Executor<'a, Context>,
    ) -> FieldResult<Option<&String>> {
        unimplemented!()
    }
}
