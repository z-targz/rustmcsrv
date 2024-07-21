use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use server_util::ConnectionState;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::MetaList;

pub (in super) fn impl_cpacket(ast: &syn::DeriveInput) -> TokenStream {
    let (
            name, 
            id, 
            state, 
            fields
        ) = impl_packet(ast);
    /* 
     * impl Self, impl Clientbound
     */
    let mut fields_text: String = String::new();
    let mut assign: String = String::new();
    let mut writes: String = String::new();

    fn borrow(str: &str) -> &str {
        match str {
            "String" => "&",
            "JSONString" => "&",
            "TextComponent<Json>" => "&",
            "TextComponent<Nbt>" => "&",
            "NBT" => "&",
            "PrefixedByteArray" => "&",
            "InferredByteArray" => "&",
            "PropertyArray" => "&",
            "DeathLocation" => "&",
            _ => "",
        }
    }

    for field in &fields.named {
        let field_name = field.ident.to_token_stream().to_string();

        fields_text += field.to_token_stream().to_string().as_str();
        fields_text += ", ";
        
        assign += format!("{field_name} : {field_name}, ").as_str();



        let field_type = field.ty.to_token_stream().to_string();
        if field_type.starts_with("Option") {
            let option_type = extract_T_from_option(&field_type);
            writes += format!("data.extend(create_option(self.{field_name}{}));", if borrow(option_type.as_str()) == "&" {".clone()"} else {""}).as_str();
        } else {
            //writes += format!("data.extend({func}({}self.{field_name}));", borrow(field_type.as_str())).as_str();
            writes += format!("data.extend(self.{field_name}.to_protocol_bytes().iter());", /*borrow(field_type.as_str())*/).as_str();
        }
    }

    let assign: proc_macro2::TokenStream = assign.parse().unwrap();
    let fields_text: proc_macro2::TokenStream = fields_text.parse().unwrap();
    let writes: proc_macro2::TokenStream = writes.parse().unwrap();

    let gen = quote! {
        impl Packet for #name {
            fn get_id(&self) -> i32{
                #id
            }
            fn get_associated_state(&self) -> ConnectionState {
                #state
            }
        }

        impl Clientbound for #name {
            fn to_be_bytes(&self) -> Vec<u8> {
                let mut data: Vec<u8> = Vec::new();
                #writes
                let mut out: Vec<u8> = VarInt::new(data.len() as i32 + 1).to_protocol_bytes();
                out.push(#id as u8);
                out.append(&mut data);
                out
            }
        }

        impl #name {
            pub fn new(
                #fields_text
            ) -> Self {
                #name {
                    #assign
                }
            }
        }
    };
    gen.into()
}

