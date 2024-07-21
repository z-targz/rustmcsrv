use quote::ToTokens;
use syn::{Attribute, Data, DataStruct, Fields, FieldsNamed, Ident};

pub (in super) fn register_entity_tag(ast: syn::DeriveInput, entity_tag_name: &str) {
    let (
        name,
        attrs,
        fields,
    ) = impl_entity(&ast);

    let mut lock = super::ENTITIES.lock().unwrap();
    
    let val = 
        lock.entry(
        (
                name.to_string(),
                attrs.iter()
                    .map(|attr| {
                        attr.to_token_stream().to_string()
                    })
                    .collect(),
                fields.named.iter()
                    .map(|field| {
                        (
                            field.ident.to_token_stream().to_string(),
                            field.ty.to_token_stream().to_string(),
                            field.attrs.iter()
                                .filter(|at| match at.style {
                                    syn::AttrStyle::Outer => true,
                                    syn::AttrStyle::Inner(_) => false,
                                })
                                .filter(|at| {
                                    at.path().get_ident().to_token_stream().to_string().as_str() != "doc"
                                })
                                .map(|at| {
                                    at.to_token_stream().to_string()
                                }).collect(),
                        )
                    })
                    .collect::<Vec<_>>()
            )
        ).or_insert_with(|| vec![]);
    val.push(entity_tag_name.to_owned());
}

pub (in super) fn impl_entity(ast: &syn::DeriveInput) -> (&Ident, &Vec<Attribute>, &FieldsNamed) {
    let attrs = &ast.attrs;

    let fields = match &ast.data {
        Data::Struct(DataStruct{ fields: Fields::Named(it), struct_token : _, semi_token : _ }) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => panic!("#[Derive(CPacket)] is only implemented for `struct`s."),
    };
    (&ast.ident, attrs, fields)
}