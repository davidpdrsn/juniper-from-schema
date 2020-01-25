# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

- Remove `MakeQueryTrail` trait. This is not a breaking change since user's shouldn't be using it.
- Require the UUID scalar type to be named `Uuid`. This is to remain consistent with the uuid crate. This is not considered a breaking change since it fixes a bug in code generation with inconsistent case. See [#104](https://github.com/davidpdrsn/juniper-from-schema/issues/104).

#### Breaking changes

None.

## [0.5.1] - 2019-11-14

- Support making fields infallible with `@juniper(infallible: true)`.
- Make sure fields on input object structs are in fact public.

## [0.5.0] - 2019-10-17

#### Breaking changes

- Remove the second lifetime from generated structs for field arguments. Turns out `'a` from `QueryTrail<'a, _, _>` is all we need.
- The `DateTime` scalar has been renamed to `DateTimeUtc` to clearly communicate the name Juniper gives it.

## [0.4.2] - 2019-10-16

- Add support for inspecting arguments to `QueryTrail`. See docs for more info. [#83](https://github.com/davidpdrsn/juniper-from-schema/pull/83).

## [0.4.1] - 2019-10-16

### Fixed

- Correctly trigger rebuild of Rust schema if only GraphQL schema changed when using `graphql_schema_from_file!`.

## [0.4.0] - 2019-10-05

### Added

- Support converting `QueryTrail`s for interfaces or unions into `QueryTrail`s for implementors of those interfaces or unions. This makes it possible to use [juniper-eager-loading](https://crates.io/crates/juniper-eager-loading) with interface or union types. [#63](https://github.com/davidpdrsn/juniper-from-schema/pull/63)

### Changed

- The `QueryTrail` type is now part of this library rather than being emitted as part of the generated code. This was done so other libraries could make sure of the type. If you're getting errors about missing methods adding `use crate::graphql_schema::query_trails::*` to you module should fix it. [#82](https://github.com/davidpdrsn/juniper-from-schema/pull/82)

## [0.3.2] - 2019-09-30

### Added

- Juniper 0.14 support.

## [0.3.1] - 2019-09-24

### Added

- Support customizing the name of the context type. [#66](https://github.com/davidpdrsn/juniper-from-schema/pull/66)
- Improve documentation of special case scalar types.
- Implement common traits for generated scalar types.
- Support converting `DateTime` into `chrono::NaiveDateTime` for those who don't care about time zones. The behavior can be customized with the `@juniper(with_time_zone: true|false)` directive on the scalar definition.

### Changed

- Special case scalars `Date`, `DateTime`, `Uuid`, and `Url` no longer support descriptions in the GraphQL schema. See [#69](https://github.com/davidpdrsn/juniper-from-schema/pull/69) for more details.

### Fixed

- Fix support for special case [`Uuid`](https://crates.io/crates/uuid) and [`Url`](https://crates.io/crates/url) scalars. [#69](https://github.com/davidpdrsn/juniper-from-schema/pull/69)
- Removed some deprecation warnings related to scalar types. [#78](https://github.com/davidpdrsn/juniper-from-schema/pull/78)
- Don't deprecate enum variants in Rust code when marked as deprecated in the schema.

## [0.3.0] - 2019-06-18

- Will now fail to compile if you schema contains field names in snake_case. [#47](https://github.com/davidpdrsn/juniper-from-schema/issues/47)

## [0.2.3] - 2019-06-15

### Fixed

- Fix `QueryTrail` walk methods always returning `false` for fields defined using snake_case in the schema. [#46](https://github.com/davidpdrsn/juniper-from-schema/pull/46)

## [0.2.2] - 2019-06-10

### Added

- Add support for "as_ref" ownership types such as `FieldResult<Option<&T>>`.

## [0.2.1] - 2019-05-24

### Changed

- Break crate into two: One that exposes shared types, one that does the code generation. Should not be a breaking change.

## [0.2.0] - 2019-05-08

### Changed

- Replace the magic `#[ownership(owned)]` comment with a schema directive.

## [0.1.7] - 2019-05-07

### Added

- Support default values for input objects.
- Support `deprecated` directives on fields and enum values.

### Fixed

- Ensure enum variants used for default values exist. This would previously be a run time error. It is now a compile time error.

## [0.1.6] - 2019-05-04

### Added

- Much better error messages. Basically a rip-off of Rust's compiler errors.

### Changed

- The `ID` GraphQL type now gets generated into `juniper::ID` instead of a custom newtype wrapper. This is a breaking change but should be straight forward to fix.

### Fixed

- Correctly panic if schema contains unsupported features such as directives or a subscription type.
- Correctly generate docs for all types.
- Fix compile error when using custom scalar.

## [0.1.5] - 2019-04-26

### Added

- Make default argument values work for floats, integers, strings, booleans, enumerations, and lists (of supported types). Objects, variables, and nulls are not supported.

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

[0.5.1]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.4.2...0.5.0
[0.4.2]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.3.2...0.4.0
[0.3.2]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.2.3...0.3.0
[0.2.3]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.7...0.2.0
[0.1.7]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/davidpdrsn/juniper-from-schema/compare/0.1.0...0.1.1
