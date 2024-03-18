use std::io::Read;

use cesu8::to_java_cesu8;

pub trait NbtTag {
    fn to_be_bytes(self) -> Vec<u8>;
}
#[allow(non_camel_case_types)]
enum Tag {
    TAG_End,
    TAG_Byte(i8),
    TAG_Short(i16),
    TAG_Int(i32),
    TAG_Long(i64),
    TAG_Float(f32),
    TAG_Double(f64),
    TAG_Byte_Array(Vec<u8>),
    TAG_String(String),
    TAG_List((u8, Vec<Tag>)),
    TAG_Compound(Vec<(Option<String>, Tag)>),
    TAG_Int_Array(Vec<i32>),
    TAG_Long_Array(Vec<i64>),
}

//EVERYTHING NEEDS TO BE OPTIONALLY NAMED RIPPPPPPP Option<String>

impl NbtTag for Tag {
    fn to_be_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        match self {
            Tag::TAG_End => out.push(0u8),
            Tag::TAG_Byte(byte) => 
            {
                out = super::create_byte(byte);
            }
            Tag::TAG_Short(short) => {
                out = super::create_short(short);
            }
            Tag::TAG_Int(int) => {
                out = super::create_int(int);
            }
            Tag::TAG_Long(long) => {
                out = super::create_long(long);
            }
            Tag::TAG_Float(float) => {
                out = super::create_float(float);
            }
            Tag::TAG_Double(double) => {
                out = super::create_double(double);
            }
            Tag::TAG_Byte_Array(array) => {
                out = super::create_int(array.len() as i32).into_iter().chain(array.into_iter()).collect();
            }
            Tag::TAG_String(string) => {
                out = super::create_ushort(string.len() as u16).into_iter().chain(to_java_cesu8(string.as_str()).iter().cloned()).collect();
            },
            Tag::TAG_List(tuple) => {
                out = vec![tuple.0]
                    .into_iter()
                    .chain(super::create_int(tuple.1.len() as i32))
                    .chain(tuple.1
                        .into_iter()
                        .map(|tag| tag.to_be_bytes())
                        .flatten())        
                    .collect();
            },
            Tag::TAG_Compound(_) => todo!(),
            Tag::TAG_Int_Array(_) => todo!(),
            Tag::TAG_Long_Array(_) => todo!(),
        };
        out
    }
}