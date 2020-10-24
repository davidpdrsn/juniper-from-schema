fn main() {
    juniper_from_schema_build::configure_for_file("schema.graphql")
        .context_type("()")
        .error_type("MyError")
        .compile()
        .unwrap();
}
