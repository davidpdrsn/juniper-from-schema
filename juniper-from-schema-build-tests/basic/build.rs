fn main() {
    juniper_from_schema_build::compile_schema_literal(
        r#"
        schema {
            query: Query
        }

        type Query {
            ping: Boolean!
        }
    "#,
    )
    .unwrap();
}
