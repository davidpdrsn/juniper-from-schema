# [juniper-from-schema](https://crates.io/crates/juniper-from-schema)
[![crates.io](https://meritbadge.herokuapp.com/juniper-from-schema)](https://crates.io/crates/juniper-from-schema)
[![Documentation](https://docs.rs/juniper-from-schema/badge.svg)](https://docs.rs/juniper-from-schema)


This library contains a procedural macro that reads a GraphQL schema file, and generates the
corresponding [Juniper](https://crates.io/crates/juniper) [macro calls]. This means you can
have a real schema file and be guaranteed that it matches your Rust implementation. It also
removes most of the boilerplate involved in using Juniper.

[macro calls]: https://graphql-rust.github.io/types/objects/complex_fields.html

See the [crate documentation](https://docs.rs/juniper-from-schema/) for a usage examples and more info.
