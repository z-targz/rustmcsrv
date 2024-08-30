use super::{tags::tags::parse_directory, Identifier, ToProtocol, VarInt};

#[derive(Clone, Debug)]
pub struct TagRegistry {
    registry: Identifier,
    tags: Vec<Tag>,
}

impl TagRegistry {
    pub fn new(directory: &str) -> Self {
        TagRegistry {
            registry: Identifier::new(format!("minecraft:{}",directory).as_str()).unwrap(),
            tags: parse_directory(format!("generated/data/minecraft/tags/{}",directory).as_str()).into_iter().map(|(k, v)| {
                Tag::new(Identifier::new(k.as_str()).unwrap(), v)
            }).collect()
        }
    }
}

impl ToProtocol for TagRegistry {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = self.registry.to_protocol_bytes();
        out.append(&mut self.tags.to_protocol_bytes());
        out
    }
}

impl ToProtocol for Vec<TagRegistry> {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = VarInt::new(self.len() as i32).to_protocol_bytes();
        self.into_iter().for_each(|reg| {
            out.append(&mut reg.to_protocol_bytes())
        });
        out
    }
}

#[derive(Clone, Debug)]
pub struct Tag {
    tag_name: Identifier,
    entries: Vec<VarInt>,
}

impl Tag {
    pub fn new(tag_name: Identifier, entries: Vec<i32>) -> Self {
        Tag {
            tag_name: tag_name,
            entries: entries.into_iter().map(|int| VarInt::new(int)).collect(),
        }
    }
}

impl ToProtocol for Tag {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = self.tag_name.to_protocol_bytes();
        out.append(&mut self.entries.to_protocol_bytes());
        out
    }
}

impl ToProtocol for Vec<VarInt> {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = VarInt::new(self.len() as i32).to_protocol_bytes();
        self.into_iter().for_each(|int| {
            out.append(&mut int.to_protocol_bytes())
        });
        out
    }
}

impl ToProtocol for Vec<Tag> {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = VarInt::new(self.len() as i32).to_protocol_bytes();
        self.into_iter().for_each(|tag| {
            out.append(&mut tag.to_protocol_bytes())
        });
        out
    }
}