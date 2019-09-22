#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    enum YES_NO {
        YES
        NO
        NOT_SURE
    }

    type Query {
        yesNo(arg: YES_NO): YES_NO!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_yes_no<'a>(
        &self,
        executor: &Executor<'a, Context>,
        arg: Option<YesNo>,
    ) -> FieldResult<&YesNo> {
        let _: YesNo = YesNo::No;
        let _: YesNo = YesNo::Yes;
        let _: YesNo = YesNo::NotSure;
        unimplemented!()
    }
}
