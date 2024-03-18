#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

use quote::ToTokens;
use syn::MetaList;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::parse_macro_input;
use syn::LitStr;

use server_util::ConnectionState;

use base64::prelude::*;


#[proc_macro_derive(PacketHandshake)]
pub fn packet_handshake_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_packet_handshake(&ast)
}

fn impl_packet_handshake(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Packet for #name {
            fn get_id(&self) -> i32{
                self.id
            }
            fn get_associated_state(&self) -> ConnectionState {
                ConnectionState::Handshake
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Packet, attributes(state, id))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_packet(&ast)
}

fn impl_packet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attributes = &ast.attrs;
    if attributes.len() < 2 {
        panic!("Missing required attributes for Derive(Packet): id, ConnectionState.");
    }
    let mut id: i32 = 0;
    let mut state: ConnectionState = ConnectionState::Handshake;

    for attr in attributes {
        let meta_list: &MetaList = attr.meta.require_list().unwrap_or_else(|_| panic!("Missing arguments for {:?}", attr.path().get_ident()));

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
                "Handshake" => {state = ConnectionState::Handshake},
                "Status" => {state = ConnectionState::Status},
                "Login" => {state = ConnectionState::Login},
                "Play" => {state = ConnectionState::Play},
                "Configuration" => {state = ConnectionState::Configuration},
                _ => panic!("{msg}"),
            }

        } else {
            panic!("Invalid attributes for Derive(Packet).");
        }
    }
    let gen = quote! {
        impl Packet for #name {
            fn get_id(&self) -> i32{
                #id
            }
            fn get_associated_state(&self) -> ConnectionState {
                #state
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(CPacket, attributes(state, id))]
pub fn cpacket_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_cpacket(&ast)
}

