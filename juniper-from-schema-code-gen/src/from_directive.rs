use crate::ast_pass::error::{self, ErrorKind, Juniper, UnsupportedDirectiveKind, ValueType};
use graphql_parser::{query::Value, schema::Directive};

pub trait FromDirective: Sized {
    fn from_directive(dir: &Directive) -> Result<Self, ErrorKind>;
}

pub trait FromDirectiveArguments: Sized {
    fn from_directive_args(args: &[(String, Value)]) -> Result<Self, ErrorKind>;
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
    fn from_directive(dir: &Directive) -> Result<Self, ErrorKind> {
        let name = &dir.name;
        if name != "deprecated" {
            return Err(ErrorKind::UnsupportedDirective(
                UnsupportedDirectiveKind::Deprecation(error::Deprecation::InvalidName(name)),
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
            if key != "reason" {
                return Err(ErrorKind::UnsupportedDirective(
                    UnsupportedDirectiveKind::Deprecation(error::Deprecation::InvalidKey(key)),
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

impl<T: FromDirectiveArguments> FromDirective for JuniperDirective<T> {
    fn from_directive(dir: &Directive) -> Result<Self, ErrorKind> {
        let name = &dir.name;
        if name != "juniper" {
            return Err(ErrorKind::UnsupportedDirective(
                UnsupportedDirectiveKind::Juniper(Juniper::InvalidName(name)),
            ));
        }

        let args = T::from_directive_args(&dir.arguments)?;

        Ok(Self {
            name: name.to_string(),
            args,
        })
    }
}

#[derive(Debug)]
pub struct FieldArguments {
    pub ownership: Ownership,
    pub deprecated: Option<Deprecation>,
}

#[derive(Debug)]
pub enum Ownership {
    Owned,
    Borrowed,
    AsRef,
}

impl Default for Ownership {
    fn default() -> Self {
        Self::Borrowed
    }
}

impl FromDirectiveArguments for Ownership {
    fn from_directive_args(args: &[(String, Value)]) -> Result<Self, ErrorKind> {
        if args.len() != 1 {
            return Err(ErrorKind::UnsupportedDirective(
                UnsupportedDirectiveKind::Ownership(error::Ownership::WrongNumberOfArgs(
                    args.len(),
                )),
            ));
        }
        let arg = &args[0];

        let key = &arg.0;
        if key != "ownership" {
            return Err(ErrorKind::UnsupportedDirective(
                UnsupportedDirectiveKind::Ownership(error::Ownership::InvalidKey(key)),
            ));
        }

        let value = &arg.1;

        let ownership_raw = value_as_string(value)?;

        let ownership = match ownership_raw.as_ref() {
            "owned" => Ownership::Owned,
            "borrowed" => Ownership::Borrowed,
            "as_ref" => Ownership::AsRef,
            value => {
                return Err(ErrorKind::UnsupportedDirective(
                    UnsupportedDirectiveKind::Ownership(error::Ownership::InvalidValue(value)),
                ));
            }
        };

        Ok(ownership)
    }
}

fn value_as_string(value: &Value) -> Result<&str, ErrorKind> {
    match value {
        Value::String(x) => Ok(x),
        other => {
            return Err(ErrorKind::UnsupportedDirective(
                UnsupportedDirectiveKind::InvalidType {
                    expected: ValueType::String,
                    actual: ValueType::from(other),
                },
            ));
        }
    }
}
