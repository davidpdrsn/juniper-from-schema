extern crate proc_macro;
extern crate proc_macro2;

use graphql_parser::{parse_schema, query::Name, schema::*};
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Colon2, AngleBracketedGenericArguments, Ident, Path,
    PathArguments, PathSegment, Token,
};

#[macro_use]
mod macros;

mod nullable_type;

use self::nullable_type::NullableType;

#[proc_macro]
pub fn graphql_schema_from_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let file = input.to_string().replace("\"", "");
    let pwd = std::env::current_dir().unwrap();
    let path = pwd.join(file);

    match read_file(&path) {
        Ok(schema) => parse_and_gen_schema(schema),
        Err(err) => panic!("{}", err),
    }
}

#[proc_macro]
pub fn graphql_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let schema = input.to_string();
    parse_and_gen_schema(schema)
}

fn parse_and_gen_schema(schema: String) -> proc_macro::TokenStream {
    let mut output = Output::new();

    match parse_schema(&schema) {
        Ok(doc) => gen_doc(doc, &mut output),
        Err(parse_error) => panic!("{}", parse_error),
    };

    output.tokens().into_iter().collect::<TokenStream>().into()
}

struct Output {
    tokens: Vec<TokenStream>,
    date_time_scalar_defined: bool,
    date_scalar_defined: bool,
}

impl Output {
    fn new() -> Self {
        Output {
            tokens: vec![],
            date_scalar_defined: false,
            date_time_scalar_defined: false,
        }
    }

    fn tokens(self) -> Vec<TokenStream> {
        self.tokens
    }

    fn push(&mut self, toks: TokenStream) {
        self.tokens.push(toks);
    }

    fn is_date_time_scalar_defined(&self) -> bool {
        self.date_time_scalar_defined
    }

    fn is_date_scalar_defined(&self) -> bool {
        self.date_scalar_defined
    }

    fn date_time_scalar_defined(&mut self) {
        self.date_time_scalar_defined = true
    }

    fn date_scalar_defined(&mut self) {
        self.date_scalar_defined = true
    }

    fn clone_without_tokens(&self) -> Self {
        Output {
            tokens: vec![],
            date_scalar_defined: self.date_scalar_defined,
            date_time_scalar_defined: self.date_time_scalar_defined,
        }
    }
}

fn gen_doc(doc: Document, out: &mut Output) {
    for def in doc.definitions {
        gen_def(def, out);
    }
}

fn gen_def(def: Definition, out: &mut Output) {
    use graphql_parser::schema::Definition::*;

    match def {
        DirectiveDefinition(_) => todo!("directive definition"),
        SchemaDefinition(schema_def) => gen_schema_def(schema_def, out),
        TypeDefinition(type_def) => gen_type_def(type_def, out),
        TypeExtension(_) => todo!("type extension"),
    }
}

fn gen_schema_def(schema_def: SchemaDefinition, out: &mut Output) {
    // TODO: use
    //   position
    //   directives
    //   subscription

    let query = match schema_def.query {
        Some(query) => ident(query),
        None => panic!("Juniper requires that the schema type has a query"),
    };

    let mutation = match schema_def.mutation {
        Some(mutation) => quote_ident(mutation),
        None => quote! { juniper::EmptyMutation<()> },
    };

    (quote! {
        pub type Schema = juniper::RootNode<'static, #query, #mutation>;
    })
    .add_to(out)
}

fn gen_type_def(type_def: TypeDefinition, out: &mut Output) {
    use graphql_parser::schema::TypeDefinition::*;

    match type_def {
        Enum(enum_type) => gen_enum_type(enum_type, out),
        Object(obj_type) => gen_obj_type(obj_type, out),
        Scalar(scalar_type) => gen_scalar_type(scalar_type, out),
        InputObject(_) => todo!("input object"),
        Interface(_) => todo!("interface"),
        Union(_) => todo!("union"),
    }
}

fn gen_enum_type(enum_type: EnumType, out: &mut Output) {
    // TODO: use
    //   position
    //   description
    //   directives

    let name = ident(enum_type.name.to_camel_case());
    pp!(name);

    let values = gen_with(gen_enum_value, enum_type.values, &out);

    (quote! {
        #[derive(juniper::GraphQLEnum, Debug, Eq, PartialEq, Copy, Clone, Hash)]
        pub enum #name {
            #values
        }
    })
    .add_to(out)
}

fn gen_enum_value(enum_type: EnumValue, out: &mut Output) {
    // TODO: use
    //   position
    //   description
    //   directives

    let graphql_name = enum_type.name;
    let name = ident(graphql_name.to_camel_case());
    (quote! {
        #[graphql(name=#graphql_name)]
        #name,
    })
    .add_to(out)
}

fn gen_scalar_type(scalar_type: ScalarType, out: &mut Output) {
    // TODO: use
    //   position
    //   description
    //   directives

    match &*scalar_type.name {
        "Date" => out.date_scalar_defined(),
        "DateTime" => out.date_time_scalar_defined(),
        _ => panic!("Only Date and DateTime scalars are supported at the moment"),
    };
}

