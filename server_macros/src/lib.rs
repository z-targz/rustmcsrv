#![feature(proc_macro_span)]

extern crate proc_macro;
extern crate seq_macro;


use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use itertools::Itertools;
use proc_macro::TokenStream;


use quote::quote;

use quote::ToTokens;

use registry::Mapping;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::parse_macro_input;
use syn::FieldsNamed;
use syn::Ident;
use syn::LitStr;

use server_util::ConnectionState;

use base64::prelude::*;

use lazy_static::lazy_static;

mod registry;
mod entity;
mod packet;

lazy_static!{
    // static ref HANDSHAKE_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    // static ref STATUS_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    // static ref LOGIN_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    // static ref CONFIGURATION_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    // static ref PLAY_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());

    // static ref ENTITIES: Mutex<HashMap<(String, Vec<String>, Vec<(String, String, Vec<String>)>), Vec<String>>> = Mutex::new(HashMap::new());

    // static ref REGISTRY: Mutex<HashMap<String, Mapping>> = Mutex::new(registry::read_registry_json());
}

static HANDSHAKE_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});
static STATUS_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});
static LOGIN_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});
static CONFIGURATION_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});
static PLAY_PACKETS: LazyLock<Mutex<HashMap<i32, String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

static ENTITIES: LazyLock<Mutex<HashMap<
    (
        String, 
        Vec<String>, 
        Vec<(String, String, Vec<String>)>
    ), 
    Vec<String>
>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static REGISTRY: LazyLock<Mutex<HashMap<String, Mapping>>> = LazyLock::new(|| {
    Mutex::new(registry::read_registry_json())
});

/*
#[proc_macro_derive(EntityBase)]
pub fn derive_entity_base(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    entity::register_entity_tag(&ast, "EntityBase");
    quote!{}.into()
}*/



#[proc_macro_derive(EntityTrait, attributes(macroception))]
pub fn derive_entity_trait(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let (
        name, 
        fields,
    ) = impl_entity_trait(&ast);
    
    let name = name.to_string();

    

    //let mut lock = TRAIT_NAMES.lock().unwrap();
    //lock.push(name.clone());
    //drop(lock);

    // let mut getters: Vec<String> = Vec::new();
    // let mut setters: Vec<String> = Vec::new();
    
    for field in &fields.named {
        field.attrs.iter().filter_map(|attr| {
            match &attr.meta {
                syn::Meta::Path(_) => None,
                syn::Meta::List(l) => {
                    match l.path.get_ident().unwrap().to_string().as_str() {
                        "server_macros" => {
                            Some(&l.tokens)
                        }
                        _ => None
                    }
                },
                syn::Meta::NameValue(_) => None,
            }
        })
        
        .for_each(|token_stream| {
            let stream: Result<syn::Expr, syn::Error> = syn::parse(token_stream.clone().into());
            match stream {
                Ok(expr) => {
                    let expression = expr.to_token_stream().to_string();
                    if expression != "skip" {
                        if expression != "skip_getter" {
                            //todo!()
                        }
                        if expression != "skip_setter" {
                            //todo!()
                        }
                    }
                },
                Err(_) => (),
            }
        });
    }    
    
    let trait_name: proc_macro2::TokenStream = 
        String::from("Trait")
            .add(name.to_string().as_str())
            .parse()
            .unwrap();

    let name: proc_macro2::TokenStream = name.parse().unwrap();
    quote!{
        pub trait #trait_name {

        }

        impl #trait_name for #name {
            
        }
    }.into()
}

fn impl_entity_trait(ast: &syn::DeriveInput) -> (&Ident, &FieldsNamed) {
    let fields = match &ast.data {
        Data::Struct(DataStruct{ 
            fields: Fields::Named(it), 
            struct_token : _, 
            semi_token : _ }
        ) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => panic!("#[Derive(EntityTrait)] is only implemented for `struct`s."),
    };
    
    (&ast.ident, fields)
}

