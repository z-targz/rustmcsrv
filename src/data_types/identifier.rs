use std::error::Error;

use serde::ser::{Serialize, Serializer};

use serde::de::{Deserialize, Deserializer, Visitor};

#[derive(Debug)]
pub struct InvalidIdentifier;

impl Error for InvalidIdentifier {}

impl std::fmt::Display for InvalidIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier must be in the format \"minecraft:{{thing}}\", \"[custom_namespace]:{{thing}}\", or \"{{thing}}\", where the namespace is implied to be \"minecraft:\".")
    }
}

#[derive(Clone)]
pub struct Identifier {
    namespace: String,
    thing: String,
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(format!("{}:{}", self.namespace, self.thing).as_str())
    }
}

struct IdentifierVisitor;

impl <'de> Visitor<'de> for IdentifierVisitor {
    type Value = Identifier;
    
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A string in the form namespace:thing.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                match Identifier::new(v) {
                    Ok(identifier) => Ok(identifier),
                    Err(_) => Err(serde::de::Error::custom(InvalidIdentifier))
                }
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        deserializer.deserialize_str(IdentifierVisitor)
    }
}

impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.thing)
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.thing)
    }
}

impl Identifier {
    pub fn new(the_string: &str) -> Result<Identifier, InvalidIdentifier> {
        let identifier: Vec<&str> = the_string.split(":").collect();
        let namespace = 
        if identifier.len() > 1 { 
            identifier[0] 
        } else if identifier.len() == 1 { 
            "minecraft" 
        } else { 
            return Err(InvalidIdentifier) 
        };
        Ok(Identifier { 
            namespace : namespace.to_string(), 
            thing : identifier.last().unwrap().to_string() 
        })
    }

    pub fn to_string(&self) -> String {
        format!("{self}")
    }
}