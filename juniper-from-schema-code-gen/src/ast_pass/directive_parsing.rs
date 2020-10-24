use crate::ast_pass::{
    code_gen_pass::CodeGenPass,
    error::{self, ErrorKind, Juniper, UnsupportedDirectiveKind, ValueType},
    EmitError,
};
use graphql_parser::{query::Value, schema::*};
use std::convert::identity;

pub trait FromDirective: Sized {
    fn from_directive<'doc>(dir: &'doc Directive<'doc, &'doc str>) -> Result<Self, ErrorKind>;
}

pub trait FromDirectiveArguments: Sized + Default {
    const KEY: &'static str;

    fn from_directive_args<'doc>(
        args: &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Self, ErrorKind>>;
}

impl<T: FromDirectiveArguments> FromDirectiveArguments for Option<T> {
    const KEY: &'static str = T::KEY;

    fn from_directive_args<'doc>(
        args: &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Option<T>, ErrorKind>> {
        match T::from_directive_args(args) {
            // KEY didn't match
            None => None,
            Some(x) => {
                // KEY *did* match
                Some(x.map(Some))
            }
        }
    }
}

#[derive(Debug)]
pub enum Deprecation {
    NoDeprecation,
    Deprecated(Option<String>),
}

impl Default for Deprecation {
    fn default() -> Self {
        Self::NoDeprecation
    }
}

impl FromDirective for Deprecation {
    fn from_directive<'doc>(dir: &'doc Directive<'doc, &'doc str>) -> Result<Self, ErrorKind> {
        let name = &dir.name;
        if *name != "deprecated" {
            return Err(ErrorKind::UnsupportedDirective(
                UnsupportedDirectiveKind::Deprecation(error::Deprecation::InvalidName(
                    name.to_string(),
                )),
            ));
        }

        if dir.arguments.len() > 1 {
            return Err(ErrorKind::UnsupportedDirective(
                UnsupportedDirectiveKind::Deprecation(error::Deprecation::WrongNumberOfArgs(
                    dir.arguments.len(),
                )),
            ));
        }

        if let Some((key, value)) = &dir.arguments.first() {
            if *key != "reason" {
                return Err(ErrorKind::UnsupportedDirective(
                    UnsupportedDirectiveKind::Deprecation(error::Deprecation::InvalidKey(
                        key.to_string(),
                    )),
                ));
            }

            let reason = match value {
                Value::String(s) => s.to_string(),
                other => {
                    return Err(ErrorKind::UnsupportedDirective(
                        UnsupportedDirectiveKind::InvalidType {
                            expected: ValueType::String,
                            actual: ValueType::from(other),
                        },
                    ));
                }
            };

            Ok(Deprecation::Deprecated(Some(reason)))
        } else {
            Ok(Deprecation::Deprecated(None))
        }
    }
}

#[derive(Debug)]
pub struct JuniperDirective<T> {
    pub name: String,
    pub args: T,
}

impl<T: Default> Default for JuniperDirective<T> {
    fn default() -> Self {
        Self {
            name: "juniper".to_string(),
            args: T::default(),
        }
    }
}

macro_rules! impl_from_directive_for {
    {
        ( $( $name:ident ),* )
    } => {
        #[allow(unused_parens)]
        impl<$($name),*> FromDirective for JuniperDirective<($($name),*)>
        where
            $($name: FromDirectiveArguments,)*
        {
            #[allow(non_snake_case)]
            fn from_directive<'doc>(dir: &'doc Directive<'doc, &'doc str>) -> Result<Self, ErrorKind> {
                let name = &dir.name;
                if *name != "juniper" {
                    return Err(ErrorKind::UnsupportedDirective(
                        UnsupportedDirectiveKind::Juniper(Juniper::InvalidName(name.to_string())),
                    ));
                }

                $(let mut $name = None::<$name>;)*

                let mut args: Vec<Option<&'doc (&'doc str, Value<'doc, &'doc str>)>> = dir.arguments.iter().map(Some).collect();

                for idx in 0..args.len() {
                    let arg = &args[idx].unwrap();

                    $(
                        if let Some(arg) = $name::from_directive_args(arg) {
                            $name.replace(arg?);
                            args[idx] = None;
                            continue;
                        }
                    )*
                }

                let unknown_args = args.into_iter().filter_map(identity).collect::<Vec<_>>();

                if !unknown_args.is_empty() {
                    let arg_names = unknown_args
                        .iter()
                        .map(|(name, _)| name.to_string())
                        .collect::<Vec<String>>();
                    return Err(ErrorKind::UnknownDirective { suggestions: arg_names });
                }

                $(let $name = $name.unwrap_or_else($name::default);)*

                Ok(Self {
                    name: name.to_string(),
                    args: ($($name),*),
                })
            }
        }
    };
}

