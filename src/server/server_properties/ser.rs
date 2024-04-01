use serde::{ser, Serialize};

use super::{Error, Result, ServerProperties};

pub struct Serializer {
    output: String,
}

pub fn to_string(value: &ServerProperties) -> Result<String> 
where
    ServerProperties: Serialize
{
    let mut serializer = Serializer {
        output: 
            "#An analog of the vanilla server.properties.\n".to_string() +
            "#Most properties here are similar to the vanilla server.properties.\n" +
            "#Most vanilla properties, unsurprisingly, are not supported.\n"
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;
    
    type SerializeSeq = Self;
    
    type SerializeTuple = Self;
    
    type SerializeTupleStruct = Self;
    
    type SerializeTupleVariant = Self;
    
    type SerializeMap = Self;
    
    type SerializeStruct = Self;
    
    type SerializeStructVariant = Self;
    
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v {"true"} else {"false"};
        Ok(())
    }
    
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_char(self, v: char) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    
    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += v;
        Ok(())
    }
    
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::UnsupportedType)
    }
    
    fn serialize_none(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }
    
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize {
        value.serialize(self)
    }
    
    fn serialize_unit(self) -> Result<()> {
        Err(Error::Empty)
    }
    
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::Empty)
    }
    
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::Empty)
    }
    
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize 
    {
        value.serialize(self)
    }
    
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize
    {
        Err(Error::Empty)
    }
    
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::UnsupportedType)
    }
    
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedType)
    }
    
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::UnsupportedType)
    }
    
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::UnsupportedType)
    }
    
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }
    
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(self)
    }
    
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize {
        self.output += key;
        self.output += "=";
        match value.serialize(&mut **self) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl <'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }
}

impl <'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }
}

impl <'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }
}

impl <'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }
}


impl <'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: Serialize 
    {
            Err(Error::UnsupportedType)
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize 
    {
            Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl <'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str,_value: &T) -> Result<()>
    where
        T: Serialize {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }
}

#[test]
fn test_serializer() {
    println!("{}", to_string(&ServerProperties::default()).unwrap());
}