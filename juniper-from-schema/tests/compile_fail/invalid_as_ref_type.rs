#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
      asRefString: String! @juniper(ownership: "as_ref")
    }

    schema {
      query: Query
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_as_ref_string<'a>(
        &self,
        executor: &Executor<'a, Context>,
    ) -> FieldResult<String> {
        unimplemented!()
    }
}