pub(in super) fn impl_spacket(ast: &syn::DeriveInput) -> TokenStream {
    let (
            name, 
            id, 
            state, 
            fields
        ) = impl_packet(ast);

    match state {
        ConnectionState::Handshake => {
            let mut lock = crate::HANDSHAKE_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Status => {
            let mut lock = crate::STATUS_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Login => {
            let mut lock = crate::LOGIN_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Configuration => {
            let mut lock = crate::CONFIGURATION_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Play => {
            let mut lock = crate::PLAY_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
    }

    /* 
     * impl Serverbound, Self
     */

    let mut field_names = String::new();
    let mut let_reads = String::new();
    let mut getters = String::new();

    //borrow variable-sized types
    fn borrow(str: &str) -> &str {
        match str {
            "String" => "&",
            "JSONString" => "&",
            "TextComponent<Json>" => "&",
            "TextComponent<Nbt>" => "&",
            "NBT" => "&",
            "PrefixedByteArray" => "&",
            "InferredByteArray" => "&",
            "PropertyArray" => "&",
            "DeathLocation" => "&",
            "Vec<DataPackID>" => "&",
            _ => "",
        }
    }

    for field in &fields.named {
        let field_name = field.ident.to_token_stream().to_string();
        field_names += format!("{field_name} : {field_name}, ").as_str();

        let field_type = field.ty.to_token_stream().to_string();

        if field_type.starts_with("Option") {
            let option_type = extract_T_from_option(&field_type);
            
            let b = borrow(option_type.as_str());
            getters += format!("pub fn get_{field_name}(&self) -> Option<{b}{option_type}> {{ self.{field_name}{} }}", if b == "&" {".as_ref()"} else {""}).as_str();

            let_reads += format!("let {field_name}: Option<{option_type}> = read_option(iter)?;").as_str();
        } else if field_type.starts_with("Vec") {
            getters += format!("pub fn get_{field_name}(&self) -> &{field_type} {{ &self.{field_name} }}").as_str();

            let_reads += format!("let {field_name}: {field_type} = Vec::from_protocol_iter(iter)?;").as_str();
        
        } else {
            let b = borrow(field_type.as_str());
            getters += format!("pub fn get_{field_name}(&self) -> {b}{field_type} {{ {b}self.{field_name} }}").as_str();

            let_reads += format!("let {field_name}: {field_type} = {field_type}::from_protocol_iter(iter)?;").as_str();
        }
    }

    let field_names: proc_macro2::TokenStream = field_names.parse().unwrap();
    let let_reads: proc_macro2::TokenStream = let_reads.parse().unwrap();
    let getters: proc_macro2::TokenStream = getters.parse().unwrap();

    let gen = quote! {
        impl Packet for #name {
            fn get_id(&self) -> i32{
                #id
            }
            fn get_associated_state(&self) -> ConnectionState {
                #state
            }
        }
        impl Serverbound for #name {
            fn parse(iter: &mut impl Iterator<Item = u8>) -> Result<Box<#name>, Box<dyn Error + Send + Sync>> {
                #let_reads
                Ok(Box::new(#name {
                    #field_names
                }))
            }
        }
        impl #name {
            #getters
        }
    };
    gen.into()
}

pub(in super) fn impl_packet(ast: &syn::DeriveInput) -> (&syn::Ident, i32, ConnectionState, &syn::FieldsNamed) {
    let name = &ast.ident;
    let attributes = &ast.attrs;
    if attributes.len() < 2 {
        panic!("Missing required attributes for Derive(Packet): id, ConnectionState.");
    }
    let mut id: i32 = 0;
    let mut state: ConnectionState = ConnectionState::Handshake;

    for attr in attributes {
        if attr.path().get_ident().to_token_stream().to_string().as_str() == "doc" {
            continue;
        }
        let meta_list: &MetaList = attr.meta.require_list().unwrap_or_else(|_| panic!("Missing arguments for {:?}", attr.path().get_ident().to_token_stream().to_string()));

        if attr.path().is_ident("id") {
            let msg = "Argument to id must be a valid positive i32.";
            let arg: syn::LitInt = meta_list.parse_args().expect(msg);
            let arg_as_int = arg.base10_parse::<i32>().expect(msg);
            if arg_as_int.is_negative() {
                panic!("{msg}");
            }
            id = arg_as_int;
        } else if attr.path().is_ident("state") {
            let msg = "Argument to state must be a valid ConnectionState: (Handshake, Status, Login, Play, Configuration) }";
            
            let arg: proc_macro2::TokenStream = meta_list.parse_args().expect(msg);
            match arg.to_string().as_str() {
                "Handshake" => state = ConnectionState::Handshake,
                "Status" => state = ConnectionState::Status,
                "Login" => state = ConnectionState::Login,
                "Configuration" => state = ConnectionState::Configuration,
                "Play" => state = ConnectionState::Play,
                _ => panic!("{msg}"),
            }
        }
    }

    let fields = match &ast.data {
        Data::Struct(DataStruct{ fields: Fields::Named(it), struct_token : _, semi_token : _ }) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => panic!("#[Derive(CPacket)] is only implemented for `struct`s."),
    };
    
    (name, id, state, fields)
}

#[allow(non_snake_case)]
fn extract_T_from_option(string: &String) -> String {
    let s = remove_whitespace(string);
    s[7..s.len()-1].to_string()
}

fn remove_whitespace(string: &String) -> String {
    let mut s = string.clone();
    s.retain(|c| !c.is_whitespace());
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_option() {
        let string = "Option < Meme >".to_string();
        assert_eq!(extract_T_from_option(&string), "Meme".to_string());
    }
}