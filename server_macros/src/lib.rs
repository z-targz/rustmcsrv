#![feature(proc_macro_span)]

extern crate proc_macro;
extern crate seq_macro;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;

use convert_case::Case;
use convert_case::Casing;
use itertools::Itertools;
use proc_macro::TokenStream;

use quote::quote;

use quote::ToTokens;

use registry::Mapping;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Fields;
use syn::LitStr;

use server_util::ConnectionState;

use base64::prelude::*;

mod entity;
mod packet;
mod registry;

use entity::EntityMacroError;

static HANDSHAKE_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static STATUS_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static LOGIN_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static CONFIGURATION_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static PLAY_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static REGISTRY: LazyLock<Mutex<HashMap<String, Mapping>>> =
    LazyLock::new(|| Mutex::new(registry::read_registry_json()));

/*
#[proc_macro_derive(EntityBase)]
pub fn derive_entity_base(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    entity::register_entity_tag(&ast, "EntityBase");
    quote!{}.into()
}*/

trait AttrsHelper {
    fn contains(&self, val: &str) -> bool;
    fn index_of(&self, val: &str) -> Option<usize>;

    fn span_of_first(&self, val: &str) -> Option<proc_macro2::Span>;
}

impl AttrsHelper for Vec<syn::Attribute> {
    fn contains(&self, val: &str) -> bool {
        self.iter()
            .map(|attr| attr.meta.path().get_ident().unwrap().to_string())
            .collect::<Vec<_>>()
            .contains(&val.to_owned())
    }
    fn index_of(&self, val: &str) -> Option<usize> {
        self.iter()
            .position(|attr| attr.meta.path().get_ident().unwrap().to_string() == val.to_owned())
    }

    fn span_of_first(&self, val: &str) -> Option<proc_macro2::Span> {
        match self.index_of(val) {
            Some(idx) => Some(self.get(idx).span()),
            None => None,
        }
    }
}