fn gen_obj_type(obj_type: ObjectType, out: &mut Output) {
    // TODO: Use
    //   description
    //   implements_interface
    //   directives

    let struct_name = ident(obj_type.name);

    let trait_name = ident(format!("{}Fields", struct_name));

    let field_tokens = obj_type
        .fields
        .into_iter()
        .map(|field| gen_field(field, &out))
        .collect::<Vec<_>>();

    let trait_methods = field_tokens
        .iter()
        .map(|field| {
            let field_name = field.field_method.clone();
            let field_type = field.field_type.clone();
            let args = field.args.clone();
            quote! {
                fn #field_name<'a>(&self, executor: &Executor<'a, Context>, #args) -> FieldResult<#field_type>;
            }
        })
        .collect::<TokenStream>();

    println!("hi");
    (quote! {
        pub trait #trait_name {
            #trait_methods
        }
    })
    .add_to(out);

    let fields = field_tokens
        .iter()
        .map(|field| {
            let field_name = field.name.clone();
            let field_type = field.field_type.clone();
            let args = field.args.clone();
            let field_method = field.field_method.clone();
            let params = field.params.clone();
            quote! {
                field #field_name(&executor, #args) -> juniper::FieldResult<#field_type> {
                    <#struct_name as self::#trait_name>::#field_method(&self, &executor, #params)
                }
            }
        })
        .collect::<TokenStream>();

    (quote! {
        juniper::graphql_object!(#struct_name: Context |&self| {
            #fields
        });
    })
    .add_to(out);
}

struct FieldTokens {
    name: Ident,
    args: TokenStream,
    field_type: TokenStream,
    field_method: Ident,
    params: TokenStream,
}

fn gen_field(field: Field, out: &Output) -> FieldTokens {
    // TODO: Use
    //   description
    //   directives

    let name = ident(field.name);

    let field_type = gen_field_type(field.field_type, out);

    let field_method = ident(format!("field_{}", name.to_string().to_snake_case()));

    let args_names_and_types = field
        .arguments
        .into_iter()
        .map(|x| argument_to_name_and_rust_type(x, out))
        .collect::<Vec<_>>();

    let args = args_names_and_types
        .iter()
        .map(|(arg, arg_type)| {
            let arg = ident(arg);
            quote! { #arg: #arg_type, }
        })
        .collect::<TokenStream>();

    let params = args_names_and_types
        .iter()
        .map(|(arg, _)| {
            let arg = ident(arg);
            quote! { #arg, }
        })
        .collect::<TokenStream>();

    FieldTokens {
        name,
        args,
        field_type,
        field_method,
        params,
    }
}

fn argument_to_name_and_rust_type(arg: InputValue, out: &Output) -> (Name, TokenStream) {
    // TODO: use
    //   position
    //   description
    //   default_value
    //   directives

    if let Some(_) = arg.default_value {
        todo!("default value");
    }

    let arg_name = arg.name.to_snake_case();
    let arg_type = gen_field_type(arg.value_type, out);
    (arg_name, arg_type)
}

fn gen_field_type(field_type: Type, out: &Output) -> TokenStream {
    let field_type = NullableType::from_type(field_type);
    gen_nullable_field_type(field_type, out)
}

fn gen_nullable_field_type(field_type: NullableType, out: &Output) -> TokenStream {
    use self::nullable_type::NullableType::*;

    match field_type {
        NamedType(name) => graphql_scalar_type_to_rust_type(name, &out),
        ListType(item_type) => {
            let item_type = gen_nullable_field_type(*item_type, &out);
            quote! { Vec<#item_type> }
        }
        NullableType(item_type) => {
            let item_type = gen_nullable_field_type(*item_type, &out);
            quote! { Option<#item_type> }
        }
    }
}

// Type according to https://graphql.org/learn/schema/#scalar-types
fn graphql_scalar_type_to_rust_type(name: Name, out: &Output) -> TokenStream {
    match &*name {
        "Int" => quote! { i32 },
        "Float" => quote! { f64 },
        "String" => quote! { String },
        "Boolean" => quote! { bool },
        "ID" => todo!("ID scalar"),
        "Date" => {
            if out.is_date_scalar_defined() {
                quote! { chrono::naive::NaiveDate }
            } else {
                panic!(
                    "Fields with type `Date` is only allowed if you have define a scalar named `Date`"
                )
            }
        }
        "DateTime" => {
            if out.is_date_scalar_defined() {
                quote! { chrono::DateTime<chrono::offset::Utc> }
            } else {
                panic!(
                    "Fields with type `DateTime` is only allowed if you have define a scalar named `DateTime`"
                )
            }
        }
        name => quote_ident(name.to_camel_case()),
    }
}

fn push_simple_path(s: &str, segments: &mut Punctuated<PathSegment, Colon2>) {
    segments.push(PathSegment {
        ident: ident(s),
        arguments: PathArguments::None,
    });
}

trait AddToOutput {
    fn add_to(self, out: &mut Output);
}

impl AddToOutput for TokenStream {
    fn add_to(self, out: &mut Output) {
        out.push(self);
    }
}

fn ident<T: AsRef<str>>(name: T) -> Ident {
    Ident::new(name.as_ref(), Span::call_site())
}

fn quote_ident<T: AsRef<str>>(name: T) -> TokenStream {
    let ident = ident(name);
    quote! { #ident }
}

fn gen_with<F, T>(f: F, ts: Vec<T>, other: &Output) -> TokenStream
where
    F: Fn(T, &mut Output),
{
    let mut acc = other.clone_without_tokens();
    for t in ts {
        f(t, &mut acc);
    }
    acc.tokens().into_iter().collect::<TokenStream>()
}

fn read_file(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
    use std::{fs::File, io::prelude::*};
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
