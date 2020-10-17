use colored::*;
use graphql_parser::{query::Value, Pos};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Error<'doc> {
    pub(super) pos: Pos,
    pub(super) kind: ErrorKind<'doc>,
}

impl<'doc> Error<'doc> {
    pub fn display(self, raw_schema: &'doc str) -> ErrorDisplay<'doc> {
        ErrorDisplay {
            error: self,
            raw_schema,
        }
    }
}

#[derive(Debug)]
pub struct ErrorDisplay<'doc> {
    error: Error<'doc>,
    raw_schema: &'doc str,
}

impl<'a> fmt::Display for ErrorDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Handle lines that are really long and cause wrapping (screenshot on desktop)
        // TODO: Seems to be issues with multiline comments (screenshot on desktop)

        let schema_lines = self.raw_schema.lines().collect::<Vec<_>>();

        let number_of_digits_in_line_count = number_of_digits(self.error.pos.line as i32);
        let indent = 4;

        writeln!(
            f,
            "{error}: {kind}",
            error = "error".bright_red(),
            kind = self.error.kind.description()
        )?;
        writeln!(
            f,
            "{indent} --> schema:{line}:{col}",
            indent = "".indent(number_of_digits_in_line_count - 1),
            line = self.error.pos.line,
            col = self.error.pos.column
        )?;
        writeln!(f, "{} |", "".indent(number_of_digits_in_line_count))?;
        writeln!(
            f,
            "{} |{}",
            self.error.pos.line,
            schema_lines[self.error.pos.line - 1].indent(indent),
        )?;
        writeln!(
            f,
            "{} |{}{}",
            "".indent(number_of_digits_in_line_count),
            "".indent(self.error.pos.column - 1 + indent),
            "^".bright_red(),
        )?;

        if let Some(notes) = self.error.kind.notes() {
            writeln!(f)?;
            for line in notes.lines() {
                writeln!(f, "{}", line)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Deprecation<'doc> {
    InvalidName(&'doc str),
    WrongNumberOfArgs(usize),
    InvalidKey(&'doc str),
}

impl<'doc> fmt::Display for Deprecation<'doc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Deprecation::*;
        match self {
            InvalidName(name) => write!(f, "Invalid name. Expected `deprecated`, got `{}`", name),
            WrongNumberOfArgs(count) => {
                write!(f, "Wrong number of args. Expected 0 or 1, got `{}`", count)
            }
            InvalidKey(key) => write!(f, "Invalid key. Exptec `reason`, got `{}`", key),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ValueType {
    Variable,
    Int,
    Float,
    String,
    Boolean,
    Null,
    Enum,
    List,
    Object,
}

impl<'doc> From<&'doc Value<'doc, &'doc str>> for ValueType {
    fn from(value: &'doc Value<'doc, &'doc str>) -> Self {
        match value {
            Value::String(_) => ValueType::String,
            Value::Variable(_) => ValueType::Variable,
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Null => ValueType::Null,
            Value::Enum(_) => ValueType::Enum,
            Value::List(_) => ValueType::List,
            Value::Object(_) => ValueType::Object,
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValueType::*;
        match self {
            String => write!(f, "String"),
            Variable => write!(f, "Variable"),
            Int => write!(f, "Int"),
            Float => write!(f, "Float"),
            Boolean => write!(f, "Boolean"),
            Null => write!(f, "Null"),
            Enum => write!(f, "Enum"),
            List => write!(f, "List"),
            Object => write!(f, "Object"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ownership<'doc> {
    InvalidValue(&'doc str),
}

impl<'doc> fmt::Display for Ownership<'doc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidValue(name) => write!(
                f,
                "Invalid value. Expected `owned`, `borrowed`, or `as_ref`, got `{}`",
                name
            ),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Juniper<'doc> {
    InvalidName(&'doc str),
}

impl<'doc> fmt::Display for Juniper<'doc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidName(name) => write!(f, "Invalid name `{}`. Expected `juniper`", name),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UnsupportedDirectiveKind<'doc> {
    Deprecation(Deprecation<'doc>),
    Ownership(Ownership<'doc>),
    Juniper(Juniper<'doc>),
    InvalidType {
        actual: ValueType,
        expected: ValueType,
    },
}

impl<'doc> fmt::Display for UnsupportedDirectiveKind<'doc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Deprecation(inner) => write!(f, "{}", inner),
            Self::Ownership(inner) => write!(f, "{}", inner),
            Self::Juniper(inner) => write!(f, "{}", inner),
            Self::InvalidType { expected, actual } => {
                write!(f, "Invalid type. Expected `{}`, got `{}`", expected, actual)
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorKind<'doc> {
    DateTimeScalarNotDefined,
    DateScalarNotDefined,
    UuidScalarNotDefined,
    UrlScalarNotDefined,
    SpecialCaseScalarWithDescription,
    UnsupportedDirective(UnsupportedDirectiveKind<'doc>),
    UnknownDirective {
        suggestions: Vec<String>,
    },
    NoQueryType,
    NonnullableFieldWithDefaultValue,
    SubscriptionsNotSupported,
    TypeExtensionNotSupported,
    UnionFieldTypeMismatch {
        union_name: &'doc str,
        field_name: &'doc str,
        type_a: &'doc str,
        field_type_a: &'doc str,
        type_b: &'doc str,
        field_type_b: &'doc str,
    },
    VariableDefaultValue,
    InputTypeFieldWithDefaultValue,
    AsRefOwnershipForNamedType,
    FieldNameInSnakeCase,
    UppercaseUuidScalar,
    InvalidJuniperDirective(String, Option<String>),
    CannotDeclareBuiltinAsScalar,
}

impl<'doc> ErrorKind<'doc> {
    fn description(&self) -> String {
        match self {
            ErrorKind::DateTimeScalarNotDefined => {
                "You have to define a custom scalar called `DateTimeUtc` to use this type".to_string()
            }
            ErrorKind::DateScalarNotDefined => {
                "You have to define a custom scalar called `Date` to use this type".to_string()
            }
            ErrorKind::UuidScalarNotDefined => {
                "You have to define a custom scalar called `Uuid` to use this type".to_string()
            }
            ErrorKind::UrlScalarNotDefined => {
                "You have to define a custom scalar called `Url` to use this type".to_string()
            }
            ErrorKind::SpecialCaseScalarWithDescription => {
                "Special case scalars don't support having descriptions because the Rust types are defined in external crates".to_string()
            }
            ErrorKind::UnsupportedDirective(_) => {
                "Unsupported directive.".to_string()
            }
            ErrorKind::UnknownDirective { suggestions: _ } => {
                "Unknown directive".to_string()
            }
            ErrorKind::SubscriptionsNotSupported => {
                "Subscriptions are currently not supported".to_string()
            }
            ErrorKind::NoQueryType => "Schema doesn't have root a Query type".to_string(),
            ErrorKind::NonnullableFieldWithDefaultValue => {
                "Fields with default arguments values must be nullable".to_string()
            }
            ErrorKind::VariableDefaultValue => {
                "Default arguments cannot refer to variables".to_string()
            }
            ErrorKind::TypeExtensionNotSupported => "Type extentions are not supported".to_string(),
            ErrorKind::UnionFieldTypeMismatch { union_name, .. } => format!(
                "Error while generating `QueryTrail` for union `{}`",
                union_name
            ),
            ErrorKind::InputTypeFieldWithDefaultValue => {
                "Default values for input type fields are not supported".to_string()
            }
            ErrorKind::AsRefOwnershipForNamedType => {
                "@juniper(ownership: \"as_ref\") is only supported on `Option` and `Vec` types"
                    .to_string()
            }
            ErrorKind::FieldNameInSnakeCase => {
                "Field names must be camelCase, not snake_case".to_string()
            }
            ErrorKind::UppercaseUuidScalar => {
                "The UUID must be named `Uuid`".to_string()
            }
            ErrorKind::InvalidJuniperDirective(msg, _) => {
                msg.clone()
            }
            ErrorKind::CannotDeclareBuiltinAsScalar => {
                "You cannot declare scalars with names matching a built-in".to_string()
            }
        }
    }

    #[allow(unused_must_use)]
    fn notes(&self) -> Option<String> {
        match self {
            ErrorKind::SubscriptionsNotSupported => Some(
                "Subscriptions are currently not supported by Juniper so we're unsure when\nor if we'll support them"
                    .to_string(),
            ),
            ErrorKind::UnionFieldTypeMismatch { union_name, field_name, type_a, type_b, field_type_a, field_type_b } => {
                let mut f = String::new();

                writeln!(f, "`{}.{}` and `{}.{}` are not the same type", type_a, field_name, type_b, field_name);
                writeln!(f, "    `{}.{}` is of type `{}`", type_a, field_name, field_type_a);
                writeln!(f, "    `{}.{}` is of type `{}`", type_b, field_name, field_type_b);
                writeln!(f, "That makes it impossible to generate code for the method `QueryTrail<_, {}, _>::{}()`", union_name, field_name);
                writeln!(f, "It would have to return `{}` if `{}` is `{},` but `{}` if it is a `{}`", field_type_a, union_name, type_a, field_type_b, type_b);

                Some(f)
            }
            ErrorKind::DateTimeScalarNotDefined => {
                Some("Insert `scalar DateTimeUtc` into your schema".to_string())
            }
            ErrorKind::DateScalarNotDefined => {
                Some("Insert `scalar Date` into your schema".to_string())
            }
            ErrorKind::InputTypeFieldWithDefaultValue => {
                let mut f = String::new();
                writeln!(f, "Consider using default field arguments instead");
                writeln!(f);
                writeln!(f, "It is not supported because the spec isn't clear");
                writeln!(f, "about what should happen when there are defaults");
                writeln!(f, "in both the input type definition and field argument");
                writeln!(f);
                writeln!(f, "See https://github.com/webonyx/graphql-php/issues/350 for an example");
                Some(f)
            }
            ErrorKind::FieldNameInSnakeCase => {
                Some("This is because Juniper always converts all field names to camelCase".to_string())
            }
            ErrorKind::UnsupportedDirective(reason) => {
                Some(format!("{}", reason))
            }
            ErrorKind::UnknownDirective { suggestions } => {
                if suggestions.is_empty() {
                    None
                } else {
                    Some(format!("Did you mean: {}?", suggestions.join(", ")))
                }
            }
            ErrorKind::UppercaseUuidScalar => {
                Some("This is to be consistent with the naming the \"uuid\" crate".to_string())
            }
            ErrorKind::InvalidJuniperDirective(_, notes) => {
                notes.to_owned()
            }
            _ => None,
        }
    }
}

trait Indent {
    fn indent(&self, size: usize) -> String;
}

impl Indent for &str {
    fn indent(&self, size: usize) -> String {
        if size == 0 {
            return (*self).to_string();
        }

        let mut out = String::new();
        for _ in 0..size {
            out.push(' ');
        }
        out.push_str(self);
        out
    }
}

fn number_of_digits(n: i32) -> usize {
    if n == 0 {
        return 1;
    }

    let n = f64::from(n);
    f64::floor(f64::log10(n)) as usize + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_digits() {
        assert_eq!(1, number_of_digits(0));
        assert_eq!(1, number_of_digits(1));
        assert_eq!(1, number_of_digits(4));
        assert_eq!(2, number_of_digits(10));
        assert_eq!(7, number_of_digits(1_000_000));
    }
}
