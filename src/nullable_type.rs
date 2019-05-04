use graphql_parser::schema::{Name, Type};

#[derive(Eq, PartialEq, Debug)]
pub enum NullableType<'a> {
    NamedType(&'a Name),
    ListType(Box<NullableType<'a>>),
    NullableType(Box<NullableType<'a>>),
}

impl<'a> NullableType<'a> {
    pub fn from_schema_type(type_: &'a Type) -> Self {
        map(&type_)
    }

    pub fn remove_one_layer_of_nullability(self) -> Self {
        match self {
            ty @ NullableType::NamedType(_) => ty,
            ty @ NullableType::ListType(_) => ty,
            NullableType::NullableType(inner) => *inner,
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            NullableType::NamedType(_) => false,
            NullableType::ListType(_) => false,
            NullableType::NullableType(_) => true,
        }
    }
}

#[cfg(test)]
impl<'a> NullableType<'a> {
    fn debug_print(&self) -> String {
        match self {
            NullableType::NamedType(name) => format!("{}", name),
            NullableType::ListType(inner) => format!("List({})", inner.debug_print()),
            NullableType::NullableType(inner) => format!("Nullable({})", inner.debug_print()),
        }
    }
}

fn map(type_: &Type) -> NullableType {
    match type_ {
        inner @ Type::NamedType(_) => map_inner(inner, false),
        Type::ListType(item_type) => {
            let item_type = map_inner(&*item_type, false);
            let list = NullableType::ListType(Box::new(item_type));
            NullableType::NullableType(Box::new(list))
        }
        Type::NonNullType(inner) => map_inner(&*inner, true),
    }
}

fn map_inner(type_: &Type, inside_non_null: bool) -> NullableType {
    match type_ {
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
        let input = Type::NonNullType(Box::new(Type::NamedType("Int".to_string())));
        let expected = "Int".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NamedType("Int".to_string());
        let expected = "Nullable(Int)".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NonNullType(
            Box::new(Type::NamedType("Int".to_string())),
        )))));
        let expected = "List(Int)".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::ListType(Box::new(Type::NonNullType(Box::new(Type::NamedType(
            "Int".to_string(),
        )))));
        let expected = "Nullable(List(Int))".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NamedType(
            "Int".to_string(),
        )))));
        let expected = "List(Nullable(Int))".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::ListType(Box::new(Type::NamedType("Int".to_string())));
        let expected = "Nullable(List(Nullable(Int)))".to_string();
        assert_eq!(map(&input).debug_print(), expected);
    }
}
