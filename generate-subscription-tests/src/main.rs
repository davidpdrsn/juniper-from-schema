// This crate is used to generate the tests you'll find in "juniper-from-schema/tests/subscriptions."
//
// There are quite a few ways in which directives interact so we generate the tests to make sure we
// cover all the cases.
//
// The generated files will be placed in
// "juniper-from-schema/generate-subscription-tests/generated_files" so currently you manually have
// to move them into "juniper-from-schema/tests/subscriptions"

fn main() {
    let combinations = Ownership::all()
        .flat_map(|ownership| {
            Infallible::all().flat_map(move |infallible| {
                Async::all().flat_map(move |async_| {
                    StreamType::all().flat_map(move |stream_type| {
                        StreamItemInfallible::all().map(move |stream_item_infallible| {
                            (
                                ownership,
                                infallible,
                                async_,
                                stream_type,
                                stream_item_infallible,
                            )
                        })
                    })
                })
            })
        })
        .collect::<Vec<(
            Ownership,
            Infallible,
            Async,
            StreamType,
            StreamItemInfallible,
        )>>();

    for (ownership, infallible, async_, stream_type, stream_item_infallible) in combinations {
        let args = vec![
            ownership.to_string(),
            infallible.to_string(),
            async_.to_string(),
            stream_type.to_string(),
            stream_item_infallible.to_string(),
        ]
        .into_iter()
        .filter_map(|item| item)
        .collect::<Vec<_>>()
        .join(", ");

        let return_type = match (stream_type, infallible, stream_item_infallible, ownership) {
            (_, _, _, Ownership::AsRef) => continue,
            (_, _, _, Ownership::Borrowed) => continue,

            (StreamType::Default, Infallible::True, StreamItemInfallible::True, Ownership::Owned) => {
                "BoxStream<User>"
            }
            (StreamType::Default, Infallible::True, StreamItemInfallible::False, Ownership::Owned) => {
                "BoxStream<FieldResult<User>>"
            }
            (StreamType::Default, Infallible::False, StreamItemInfallible::True, Ownership::Owned) => {
                "FieldResult<BoxStream<User>>"
            }
            (StreamType::Default, Infallible::False, StreamItemInfallible::False, Ownership::Owned) => {
                "FieldResult<BoxStream<FieldResult<User>>>"
            }

            (StreamType::Custom, Infallible::True, _, _) => "UserStream",
            (StreamType::Custom, Infallible::False, _, _) => "FieldResult<UserStream>",
        };

        let async_trait = match async_ {
            Async::True => "#[async_trait]\n",
            Async::False => "",
        };

        let async_str = match async_ {
            Async::True => "async ",
            Async::False => "",
        };

        let stream_item_ty = match stream_item_infallible {
            StreamItemInfallible::True => "User",
            StreamItemInfallible::False => "FieldResult<User>",
        };

        let user_stream = if return_type.contains("UserStream") {
            format!(r#"pub struct UserStream;

impl Stream for UserStream {{
    type Item = {};

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut futures::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {{
        todo!()
    }}
}}"#, stream_item_ty)
        } else {
            String::new()
        };

        let body = "todo!()";

        let code = format!(
            r##"#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../subscription_setup.rs");

juniper_from_schema::graphql_schema! {{
    type Query {{
      ping: Boolean!
    }}

    type Subscription {{
      users: User! @juniper({args})
    }}

    type User {{
      id: ID!
      name: String!
    }}

    schema {{
      query: Query
      subscription: Subscription
    }}
}}

{async_trait}impl SubscriptionFields for Subscription {{
    {async}fn field_users<'s, 'r, 'a>(
        &'s self,
        _: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, User, Walked>,
    ) -> {type} {{
        {body}
    }}
}}

{user_stream}"##,
            args = args, type = return_type, body = body, async = async_str, async_trait = async_trait, user_stream = user_stream
        );

        let file_name = format!(
            "generated_files/{}_{}_{}_{}_{}_{}_{}_{}_{}_{}.rs",
            ownership.name(),
            ownership.variant().unwrap_or("default").replace('"', ""),
            infallible.name(),
            infallible.variant().unwrap_or("default"),
            async_.name(),
            async_.variant().unwrap_or("default"),
            stream_type.name(),
            stream_type.variant().unwrap_or("default").replace('"', ""),
            stream_item_infallible.name(),
            stream_item_infallible.variant().unwrap_or("default"),
        );

        std::fs::write(file_name, code.as_bytes()).unwrap();
    }
}

macro_rules! def_enum {
    (
        ($name:ident, $name_str:expr), [$( ($variant:ident, $variant_str:expr) ),* $(,)?]
    ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        enum $name {
            $($variant),*
        }

        impl $name {
            fn all() -> impl Iterator<Item = Self> {
                vec![ $($name::$variant),* ].into_iter()
            }

            fn name(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant => $name_str,
                    )*
                }
            }

            fn variant(&self) -> Option<&'static str> {
                match self {
                    $(
                        $name::$variant => $variant_str,
                    )*
                }
            }

            fn to_string(&self) -> Option<String> {
                if let Some(s) = self.variant() {
                    Some(format!("{}: {}", $name_str, s))
                } else {
                    None
                }
            }
        }
    };
}

def_enum!(
    (Ownership, "ownership"),
    [
        (Borrowed, Some("\"borrowed\"")),
        (AsRef, Some("\"as_ref\"")),
        (Owned, Some("\"owned\""))
    ]
);

def_enum!(
    (Infallible, "infallible"),
    [(True, Some("true")), (False, Some("false"))]
);

def_enum!(
    (Async, "async"),
    [(True, Some("true")), (False, Some("false"))]
);

def_enum!(
    (StreamType, "stream_type"),
    [(Default, None), (Custom, Some("\"UserStream\""))]
);

def_enum!(
    (StreamItemInfallible, "stream_item_infallible"),
    [(True, Some("true")), (False, Some("false"))]
);
