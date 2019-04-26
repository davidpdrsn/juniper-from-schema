# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

### Added

- Make default argument values work for floats, integers, strings, booleans, enumerations, and lists (of supported types). Objects, variables, and nulls are not supported.

### Changed

N/A

### Removed

N/A

### Fixed

- Field methods that return enumerations no longer get `QueryTrails`. You couldn't really do anything with them since enumerations cannot contain data.
- Schemes that don't contain a root mutation type now doesn't fail to compile. It would use `()` for the context, when it should have used `Context`.

## [0.1.4] - 2019-02-16

### Fixed

- Make `graphql_schema_from_file!` look from same folder as your `Cargo.toml`. This fixes issues with finding the schema within a [workspace](https://doc.rust-lang.org/book/second-edition/ch14-03-cargo-workspaces.html) project. Should not be a breaking change.
- Added missing `juniper::` qualifications for generated code that referenced `Executor`.
- Many documentation improvements, including:
    - Table of contents
    - Description of how to customize the error type
    - Clearer example code by removing `use juniper::*`
    - Description of how `QueryTrail` works for `Vec<T>` and `Option<T>`
    - Describe how to view the generated code
    - Several typo fixes

## [0.1.3] - 2019-02-01

### Fixed

- `QueryTrail` methods now generated for union types.

## [0.1.2] - 2019-01-14

### Fixed

- `QueryTrail` methods now generated for interface types.

## [0.1.1] - 2018-12-21

Just fixed broken homepage link on crates.io

## 0.1.0 - 2018-12-21

Initial release.

[0.1.4]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.0...0.1.1