impl_from_directive_for! { (T) }
impl_from_directive_for! { (T1, T2) }
impl_from_directive_for! { (T1, T2, T3) }
impl_from_directive_for! { (T1, T2, T3, T4) }
impl_from_directive_for! { (T1, T2, T3, T4, T5) }

#[derive(Debug)]
pub struct FieldDirectives {
    pub ownership: Ownership,
    pub deprecated: Option<Deprecation>,
    pub infallible: Infallible,
    pub r#async: Async,
    pub stream_type: Option<StreamType>,
    pub stream_item_infallible: Option<StreamItemInfallible>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Ownership {
    Owned,
    Borrowed,
    AsRef,
}

impl Ownership {
    pub fn is_as_ref(&self) -> bool {
        matches!(self, Ownership::AsRef)
    }
}

impl Default for Ownership {
    fn default() -> Self {
        Self::Borrowed
    }
}

impl FromDirectiveArguments for Ownership {
    const KEY: &'static str = "ownership";

    fn from_directive_args<'doc>(
        (key, value): &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Self, ErrorKind>> {
        if *key != Self::KEY {
            return None;
        }

        let directive = (|| {
            let ownership_raw = value_as_string(value)?;

            let ownership = match ownership_raw {
                "owned" => Ownership::Owned,
                "borrowed" => Ownership::Borrowed,
                "as_ref" => Ownership::AsRef,
                value => {
                    return Err(ErrorKind::UnsupportedDirective(
                        UnsupportedDirectiveKind::Ownership(error::Ownership::InvalidValue(
                            value.to_string(),
                        )),
                    ));
                }
            };

            Ok(ownership)
        })();
        Some(directive)
    }
}

#[derive(Debug)]
pub struct Infallible {
    pub value: bool,
}

impl Default for Infallible {
    fn default() -> Self {
        Infallible { value: false }
    }
}

impl FromDirectiveArguments for Infallible {
    const KEY: &'static str = "infallible";

    fn from_directive_args<'doc>(
        (key, value): &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Self, ErrorKind>> {
        if *key != Self::KEY {
            return None;
        }

        let directive = (|| {
            let value = value_as_bool(value)?;
            Ok(Self { value })
        })();

        Some(directive)
    }
}

#[derive(Debug)]
pub struct Async {
    pub value: bool,
}

impl FromDirectiveArguments for Async {
    const KEY: &'static str = "async";

    fn from_directive_args<'doc>(
        (key, value): &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Self, ErrorKind>> {
        if *key != Self::KEY {
            return None;
        }

        let directive = (|| {
            let value = value_as_bool(value)?;
            Ok(Self { value })
        })();

        Some(directive)
    }
}

impl Default for Async {
    fn default() -> Self {
        Async { value: false }
    }
}

#[derive(Debug, Default)]
pub struct StreamType {
    pub value: String,
}

impl FromDirectiveArguments for StreamType {
    const KEY: &'static str = "stream_type";

    fn from_directive_args<'doc>(
        (key, value): &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Self, ErrorKind>> {
        if *key != Self::KEY {
            return None;
        }

        let directive = (|| {
            let value = value_as_string(value)?;
            Ok(Self {
                value: value.to_string(),
            })
        })();

        Some(directive)
    }
}

#[derive(Debug, Default)]
pub struct StreamItemInfallible {
    pub value: bool,
}

impl FromDirectiveArguments for StreamItemInfallible {
    const KEY: &'static str = "stream_item_infallible";

    fn from_directive_args<'doc>(
        (key, value): &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Self, ErrorKind>> {
        if *key != Self::KEY {
            return None;
        }

        let directive = (|| {
            let value = value_as_bool(value)?;
            Ok(Self { value })
        })();

        Some(directive)
    }
}

#[derive(Debug)]
pub struct DateTimeScalarArguments {
    pub with_time_zone: bool,
}

impl Default for DateTimeScalarArguments {
    fn default() -> Self {
        DateTimeScalarArguments {
            with_time_zone: true,
        }
    }
}

impl FromDirectiveArguments for DateTimeScalarArguments {
    const KEY: &'static str = "with_time_zone";

    fn from_directive_args<'doc>(
        (key, value): &'doc (&'doc str, Value<'doc, &'doc str>),
    ) -> Option<Result<Self, ErrorKind>> {
        if *key != Self::KEY {
            return None;
        }

        let directive = (|| {
            let with_time_zone = value_as_bool(value)?;
            Ok(Self { with_time_zone })
        })();

        Some(directive)
    }
}