#[proc_macro]
pub fn pack_registry_json_files(_: TokenStream) -> TokenStream {
    const REGISTRIES: [&str;9] = [
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

    let data =  REGISTRIES
        .iter()
        .map(|registry_name| {
            let keys = get_json_map(registry_name).unwrap()
                .into_iter()
                .map(|(k, v)| {
                    quote!{#k.to_owned() => #v.to_owned(),}
                })
                .fold(proc_macro2::TokenStream::new(), |t1, t2| {
                    let mut out: proc_macro2::TokenStream = t1.clone().into();
                    out.extend(t2.into_iter());
                    out
                });
            quote!{
                #registry_name.to_owned()=>hashmap!{#keys},
            }
        })
        .fold(proc_macro2::TokenStream::new(), |t1, t2| {
            let mut out: proc_macro2::TokenStream = t1.clone().into();
            out.extend(t2.into_iter());
            out
        });

    let out = quote!{
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
    files.into_iter()
        .map(|result| result.unwrap().path())
        .filter(|f| f.is_file())
        .sorted_by(|p, p2| p.cmp(p2))
        .map(|p| Ok((p.clone(), File::open(p)?)))
        .collect::<Result<Vec<(PathBuf, File)>, std::io::Error>>()?.into_iter()
        .map(|(p, mut f)| {
            let the_name = format!("minecraft:{}", p.file_stem().unwrap().to_str().unwrap());
            let mut out = String::new();
            f.read_to_string(&mut out)?;
            Ok((the_name, out))
        }).collect()
}

macroception::create_entity_macros!{}

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
    let mut mapping_vec = mappings.into_iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<(String, i32)>>();
    mapping_vec.sort_by(|a, b| a.1.cmp(&(b.1)));

    let mut body = String::new();

    mapping_vec
        .into_iter()
        .map(|(k, v)| (k.clone(), k.split(':').last().unwrap().to_case(Case::Pascal), v)) //remove namespace and format
        .for_each(|(mc, k, v)| {
            body += (format!("
                #[serde(rename = \"{mc}\")]
                {k} = {v},
            ")).as_str()
        });
        
    let body: proc_macro2::TokenStream = body.parse().unwrap();

    let enum_name = (String::from("enum_") + registry.split(':').last().unwrap_or_else(|| {
        let registries_json_path = 
            Path::new("generated")
                .join("reports")
                .join("registries.json");
        panic!(
            "Error: registry {registry} is not a valid entry in \"{path}\".",
            path=registries_json_path.to_str().unwrap()
        )
    })).to_case(Case::Pascal);

    let enum_name: proc_macro2::TokenStream = enum_name.parse().unwrap();

    quote!{
        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        pub enum #enum_name {
            #body
        }
    }.into()
}


#[proc_macro]
pub fn create_entity_enum(_: TokenStream) -> TokenStream {
    
    let mut variants = String::new();
    let lock = ENTITIES.lock().unwrap();
    for ((name, attrs, fields), tags) in lock.iter() {

        let mut body = String::new();

        (0..tags.len()).for_each(|i| {
            body += (format!("
            #[serde(flatten)]
            __field_{i}: {tag},
            ", tag=tags.get(i).unwrap())).as_str()
        });

        for (name, r_type, field_attrs) in fields {
            body += (format!("
            {attributes}
            {name}: {r_type},
            ", attributes=field_attrs.join("\n"))).as_str()
        }

        variants += (format!("
            {attributes}
            {name} {{
                {body}
            }}
        ",attributes=attrs.join("\n"))).as_str();
    }
    
    let variants: proc_macro2::TokenStream = variants.parse().unwrap();

    quote!{
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(untagged)]
        pub enum Entity {
            EntityBase {
                __field_0: EntityBase,
            },
            #variants
        }
    }.into()
}



#[proc_macro_derive(SPacketManual, attributes(state, id))]
pub fn register_spacket_manual(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let (
        name, 
        id, 
        state, 
        _fields
    ) = packet::impl_packet(&ast);
    match state {
        ConnectionState::Handshake => {
            let mut lock = HANDSHAKE_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Status => {
            let mut lock = STATUS_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Login => {
            let mut lock = LOGIN_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Configuration => {
            let mut lock = CONFIGURATION_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Play => {
            let mut lock = PLAY_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
    }
    quote!{}.into()
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
            quote!{ Ok(#base64_string) }.into()
        }
        Err(_) => {
            quote!{ Err(std::io::Error::from(std::io::ErrorKind::NotFound))}.into()
        }
    }
}

#[proc_macro]
pub fn json_text_component(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let text = input.value();
    let the_string = format!("{{\"text\":\"{text}\"}}");
    quote!{ #the_string }.into()
}

#[proc_macro]
pub fn register_packets(_: TokenStream) -> TokenStream {

    let mut packets = String::new();

    let handshake_lock = HANDSHAKE_PACKETS.lock().unwrap();
    for handshake_packet in handshake_lock.iter() {
        packets += format!("{}(Box<handshake::{}>),", handshake_packet.1, handshake_packet.1).as_str();
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
        packets += format!("{}(Box<configuration::{}>),", config_packet.1, config_packet.1).as_str();
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
    }.into()
}

#[proc_macro]
pub fn create_handshake_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = HANDSHAKE_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(handshake::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let handshake_packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #handshake_packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_status_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = STATUS_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(status::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_login_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = LOGIN_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(login::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_config_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = CONFIGURATION_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(configuration::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_play_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = PLAY_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(play::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro_derive(ServerPropertiesDerive)]
pub fn create_property_types(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_create_property_types(&ast)
}

fn impl_create_property_types(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct{ fields: Fields::Named(it), struct_token : _, semi_token : _ }) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => panic!("#[Derive(CPacket)] is only implemented for `struct`s."),
    };

    let mut out_quote = quote!{};
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