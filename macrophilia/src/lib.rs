use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{parse::Parser, spanned::Spanned, visit::Visit, DeriveInput};

type StructName = String;
type FieldName = String;
type FieldAttrs = Vec<String>;
type FieldType = String;

static INHERITABLES: LazyLock<
    Mutex<HashMap<StructName, HashMap<FieldName, (FieldType, FieldAttrs)>>>,
> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[proc_macro_attribute]
pub fn inherit(meta: TokenStream, input: TokenStream) -> TokenStream {
    //let metadata: syn::MetaList = syn::parse(meta).unwrap_or_else(|e| panic!("oops"));
    let mut struct_name: Option<String> = None;
    let mut fields_to_inherit: HashMap<FieldName, (FieldType, FieldAttrs)> = HashMap::new();
    let lock = INHERITABLES.lock().unwrap();
    let arg_parser =
        syn::meta::parser(|meta| match meta.path.get_ident() {
            Some(path) => {
                if lock.contains_key(&path.to_string()) {
                    struct_name = Some(path.to_string().clone());
                    fields_to_inherit.extend(
                        lock.get(&path.to_string())
                            .unwrap()
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone())),
                    );
                    Ok(())
                } else if path.to_string().as_str() == "skip" {
                    if struct_name == None {
                        meta.error("Specify a struct to inherit first");
                    }
                    match meta.value()?.parse::<syn::Expr>()? {
                        syn::Expr::Array(arr) => {
                            let the_vec = arr
                                .elems
                                .into_iter()
                                .map(|elem| match elem {
                                    syn::Expr::Lit(lit) => match lit.lit {
                                        syn::Lit::Str(lit_str) => {
                                            if fields_to_inherit.contains_key(&lit_str.value()) {
                                                Ok(lit_str.value())
                                            } else {
                                                Err(meta.error(format!(
                                                    "field `{}` does not exist in struct `{}`",
                                                    lit_str.value(),
                                                    struct_name.clone().unwrap(),
                                                )))
                                            }
                                        }
                                        _ => Err(meta.error("expected a string literal")),
                                    },
                                    _ => Err(meta.error("expected a string literal")),
                                })
                                .collect::<Result<Vec<String>, syn::Error>>()?;
                            the_vec.into_iter().for_each(|field_name| {
                                fields_to_inherit.remove(&field_name);
                            });
                            Ok(())
                        }
                        syn::Expr::Lit(lit) => match lit.lit {
                            syn::Lit::Str(lit_str) => {
                                fields_to_inherit.remove(&lit_str.value());
                                Ok(())
                            }
                            _ => Err(meta.error("expected a string literal")),
                        },
                        _ => Err(meta
                            .error("must be a single string literal or slice of string literals")),
                    }
                } else {
                    Err(meta.error("unsupported arguments"))
                }
            }
            None => Err(meta.error("")),
        });
    syn::parse_macro_input!(meta with arg_parser);
    let mut ast: DeriveInput = syn::parse(input).unwrap();

    let data: Result<(), syn::Error> = match ast.data {
        syn::Data::Struct(ref mut data_struct) => match &mut data_struct.fields {
            syn::Fields::Named(ref mut fields) => fields_to_inherit
                .into_iter()
                .map(|(field_name, (field_type, field_attrs))| {
                    let mut attrs = String::new();
                    field_attrs.into_iter().for_each(|attr| {
                        attrs.push_str((attr + "\n").as_str());
                    });
                    let attrs: proc_macro2::TokenStream = attrs.parse().unwrap();
                    let field_name: proc_macro2::TokenStream = field_name.parse().unwrap();
                    let field_type: proc_macro2::TokenStream = field_type.parse().unwrap();
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        #attrs
                        #field_name: #field_type
                    })?);
                    Ok(())
                })
                .collect::<Result<_, _>>(),
            _ => Err(syn::Error::new(
                data_struct.fields.span(),
                "Fields are not named.",
            )),
        },
        _ => Err(syn::Error::new(
            ast.span(),
            "This macro can only be used with structs.",
        )),
    };

    if data.is_err() {
        data.err().unwrap().to_compile_error().into()
    } else {
        ast.to_token_stream().into()
    }
}

struct StructVisitor;

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_derive_input(&mut self, node: &'ast syn::DeriveInput) {
        let struct_name = node.ident.to_string();

        match &node.data {
            syn::Data::Struct(data_struct) => {
                let mut lock = INHERITABLES.lock().unwrap();
                lock.insert(
                    struct_name,
                    data_struct
                        .fields
                        .iter()
                        .map(|f| {
                            (
                                f.ident.to_token_stream().to_string(),
                                (
                                    f.ty.to_token_stream().to_string(),
                                    f.attrs
                                        .iter()
                                        .map(|a| a.to_token_stream().to_string())
                                        .collect::<Vec<_>>(),
                                ),
                            )
                        })
                        .collect::<HashMap<_, (_, _)>>(),
                );
            }
            syn::Data::Enum(_) => return,
            syn::Data::Union(_) => return,
        }
    }
}

#[proc_macro_attribute]
pub fn mark_parent(_meta: TokenStream, input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input.clone()).unwrap();

    StructVisitor.visit_derive_input(&ast);

    input
}