fn value_as_string<'doc>(value: &'doc Value<'doc, &'doc str>) -> Result<&'doc str, ErrorKind> {
    match value {
        Value::String(x) => Ok(x),
        other => Err(ErrorKind::UnsupportedDirective(
            UnsupportedDirectiveKind::InvalidType {
                expected: ValueType::String,
                actual: ValueType::from(other),
            },
        )),
    }
}

fn value_as_bool<'doc>(value: &'doc Value<'doc, &'doc str>) -> Result<bool, ErrorKind> {
    match value {
        Value::Boolean(x) => Ok(*x),
        other => Err(ErrorKind::UnsupportedDirective(
            UnsupportedDirectiveKind::InvalidType {
                expected: ValueType::Boolean,
                actual: ValueType::from(other),
            },
        )),
    }
}

pub trait ParseDirective<T> {
    type Output;

    fn parse_directives(&mut self, input: T) -> Self::Output;
}

impl<'doc> ParseDirective<&'doc Field<'doc, &'doc str>> for CodeGenPass<'doc> {
    type Output = FieldDirectives;

    fn parse_directives(&mut self, input: &'doc Field<'doc, &'doc str>) -> Self::Output {
        let mut ownership = Ownership::default();
        let mut deprecated = None::<Deprecation>;
        let mut infallible = Infallible::default();
        let mut r#async = Async::default();
        let mut stream_type = None::<StreamType>;
        let mut stream_item_infallible = None::<StreamItemInfallible>;

        for dir in &input.directives {
            if let Ok(juniper_directive) = JuniperDirective::<(
                Ownership,
                Infallible,
                Async,
                Option<StreamType>,
                Option<StreamItemInfallible>,
            )>::from_directive(dir)
            {
                ownership = juniper_directive.args.0;
                infallible = juniper_directive.args.1;
                r#async = juniper_directive.args.2;
                stream_type = juniper_directive.args.3;
                stream_item_infallible = juniper_directive.args.4;
                continue;
            }

            if let Ok(x) = Deprecation::from_directive(dir) {
                deprecated = Some(x);
                continue;
            }

            self.emit_error(
                dir.position,
                ErrorKind::UnknownDirective {
                    suggestions: vec![],
                },
            );
        }

        FieldDirectives {
            ownership,
            deprecated,
            infallible,
            r#async,
            stream_type,
            stream_item_infallible,
        }
    }
}

impl<'doc> ParseDirective<&'doc EnumValue<'doc, &'doc str>> for CodeGenPass<'doc> {
    type Output = Deprecation;

    fn parse_directives(&mut self, input: &'doc EnumValue<'doc, &'doc str>) -> Self::Output {
        let mut deprecated = Deprecation::default();

        for dir in &input.directives {
            match Deprecation::from_directive(dir) {
                Ok(x) => {
                    deprecated = x;
                }
                Err(err) => {
                    self.emit_error(dir.position, err);
                }
            }
        }

        deprecated
    }
}

#[derive(Debug)]
pub struct DateTimeScalarType<'a>(pub &'a ScalarType<'a, &'a str>);

impl<'doc, T> ParseDirective<DateTimeScalarType<'doc>> for T
where
    T: EmitError,
{
    type Output = DateTimeScalarArguments;

    fn parse_directives(&mut self, input: DateTimeScalarType<'doc>) -> Self::Output {
        let mut args = DateTimeScalarArguments::default();

        for dir in &input.0.directives {
            match JuniperDirective::<DateTimeScalarArguments>::from_directive(dir) {
                Ok(x) => {
                    args = x.args;
                }
                Err(err) => {
                    self.emit_error(dir.position, err);
                }
            }
        }

        args
    }
}

macro_rules! supports_no_directives {
    ($ty:ty) => {
        impl<'doc> ParseDirective<&'doc $ty> for CodeGenPass<'doc> {
            type Output = ();

            fn parse_directives(&mut self, input: &'doc $ty) -> Self::Output {
                for directive in &input.directives {
                    self.emit_error(
                        directive.position,
                        ErrorKind::UnknownDirective {
                            suggestions: vec![],
                        },
                    );
                }
            }
        }
    };
}

supports_no_directives!(SchemaDefinition<'doc, &'doc str>);
supports_no_directives!(ScalarType<'doc, &'doc str>);
supports_no_directives!(ObjectType<'doc, &'doc str>);
supports_no_directives!(InterfaceType<'doc, &'doc str>);
supports_no_directives!(UnionType<'doc, &'doc str>);
supports_no_directives!(EnumType<'doc, &'doc str>);
supports_no_directives!(InputObjectType<'doc, &'doc str>);
supports_no_directives!(InputValue<'doc, &'doc str>);
