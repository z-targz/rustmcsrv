use super::{FromProtocol, ToProtocol, VarInt};


#[derive(Debug, Clone)]
pub struct DataPackID {
    namespace: String,
    id: String,
    version: String,
}

#[derive(Debug, Clone)]
pub struct PackResponse {
    data: Vec<DataPackID>
}

impl FromProtocol for PackResponse {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, server_util::error::ProtocolError> 
        where Self: Sized 
    {
        Ok(Self {
            data: Vec::from_protocol_iter(iter)?
        })
    }
}

impl DataPackID {
    pub fn new(namespace: String, id: String, version: String) -> Self {
        DataPackID {
            namespace: namespace,
            id: id,
            version: version,
        }
    }
}

impl ToProtocol for DataPackID {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = self.namespace.to_protocol_bytes();
        out.append(&mut self.id.to_protocol_bytes());
        out.append(&mut self.version.to_protocol_bytes());
        out
    }
}

impl FromProtocol for DataPackID {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, server_util::error::ProtocolError> 
        where Self: Sized 
    {
        let the_namespace = String::from_protocol_iter(iter)?;
        let the_id = String::from_protocol_iter(iter)?;
        let the_version = String::from_protocol_iter(iter)?;
        Ok(Self {
            namespace: the_namespace,
            id: the_id,
            version: the_version,
        })
    }
}



impl FromProtocol for Vec<DataPackID> {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, server_util::error::ProtocolError> 
        where Self: Sized 
    {
        let mut out = Vec::new();
        let len = VarInt::from_protocol_iter(iter)?;
        for i in 0..len.get() {
            out.push(DataPackID::from_protocol_iter(iter)?);
        }
        Ok(out)
    }
}

impl ToProtocol for Vec<DataPackID> {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = VarInt::new(self.len() as i32).to_protocol_bytes();
        self.into_iter().for_each(|datapack| out.append(&mut datapack.to_protocol_bytes()));
        out
    }
}

