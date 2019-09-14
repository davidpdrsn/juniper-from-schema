use colored::*;
use graphql_parser::Pos;
use std::fmt::{self, Write};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Error<'doc> {
    pub(super) pos: Pos,
    pub(super) kind: ErrorKind<'doc>,
    pub(super) raw_schema: &'doc str,
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Handle lines that are really long and cause wrapping (screenshot on desktop)
        // TODO: Seems to be issues with multiline comments (screenshot on desktop)

        let schema_lines = self.raw_schema.lines().collect::<Vec<_>>();

        let number_of_digits_in_line_count = number_of_digits(self.pos.line as i32);
        let indent = 4;

        writeln!(
            f,
            "{error}: {kind}",
            error = "error".bright_red(),
            kind = self.kind.description()
        )?;
        writeln!(
            f,
            "{indent} --> schema:{line}:{col}",
            indent = "".indent(number_of_digits_in_line_count - 1),
            line = self.pos.line,
            col = self.pos.column
        )?;
        writeln!(f, "{} |", "".indent(number_of_digits_in_line_count))?;
        writeln!(
            f,
            "{} |{}",
            self.pos.line,
            schema_lines[self.pos.line - 1].indent(indent),
        )?;
        writeln!(
            f,
            "{} |{}{}",
            "".indent(number_of_digits_in_line_count),
            "".indent(self.pos.column - 1 + indent),
            "^".bright_red(),
        )?;

        if let Some(notes) = self.kind.notes() {
            writeln!(f)?;
            for line in notes.lines() {
                writeln!(f, "{}", line)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorKind<'doc> {
    DateTimeScalarNotDefined,
    DateScalarNotDefined,
    UuidScalarNotDefined,
    UrlScalarNotDefined,
    SpecialCaseScalarWithDescription,
    DirectivesNotSupported,
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
    InvalidArgumentsToDeprecateDirective,
    InvalidArgumentsToJuniperDirective,
    AsRefOwnershipForNamedType,
    FieldNameInSnakeCase,
}

impl<'doc> ErrorKind<'doc> {
    fn description(&self) -> String {
        match self {
            ErrorKind::DateTimeScalarNotDefined => {
                "You have to define a custom scalar called `DateTime` to use this type".to_string()
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
            ErrorKind::DirectivesNotSupported => {
                "Directives are currently not supported".to_string()
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
            ErrorKind::InvalidArgumentsToDeprecateDirective => {
                "Invalid arguments passed to @deprecated".to_string()
            }
            ErrorKind::InvalidArgumentsToJuniperDirective => {
                "Invalid arguments passed to @juniper".to_string()
            }
            ErrorKind::AsRefOwnershipForNamedType => {
                "@juniper(ownership: \"as_ref\") is only supported on `Option` and `Vec` types"
                    .to_string()
            }
            ErrorKind::FieldNameInSnakeCase => {
                "Field names must be camelCase, not snake_case".to_string()
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
                Some("Insert `scalar DateTime` into your schema".to_string())
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
            ErrorKind::InvalidArgumentsToDeprecateDirective => {
                Some("It takes 0 or 1 argument and the argument most be named `reason` and be of type `String`".to_string())
            }
            ErrorKind::InvalidArgumentsToJuniperDirective => {
                let mut f = String::new();
                writeln!(f, "It takes exactly 1 argument and the argument most be named `ownership`");
                writeln!(f, "and be either \"owned\", \"borrowed\", or \"as_ref\"");
                Some(f)
            }
            ErrorKind::FieldNameInSnakeCase => {
                Some("This is because Juniper always converts all field names to camelCase".to_string())
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
            return self.to_string();
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
