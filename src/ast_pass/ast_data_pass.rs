mod find_enum_variants;
mod find_interface_implementors;
mod find_special_scalar_types;

use self::find_enum_variants::{find_enum_variants, EnumVariants};
use self::find_interface_implementors::{find_interface_implementors, InterfaceImplementors};
use self::find_special_scalar_types::{find_special_scalar_types, SpecialScalarTypesList};
use graphql_parser::schema::Document;

pub struct AstData<'doc> {
    pub(super) interface_implementors: InterfaceImplementors<'doc>,
    pub(super) special_scalars: SpecialScalarTypesList<'doc>,
    pub(super) enum_variants: EnumVariants<'doc>,
}

impl<'doc> AstData<'doc> {
    pub fn new(doc: &'doc Document) -> Self {
        let interface_implementors = find_interface_implementors(&doc);
        let special_scalars = find_special_scalar_types(&doc);
        let enum_variants = find_enum_variants(&doc);

        Self {
            interface_implementors,
            special_scalars,
            enum_variants,
        }
    }
}
