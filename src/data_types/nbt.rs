use std::io::Read;
use std::collections::HashMap;


use cesu8::to_java_cesu8;

pub trait NbtTag {
    fn to_be_bytes(self) -> Vec<u8>;
}
#[allow(non_camel_case_types)]
enum Tag {
    TAG_End,
    TAG_Byte(Option<String>, i8),
    TAG_Short(Option<String>, i16),
    TAG_Int(Option<String>, i32),
    TAG_Long(Option<String>, i64),
    TAG_Float(Option<String>, f32),
    TAG_Double(Option<String>, f64),
    TAG_Byte_Array(Option<String>, Vec<u8>),
    TAG_String(Option<String>, String),
    TAG_List(Option<String>, (u8, Vec<Tag>)),
    TAG_Compound(Option<String>, HashMap<String, Tag>), //A list of named tags
    TAG_Int_Array(Option<String>, Vec<i32>),
    TAG_Long_Array(Option<String>, Vec<i64>),
}

//EVERYTHING NEEDS TO BE OPTIONALLY NAMED RIPPPPPPP Option<String>

impl NbtTag for Tag {
    fn to_be_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        match self {
            Tag::TAG_End => out.push(0u8),
            Tag::TAG_Byte(name, byte) => 
            {
                out = super::create_byte(byte);
            }
            Tag::TAG_Short(name, short) => {
                out = super::create_short(short);
            }
            Tag::TAG_Int(name, int) => {
                out = super::create_int(int);
            }
            Tag::TAG_Long(name, long) => {
                out = super::create_long(long);
            }
            Tag::TAG_Float(name, float) => {
                out = super::create_float(float);
            }
            Tag::TAG_Double(name, double) => {
                out = super::create_double(double);
            }
            Tag::TAG_Byte_Array(name, array) => {
                out = super::create_int(array.len() as i32).into_iter().chain(array.into_iter()).collect();
            }
            Tag::TAG_String(name, string) => {
                out = super::create_ushort(string.len() as u16).into_iter().chain(to_java_cesu8(string.as_str()).iter().cloned()).collect();
            },
            Tag::TAG_List(name, tuple) => {
                out = vec![tuple.0]
                    .into_iter()
                    .chain(super::create_int(tuple.1.len() as i32))
                    .chain(tuple.1
                        .into_iter()
                        .map(|tag| tag.to_be_bytes())
                        .flatten())        
                    .collect();
            },
            Tag::TAG_Compound(name, children) => todo!(),
            Tag::TAG_Int_Array(name, data) => todo!(),
            Tag::TAG_Long_Array(name, data) => todo!(),
        };
        out
    }
}