fn impl_cpacket(ast: &syn::DeriveInput) -> TokenStream {
    /* 
     * impl Packet
     */
    let name = &ast.ident;

    let attributes = &ast.attrs;
    if attributes.len() < 2 {
        panic!("Missing required attributes for Derive(Packet): `id`, `state`.");
    }
    let mut id: i32 = 0;
    let mut state: ConnectionState = ConnectionState::Handshake;

    for attr in attributes {
        let meta_list: &MetaList = attr.meta.require_list().unwrap_or_else(|_| panic!("Missing arguments for {:?}", attr.path().get_ident()));

        if attr.path().is_ident("id") {
            let msg = "Argument to id must be a valid positive i32.";
            let arg: syn::LitInt = meta_list.parse_args().expect(msg);
            let arg_as_int = arg.base10_parse::<i32>().expect(msg);
            if arg_as_int.is_negative() {
                panic!("{msg}");
            }
            id = arg_as_int;
        } else if attr.path().is_ident("state") {
            let msg = "Argument to state must be a valid ConnectionState: (`Handshake`, `Status`, `Login`, `Play`, `Configuration`) }";
            
            let arg: proc_macro2::TokenStream = meta_list.parse_args().expect(msg);
            match arg.to_string().as_str() {
                "Handshake" => {state = ConnectionState::Handshake},
                "Status" => {state = ConnectionState::Status},
                "Login" => {state = ConnectionState::Login},
                "Play" => {state = ConnectionState::Play},
                "Configuration" => {state = ConnectionState::Configuration},
                _ => panic!("{msg}"),
            }
        }
    }
    /* 
     * impl Self, impl Clientbound
     */

    let fields = match &ast.data {
        Data::Struct(DataStruct{ fields: Fields::Named(it), struct_token : _, semi_token : _ }) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => { panic!("#[Derive(CPacket)] is only implemented for `struct`s.") }
    };

    let mut fields_text: String = String::new();
    let mut assign: String = String::new();
    let mut writes: String = String::new();

    //borrow variable-sized types
    fn borrow(str: &str) -> &str {
        match str {
            "String" => "&",
            "JSON" => "&",
            "CJSONTextComponent" => "&",
            "PrefixedByteArray" => "&",
            "InferredByteArray" => "&",
            "PropertyArray" => "&",
            _ => "",
        }
    }

    for field in &fields.named {
        let field_name = field.ident.to_token_stream().to_string();
        
        let func: &str;
        let func_string: String;


        fields_text += field.to_token_stream().to_string().as_str();
        fields_text += ", ";
        
        assign += format!("{field_name} : {field_name}, ").as_str();


        let field_type = field.ty.to_token_stream().to_string();
        if field_type.starts_with("Option") {
            let option_type = extract_T_from_option(&field_type);
            func_string = format!("create_option");
            func = func_string.as_str();

            writes += format!("data.extend({func}({}self.{field_name}));", "").as_str();

        } else {
            func = match field_type.as_str() {
                "String" => "create_string",
                "JSON" => "create_string",
                "CJSONTextComponent" => "create_json_text_component",
                "VarInt" => "create_var_int",
                "VarLong" => "create_var_long",
                "bool" => "create_bool",
                "u8" => "create_u8",
                "u16" => "create_ushort",
                "Uuid" => "create_uuid",
                "i64" => "create_long",
                "f32" => "create_float",
                "f64" => "create_double",
                
                "PrefixedByteArray" => "create_prefixed_byte_array",
                "InferredByteArray" => "create_inferred_byte_array",
                "PropertyArray" => "create_property_array",
                _ => panic!("Type not supported"),
            };
        }
        

        writes += format!("data.extend({func}({}self.{field_name}));", borrow(field_type.as_str())).as_str();
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
                let mut out: Vec<u8> = create_var_int(data.len() as i32 + 1);
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


#[proc_macro_derive(SPacket, attributes(state, id))]
pub fn spacket_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_spacket(&ast)
}

fn impl_spacket(ast: &syn::DeriveInput) -> TokenStream {
    /* 
     * impl Packet
     */
    let name = &ast.ident;
    let attributes = &ast.attrs;
    if attributes.len() < 2 {
        panic!("Missing required attributes for Derive(Packet): id, ConnectionState.");
    }
    let mut id: i32 = 0;
    let mut state: ConnectionState = ConnectionState::Handshake;

    for attr in attributes {
        let meta_list: &MetaList = attr.meta.require_list().unwrap_or_else(|_| panic!("Missing arguments for {:?}", attr.path().get_ident()));

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
                "Handshake" => {state = ConnectionState::Handshake},
                "Status" => {state = ConnectionState::Status},
                "Login" => {state = ConnectionState::Login},
                "Play" => {state = ConnectionState::Play},
                "Configuration" => {state = ConnectionState::Configuration},
                _ => panic!("{msg}"),
            };
        }
    }

    /* 
     * impl Serverbound, Self
     */

    let fields = match &ast.data {
        Data::Struct(DataStruct{ fields: Fields::Named(it), struct_token : _, semi_token : _ }) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => panic!("#[Derive(CPacket)] is only implemented for `struct`s."),
    };

    let mut field_names = String::new();
    let mut let_reads = String::new();
    let mut getters = String::new();

    //borrow variable-sized types
    fn borrow(str: &str) -> &str {
        match str {
            "String" => "&",
            "JSON" => "&",
            "CJSONTextComponent" => "&",
            "PrefixedByteArray" => "&",
            "InferredByteArray" => "&",
            "PropertyArray" => "&",
            _ => "",
        }
    }

    for field in &fields.named {
        let field_name = field.ident.to_token_stream().to_string();
        field_names += format!("{field_name} : {field_name}, ").as_str();

        let field_type = field.ty.to_token_stream().to_string();

        let func: &str;
        let func_string;

        if field_type.starts_with("Option") {
            let option_type = extract_T_from_option(&field_type);

            func_string = format!("read_option");
            func = func_string.as_str();
            
            let b = borrow(option_type.as_str());
            getters += format!("pub fn get_{field_name}(&self) -> Option<{b}{option_type}> {{ self.{field_name}{} }}", if b == "&" {".as_ref()"} else {""}).as_str();

            let_reads += format!("let {field_name}: Option<{option_type}> = {func}(iter)?;").as_str();
        } else {
            func = match field_type.as_str() {
                "String" => "read_string",
                "JSON" => "read_string",
                "VarInt" => "read_var_int",
                "VarLong" => "read_var_long",
                "bool" => "read_bool",
                "u8" => "read_ubyte",
                "u16" => "read_ushort",
                "Uuid" => "read_uuid",
                "i64" => "read_long",
                "f32" => "read_float",
                "f64" => "read_double",
                
                "PrefixedByteArray" => "read_prefixed_byte_array",
                "InferredByteArray" => "read_inferred_byte_array",
                "PropertyArray" => "read_property_array",
                _ => panic!("Type not supported: {field_type}"),
            };

            let b = borrow(field_type.as_str());
            getters += format!("pub fn get_{field_name}(&self) -> {b}{field_type} {{ {b}self.{field_name} }}").as_str();

            let_reads += format!("let {field_name}: {field_type} = {func}(iter)?;").as_str();
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