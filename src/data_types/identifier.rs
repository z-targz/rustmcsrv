use std::error::Error;

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
    pub fn new(the_string: &str) -> Result<Self, InvalidIdentifier> {
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