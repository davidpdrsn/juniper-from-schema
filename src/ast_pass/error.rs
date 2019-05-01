use graphql_parser::Pos;
use std::fmt;

#[derive(Debug)]
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

        writeln!(f, "error: {kind}", kind = self.kind.description())?;
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
            "{} |{}^",
            "".indent(number_of_digits_in_line_count),
            "".indent(self.pos.column - 1 + indent),
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

#[derive(Debug)]
pub enum ErrorKind<'doc> {
    DirectivesNotSupported,
    NoQueryType,
    NonnullableFieldWithDefaultValue,
    NullDefaultValue,
    ObjectArgumentWithDefaultValue,
    SubscriptionsNotSupported,
    TypeExtensionNotSupported,
    UnsupportedAttribute(&'doc str),
    UnsupportedAttributePair(&'doc str, &'doc str),
    VariableDefaultValue,
}

impl<'doc> ErrorKind<'doc> {
    fn description(&self) -> String {
        use ErrorKind::*;

        match self {
            DirectivesNotSupported => "Directives are currently not supported".to_string(),
            SubscriptionsNotSupported => "Subscriptions are currently not supported".to_string(),
            NoQueryType => "Schema doesn't have root a Query type".to_string(),
            NonnullableFieldWithDefaultValue => {
                "Fields with default arguments values must be nullable".to_string()
            },
            UnsupportedAttribute(attr) => {
                format!("The attribute {} is unsupported", attr)
            }
            UnsupportedAttributePair(attr, value) => {
                format!("Unsupported attribute value '{}' for attribute '{}'", value, attr)
            }
            ObjectArgumentWithDefaultValue => {
                "Default arguments where the type is an object is currently not supported".to_string()
            }
            NullDefaultValue => {
                "Having a default argument value of `null` is not supported. Use a nullable type instead".to_string()
            }
            VariableDefaultValue => {
                "Default arguments cannot refer to variables".to_string()
            }
            TypeExtensionNotSupported => {
                "Type extentions are not supported".to_string()
            }
        }
    }

    fn notes(&self) -> Option<String> {
        use self::ErrorKind::*;

        match self {
            SubscriptionsNotSupported => Some(
                "Subscriptions are currently not supported by Juniper so we're unsure when\nor if we'll support them"
                    .to_string(),
            ),
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

    let n = n as f64;
    f64::floor(f64::log10(n)) as usize + 1
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
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
