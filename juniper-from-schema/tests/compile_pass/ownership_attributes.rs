#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema_from_file!(
    "../../../juniper-from-schema/tests/schemas/ownership_attributes.graphql"
);

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
