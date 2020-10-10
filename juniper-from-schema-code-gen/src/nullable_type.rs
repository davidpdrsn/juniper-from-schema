use graphql_parser::schema::Type;

#[derive(Eq, PartialEq, Debug)]
pub enum NullableType<'a> {
    NamedType(&'a str),
    ListType(Box<NullableType<'a>>),
    NullableType(Box<NullableType<'a>>),
}

impl<'a> NullableType<'a> {
    pub fn from_schema_type(ty: &Type<'a, &'a str>) -> Self {
        map(&ty)
    }
}

#[cfg(test)]
impl<'a> NullableType<'a> {
    fn debug_print(&self) -> String {
        match self {
            NullableType::NamedType(name) => (*name).to_string(),
            NullableType::ListType(inner) => format!("List({})", inner.debug_print()),
            NullableType::NullableType(inner) => format!("Nullable({})", inner.debug_print()),
        }
    }
}

fn map<'a>(ty: &Type<'a, &'a str>) -> NullableType<'a> {
    match ty {
        inner @ Type::NamedType(_) => map_inner(inner, false),
        Type::ListType(item_type) => {
            let item_type = map_inner(&*item_type, false);
            let list = NullableType::ListType(Box::new(item_type));
            NullableType::NullableType(Box::new(list))
        }
        Type::NonNullType(inner) => map_inner(&*inner, true),
    }
}

fn map_inner<'a>(ty: &Type<'a, &'a str>, inside_non_null: bool) -> NullableType<'a> {
    match ty {
        Type::NamedType(name) => {
            let inner_mapped = NullableType::NamedType(&name);
            if inside_non_null {
                inner_mapped
            } else {
                NullableType::NullableType(Box::new(inner_mapped))
            }
        }
        Type::ListType(inner) => {
            let inner_mapped = NullableType::ListType(Box::new(map(&*inner)));
            if inside_non_null {
                inner_mapped
            } else {
                NullableType::NullableType(Box::new(inner_mapped))
            }
        }
        Type::NonNullType(inner) => map_inner(&*inner, true),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn named_type() {
        let input = Type::NonNullType(Box::new(Type::NamedType("Int")));
        let expected = "Int".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NamedType("Int");
        let expected = "Nullable(Int)".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NonNullType(
            Box::new(Type::NamedType("Int")),
        )))));
        let expected = "List(Int)".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::ListType(Box::new(Type::NonNullType(Box::new(Type::NamedType(
            "Int",
        )))));
        let expected = "Nullable(List(Int))".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NamedType("Int")))));
        let expected = "List(Nullable(Int))".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::ListType(Box::new(Type::NamedType("Int")));
        let expected = "Nullable(List(Nullable(Int)))".to_string();
        assert_eq!(map(&input).debug_print(), expected);
    }
}