#[proc_macro_derive(TickableEntity, attributes(tickable, tick))]
pub fn derive_tickable_entity(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("tickable") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[tickable]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    let mut custom_tick_function: String = String::new();
    match ast.attrs.index_of("tick") {
        Some(idx) => match ast.attrs.get(idx).unwrap().meta.require_name_value() {
            Ok(name_val) => match &name_val.value {
                syn::Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Str(lit_str) => {
                        custom_tick_function = format!("{}(self);", lit_str.value())
                    }

                    _ => {
                        return syn::Error::new(name_val.value.span(), "must be a string literal")
                            .to_compile_error()
                            .into()
                    }
                },
                _ => {
                    return syn::Error::new(name_val.value.span(), "must be a string literal")
                        .to_compile_error()
                        .into()
                }
            },
            Err(_) => (),
        },
        None => (),
    };
    let custom_tick_function: proc_macro2::TokenStream = custom_tick_function.parse().unwrap();

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut tickable_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("tickable") {
                        tickable_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if tickable_fields.len() == 0 {
                    return syn::Error::new(
                            ast.ident.span(),
                            format!(
                                "no fields in `{}` are tickable. Mark fields as tickable with `#[tickable]`",
                                struct_name,
                            )
                        ).to_compile_error().into();
                }

                let body: proc_macro2::TokenStream = tickable_fields
                    .into_iter()
                    .map(|field| format!("self.{field}.tick();"))
                    .collect::<Vec<String>>()
                    .join("")
                    .parse()
                    .unwrap();

                return quote! {
                    impl TickableEntity for #struct_name {
                        fn tick(&mut self) {
                            #body
                            #custom_tick_function
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro_attribute]
pub fn entity(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Entity, attributes(entity_base))]
pub fn entity_base(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("entity_base") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[entity_base]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut entity_base_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("entity_base") {
                        entity_base_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if entity_base_fields.len() == 0 {
                    return syn::Error::new(
                        ast.ident.span(),
                        format!("no field in `{}` is marked `#[entity_base]`", struct_name,),
                    )
                    .to_compile_error()
                    .into();
                }

                if entity_base_fields.len() > 1 {
                    return syn::Error::new(
                        entity_base_fields.get(1).unwrap().span(),
                        format!(
                            "more than one field in `{}` is marked `#[entity_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                let field_name: proc_macro2::TokenStream =
                    entity_base_fields.get(0).unwrap().parse().unwrap();

                let entity_id: proc_macro2::TokenStream = match ast.ident.to_string().as_str() {
                    "EntityPlayer" => "None".parse().unwrap(),
                    id => format!("Some(EnumEntityType::{})", id.trim_start_matches("Entity"))
                        .parse()
                        .unwrap(),
                };

                return quote! {
                    use crate::nbt::tags::entity::entity_base::TraitEntityBase;
                    use crate::entity::EnumEntityType;
                    impl TraitEntityBase for #struct_name {
                        fn base_entity_tags(&self) -> &EntityBase<Self> {
                            &self.#field_name
                        }

                        fn base_entity_tags_mut(&mut self) -> &mut EntityBase<Self> {
                            &mut self.#field_name
                        }

                        fn get_identifier() -> Option<EnumEntityType> {
                            #entity_id
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro_derive(LivingEntity, attributes(living_base))]
pub fn living_base(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("living_base") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[living_base]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut living_base_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("living_base") {
                        living_base_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if living_base_fields.len() == 0 {
                    return syn::Error::new(
                        ast.ident.span(),
                        format!("no field in `{}` is marked `#[living_base]`", struct_name,),
                    )
                    .to_compile_error()
                    .into();
                }

                if living_base_fields.len() > 1 {
                    return syn::Error::new(
                        living_base_fields.get(1).unwrap().span(),
                        format!(
                            "more than one field in `{}` is marked `#[living_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                let field_name: proc_macro2::TokenStream =
                    living_base_fields.get(0).unwrap().parse().unwrap();

                return quote! {
                    use crate::nbt::tags::entity::living_base::TraitLivingBase;
                    impl TraitLivingBase for #struct_name {
                        fn living_tags(&self) -> &LivingBase<Self> {
                            &self.#field_name
                        }

                        fn living_tags_mut(&mut self) -> &mut LivingBase<Self> {
                            &mut self.#field_name
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro_derive(Mob, attributes(mob_base))]
pub fn mob_base(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("mob_base") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[mob_base]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut mob_base_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("mob_base") {
                        mob_base_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if mob_base_fields.len() == 0 {
                    return syn::Error::new(
                        ast.ident.span(),
                        format!("no field in `{}` is marked `#[mob_base]`", struct_name,),
                    )
                    .to_compile_error()
                    .into();
                }

                if mob_base_fields.len() > 1 {
                    return syn::Error::new(
                        mob_base_fields.get(1).unwrap().span(),
                        format!(
                            "more than one field in `{}` is marked `#[mob_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                let field_name: proc_macro2::TokenStream =
                    mob_base_fields.get(0).unwrap().parse().unwrap();

                return quote! {
                    use crate::nbt::tags::entity::mob_base::TraitMobBase;
                    impl TraitMobBase for #struct_name {
                        fn mob_tags(&self) -> &MobBase<Self> {
                            &self.#field_name
                        }

                        fn mob_tags_mut(&mut self) -> &mut MobBase<Self> {
                            &mut self.#field_name
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro_derive(Lootable, attributes(lootable_base))]
pub fn lootable_base(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("lootable_base") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[lootable_base]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut lootable_base_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("lootable_base") {
                        lootable_base_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if lootable_base_fields.len() == 0 {
                    return syn::Error::new(
                        ast.ident.span(),
                        format!("no field in `{}` is marked `#[lootable_base]`", struct_name,),
                    )
                    .to_compile_error()
                    .into();
                }

                if lootable_base_fields.len() > 1 {
                    return syn::Error::new(
                        lootable_base_fields.get(1).unwrap().span(),
                        format!(
                            "more than one field in `{}` is marked `#[lootable_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                let field_name: proc_macro2::TokenStream =
                    lootable_base_fields.get(0).unwrap().parse().unwrap();

                return quote! {
                    use crate::nbt::tags::entity::lootable_base::TraitLootableBase;
                    impl TraitLootableBase for #struct_name {
                        fn lootable_tags(&self) -> &LootableBase<Self> {
                            &self.#field_name
                        }

                        fn lootable_tags_mut(&mut self) -> &mut LootableBase<Self> {
                            &mut self.#field_name
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro_derive(Intelligent, attributes(brain_base))]
pub fn brain_base(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("brain_base") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[brain_base]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut brain_base_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("brain_base") {
                        brain_base_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if brain_base_fields.len() == 0 {
                    return syn::Error::new(
                        ast.ident.span(),
                        format!("no field in `{}` is marked `#[brain_base]`", struct_name,),
                    )
                    .to_compile_error()
                    .into();
                }

                if brain_base_fields.len() > 1 {
                    return syn::Error::new(
                        brain_base_fields.get(1).unwrap().span(),
                        format!(
                            "more than one field in `{}` is marked `#[brain_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                let field_name: proc_macro2::TokenStream =
                    brain_base_fields.get(0).unwrap().parse().unwrap();

                return quote! {
                    use crate::nbt::tags::entity::brain_base::TraitHasBrain;
                    impl TraitHasBrain for #struct_name {
                        fn get_brain(&self) -> &Brain<Self> {
                            &self.#field_name
                        }

                        fn get_brain_mut(&mut self) -> &mut Brain<Self> {
                            &mut self.#field_name
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro_derive(Tameable, attributes(tameable_base))]
pub fn tameable_base(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("tameable_base") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[tameable_base]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut tameable_base_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("tameable_base") {
                        tameable_base_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if tameable_base_fields.len() == 0 {
                    return syn::Error::new(
                        ast.ident.span(),
                        format!("no field in `{}` is marked `#[tameable_base]`", struct_name,),
                    )
                    .to_compile_error()
                    .into();
                }

                if tameable_base_fields.len() > 1 {
                    return syn::Error::new(
                        tameable_base_fields.get(1).unwrap().span(),
                        format!(
                            "more than one field in `{}` is marked `#[tameable_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                let field_name: proc_macro2::TokenStream =
                    tameable_base_fields.get(0).unwrap().parse().unwrap();

                return quote! {
                    use crate::nbt::tags::entity::tameable_base::TraitTameableBase;
                    impl TraitTameableBase for #struct_name {
                        fn tameable_tags(&self) -> &TameableBase<Self> {
                            &self.#field_name
                        }

                        fn tameable_tags_mut(&mut self) -> &mut TameableBase<Self> {
                            &mut self.#field_name
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro_derive(Breedable, attributes(breedable_base))]
pub fn breedable_base(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident.to_token_stream();

    match &ast.attrs.span_of_first("breedable_base") {
        Some(span) => {
            return syn::Error::new(
                span.clone(),
                "cannot apply field attribute `#[breedable_base]` to a struct",
            )
            .to_compile_error()
            .into()
        }
        None => (),
    }

    match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut tameable_base_fields: Vec<String> = Vec::new();

                for field in &fields_named.named {
                    if field.attrs.contains("breedable_base") {
                        tameable_base_fields.push(field.ident.clone().unwrap().to_string());
                    }
                }

                if tameable_base_fields.len() == 0 {
                    return syn::Error::new(
                        ast.ident.span(),
                        format!(
                            "no field in `{}` is marked `#[breedable_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                if tameable_base_fields.len() > 1 {
                    return syn::Error::new(
                        tameable_base_fields.get(1).unwrap().span(),
                        format!(
                            "more than one field in `{}` is marked `#[breedable_base]`",
                            struct_name,
                        ),
                    )
                    .to_compile_error()
                    .into();
                }

                let field_name: proc_macro2::TokenStream =
                    tameable_base_fields.get(0).unwrap().parse().unwrap();

                return quote! {
                    use crate::nbt::tags::entity::breedable_base::TraitBreedableBase;
                    impl TraitBreedableBase for #struct_name {
                        fn breedable_tags(&self) -> &BreedableBase<Self> {
                            &self.#field_name
                        }

                        fn breedable_tags_mut(&mut self) -> &mut BreedableBase<Self> {
                            &mut self.#field_name
                        }
                    }
                }
                .into();
            }
            _ => {
                return syn::Error::new(
                    ast.ident.span(),
                    format!("`{struct_name}` has no named fields"),
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                format!("`{struct_name}` is not a data struct"),
            )
            .to_compile_error()
            .into()
        }
    }
}

#[proc_macro]
pub fn pack_registry_json_files(_: TokenStream) -> TokenStream {
    const REGISTRIES: [&str; 9] = [
        "worldgen/biome",
        "chat_type",
        "trim_pattern",
        "trim_material",
        "wolf_variant",
        "dimension_type",
        "damage_type",
        "banner_pattern",
        "painting_variant",
    ];

    let data = REGISTRIES
        .iter()
        .map(|registry_name| {
            let keys = get_json_map(registry_name)
                .unwrap()
                .into_iter()
                .map(|(k, v)| {
                    quote! {#k.to_owned() => #v.to_owned(),}
                })
                .fold(proc_macro2::TokenStream::new(), |t1, t2| {
                    let mut out: proc_macro2::TokenStream = t1.clone().into();
                    out.extend(t2.into_iter());
                    out
                });
            quote! {
                #registry_name.to_owned()=>hashmap!{#keys},
            }
        })
        .fold(proc_macro2::TokenStream::new(), |t1, t2| {
            let mut out: proc_macro2::TokenStream = t1.clone().into();
            out.extend(t2.into_iter());
            out
        });

    let out = quote! {
        hashmap!{#data}
    };

    //panic!("{}",out.to_string());

    out.into()
}

fn get_json_map(registry_name: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(cargo_manifest_dir.as_str())
        .join("generated")
        .join("data")
        .join("minecraft")
        .join(registry_name);
    let files = std::fs::read_dir(path).unwrap();
    files
        .into_iter()
        .map(|result| result.unwrap().path())
        .filter(|f| f.is_file())
        .sorted_by(|p, p2| p.cmp(p2))
        .map(|p| Ok((p.clone(), File::open(p)?)))
        .collect::<Result<Vec<(PathBuf, File)>, std::io::Error>>()?
        .into_iter()
        .map(|(p, mut f)| {
            let the_name = format!("minecraft:{}", p.file_stem().unwrap().to_str().unwrap());
            let mut out = String::new();
            f.read_to_string(&mut out)?;
            Ok((the_name, out))
        })
        .collect()
}

//macroception::create_entity_macros!{}

#[proc_macro]
pub fn generate_entity_id_enum(_: TokenStream) -> TokenStream {
    generate_registry_enum("minecraft:entity_type").into()
}

#[proc_macro]
pub fn generate_potion_effect_id_enum(_: TokenStream) -> TokenStream {
    generate_registry_enum("minecraft:mob_effect").into()
}

fn generate_registry_enum(registry: &str) -> TokenStream {
    let binding = REGISTRY.lock().unwrap();
    let mappings = binding.get(registry).unwrap().get_mappings();
    let mut mapping_vec = mappings
        .into_iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect::<Vec<(String, i32)>>();
    mapping_vec.sort_by(|a, b| a.1.cmp(&(b.1)));

    let mut body = String::new();

    mapping_vec
        .into_iter()
        .map(|(k, v)| {
            (
                k.clone(),
                k.split(':').last().unwrap().to_case(Case::Pascal),
                v,
            )
        }) //remove namespace and format
        .for_each(|(mc, k, v)| {
            body += (format!(
                "
                #[serde(rename = \"{mc}\")]
                {k} = {v},
            "
            ))
            .as_str()
        });

    let body: proc_macro2::TokenStream = body.parse().unwrap();

    let enum_name = (String::from("enum_")
        + registry.split(':').last().unwrap_or_else(|| {
            let registries_json_path = Path::new("generated")
                .join("reports")
                .join("registries.json");
            panic!(
                "Error: registry {registry} is not a valid entry in \"{path}\".",
                path = registries_json_path.to_str().unwrap()
            )
        }))
    .to_case(Case::Pascal);

    let enum_name: proc_macro2::TokenStream = enum_name.parse().unwrap();

    quote! {
        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        pub enum #enum_name {
            #body
        }
    }
    .into()
}

#[proc_macro]
pub fn create_entity_enum(_: TokenStream) -> TokenStream {
    let src_dir = Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("src");
    let files = std::fs::read_dir(src_dir.join("entity").join("entities")).unwrap();

    let mut variants = String::new();
    let mut imports = String::new();
    let mut getters = String::new();

    let _ = files
        .into_iter()
        .map(|result| {
            match result {
                Ok(dir_entry) => {
                    let module_path = dir_entry
                        .path()
                        .strip_prefix(src_dir.to_str().unwrap())
                        .unwrap()
                        .iter()
                        .map(|segment| segment.to_str().unwrap().to_owned())
                        .collect::<Vec<_>>()
                        .join("::")
                        .trim_end_matches(".rs")
                        .to_owned();

                    let mut file_contents = String::new();
                    File::open(dir_entry.path())?.read_to_string(&mut file_contents)?;
                    let file_src = syn::parse_file(&file_contents.as_str())?;

                    struct FileVisitor {
                        pub tag_names: Vec<String>,
                    }
                    impl FileVisitor {
                        pub fn new(_: PathBuf) -> Self {
                            Self {
                                tag_names: Vec::new(),
                            }
                        }
                    }
                    impl<'ast> Visit<'ast> for FileVisitor {
                        fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
                            let mut to_add: bool = false;
                            for attr in &item.attrs {
                                match attr.path().get_ident() {
                                    Some(ident) => {
                                        if ident.to_string().as_str() == "entity" {
                                            to_add = true;
                                        }
                                    }
                                    None => (),
                                }
                            }
                            if to_add {
                                self.tag_names.push(item.ident.to_string());
                            }
                        }
                    }
                    let mut file_visitor = FileVisitor::new(dir_entry.path());
                    file_visitor.visit_file(&file_src);

                    //let tag_name = module_name.to_case(Case::Pascal);
                    Ok((file_visitor.tag_names, module_path))
                }
                Err(e) => Err(EntityMacroError::IOError(e)),
            }
        })
        .map(|result| match result {
            Ok((tag_names, module_path)) => {
                tag_names.into_iter().for_each(|tag_name| {
                    let body = format!(
                        "
                            #[serde(flatten)]
                            __field: {tag_name},
                            "
                    );

                    variants += (format!(
                        "
                            {tag_name} {{
                                {body}
                            }},
                        "
                    ))
                    .as_str();

                    imports += format!("use crate::{module_path}::{tag_name};\n").as_str();

                    getters += format!(
                        "Entity::{tag_name} {{__field}} => {{
                            if {tag_name}::IS_TICKABLE {{
                                __field.tick()
                            }}
                        }},"
                    )
                    .as_str();
                });
                Ok(())
            }
            Err(e) => Err(e),
        })
        .collect::<Result<(), EntityMacroError>>();

    let variants: proc_macro2::TokenStream = variants.parse().unwrap();
    let imports: proc_macro2::TokenStream = imports.parse().unwrap();
    let getters: proc_macro2::TokenStream = getters.parse().unwrap();

    quote! {
        #imports
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(untagged)]
        pub enum Entity {
            #variants
        }

        impl Entity {
            pub fn try_tick(&mut self) {
                match self {
                    #getters
                }
            }
        }

    }
    .into()
}

#[proc_macro_derive(SPacketManual, attributes(state, id))]
pub fn register_spacket_manual(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let (name, id, state, _fields) = packet::impl_packet(&ast);
    match state {
        ConnectionState::Handshake => {
            let mut lock = HANDSHAKE_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
        ConnectionState::Status => {
            let mut lock = STATUS_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
        ConnectionState::Login => {
            let mut lock = LOGIN_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
        ConnectionState::Configuration => {
            let mut lock = CONFIGURATION_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
        ConnectionState::Play => {
            let mut lock = PLAY_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
    }
    quote! {}.into()
}

#[proc_macro_derive(CPacket, attributes(state, id))]
pub fn cpacket_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    packet::impl_cpacket(&ast).into()
}

#[proc_macro_derive(SPacket, attributes(state, id))]
pub fn spacket_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    packet::impl_spacket(&ast).into()
}

#[proc_macro]
pub fn base64_image(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let path = input.value();

    let data = std::fs::read(&path);
    match data {
        Ok(vec) => {
            let mut base64_string = String::new();
            BASE64_STANDARD.encode_string(vec, &mut base64_string);
            quote! { Ok(#base64_string) }.into()
        }
        Err(_) => quote! { Err(std::io::Error::from(std::io::ErrorKind::NotFound))}.into(),
    }
}

#[proc_macro]
pub fn json_text_component(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let text = input.value();
    let the_string = format!("{{\"text\":\"{text}\"}}");
    quote! { #the_string }.into()
}

#[proc_macro]
pub fn register_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();

    let handshake_lock = HANDSHAKE_PACKETS.lock().unwrap();
    for handshake_packet in handshake_lock.iter() {
        packets += format!(
            "{}(Box<handshake::{}>),",
            handshake_packet.1, handshake_packet.1
        )
        .as_str();
    }
    drop(handshake_lock);

    let status_lock = STATUS_PACKETS.lock().unwrap();
    for status_packet in status_lock.iter() {
        packets += format!("{}(Box<status::{}>),", status_packet.1, status_packet.1).as_str();
    }
    drop(status_lock);

    let login_lock = LOGIN_PACKETS.lock().unwrap();
    for login_packet in login_lock.iter() {
        packets += format!("{}(Box<login::{}>),", login_packet.1, login_packet.1).as_str();
    }
    drop(login_lock);

    let config_lock = CONFIGURATION_PACKETS.lock().unwrap();
    for config_packet in config_lock.iter() {
        packets += format!(
            "{}(Box<configuration::{}>),",
            config_packet.1, config_packet.1
        )
        .as_str();
    }
    drop(config_lock);

    let play_lock = PLAY_PACKETS.lock().unwrap();
    for play_packet in play_lock.iter() {
        packets += format!("{}(Box<play::{}>),", play_packet.1, play_packet.1).as_str();
    }
    drop(play_lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub enum SPacket {
            #packets
        }
    }
    .into()
}

#[proc_macro]
pub fn create_handshake_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = HANDSHAKE_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!(
            "{} => Ok(SPacket::{}(handshake::{}::parse(iter)?)),",
            packet.0, packet.1, packet.1
        )
        .as_str();
    }
    drop(lock);

    let handshake_packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #handshake_packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }
    .into()
}

#[proc_macro]
pub fn create_status_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = STATUS_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!(
            "{} => Ok(SPacket::{}(status::{}::parse(iter)?)),",
            packet.0, packet.1, packet.1
        )
        .as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }
    .into()
}

#[proc_macro]
pub fn create_login_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = LOGIN_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!(
            "{} => Ok(SPacket::{}(login::{}::parse(iter)?)),",
            packet.0, packet.1, packet.1
        )
        .as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }
    .into()
}

#[proc_macro]
pub fn create_config_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = CONFIGURATION_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!(
            "{} => Ok(SPacket::{}(configuration::{}::parse(iter)?)),",
            packet.0, packet.1, packet.1
        )
        .as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }
    .into()
}

#[proc_macro]
pub fn create_play_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = PLAY_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!(
            "{} => Ok(SPacket::{}(play::{}::parse(iter)?)),",
            packet.0, packet.1, packet.1
        )
        .as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }
    .into()
}

#[proc_macro_derive(ServerPropertiesDerive)]
pub fn create_property_types(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_create_property_types(&ast)
}

fn impl_create_property_types(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(it),
            struct_token: _,
            semi_token: _,
        }) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => {
            panic!("#[Derive(CPacket)] is only implemented for `struct`s.")
        }
    };

    let mut out_quote = quote! {};
    for field in &fields.named {
        let field_name1 = field.ident.to_token_stream().to_string();
        let field_name2 = field_name1.replace("_", "-");
        let field_name = field_name2.as_str();

        let field_name_token_stream: proc_macro2::TokenStream = field_name1.parse().unwrap();
        let field_type1 = field.ty.to_token_stream().to_string();
        let field_type: proc_macro2::TokenStream = field_type1.parse().unwrap();

        let optional = field_type1.starts_with("Option");

        let empty_str_result: proc_macro2::TokenStream = if optional {
            quote!{server_properties.#field_name_token_stream = None}
        } else {
            quote!{return Err(LoadPropertiesError::PropertyCannotBeNone(#field_name2.to_string(), i))}
        }.into();

        out_quote.extend(quote!{
            #field_name => {
                match tuple.1 {
                    "" => #empty_str_result,
                    some => {
                        match some.parse::<#field_type>() {
                            Ok(x) => {
                                server_properties.#field_name_token_stream = x;
                            },
                            Err(_) => {
                                return Err(LoadPropertiesError::InvalidValueForProperty(tuple.0.to_string(), i));
                            }
                        }
                    },
                }
            },
        }.into_iter());
    }
    quote!{

        impl #name {

            /// Loads the server properties from the file server.properties in the
            /// main directory.
            /// 
            /// TODO: replace this with a custom serde implementation
            /// 
            /// Desired functionality for the serde implementation:
            ///
            /// 1. creates the default ServerProperties
            /// 2. deserializes server.properties from field-name=value to field_name = value 
            ///    into the created ServerProperties
            /// 3. serializes the createdServerProperties to server.properties to populate 
            ///    missing properties
            pub fn load_server_properties() -> Result<ServerProperties, LoadPropertiesError> {
                if !Path::new("server.properties").exists() {
                    let mut file = File::create("server.properties")?;
                    match ServerProperties::default().write_to_file(&mut file) {
                        Ok(_) => (),
                        Err(e) => return Err(LoadPropertiesError::IOError(e.to_string())),
                    }
                };

                let reader = BufReader::new(OpenOptions::new().read(true).open("server.properties")?);
                let mut server_properties = ServerProperties::default();
                let mut i = 0;
                for result in reader.lines() {
                    i += 1;
                    match result {
                        Ok(line) => {
                            if line.starts_with("#") { continue; }

                            let pair = line.split("=").collect::<Vec<_>>();

                            if pair.len() != 2 {
                                return Err(LoadPropertiesError::MalformedLine(i));
                            }
                            let tuple: (&str, &str) = (pair.get(0).unwrap(), pair.get(1).unwrap());
                            match tuple.0 {
                                #out_quote
                                _ => return Err(LoadPropertiesError::InvalidProperty(pair.get(0).unwrap().to_string(), i)),
                            }
                        },
                        Err(e) => return Err(e)?,
                    }
                }
                let mut writer = BufWriter::new(OpenOptions::new().read(true).write(true).truncate(true).open("server.properties")?);
                match server_properties.write_to_file(&mut writer) {
                    Ok(_) => (),
                    Err(e) => return Err(LoadPropertiesError::IOError(e.to_string())),
                }
                Ok(server_properties)
            }
        }
    }.into()
}
