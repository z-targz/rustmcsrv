use std::{
    fmt::Display,
    hash::Hash
};

#[derive(Debug, Clone, PartialOrd)]
pub enum ArgType {
    Command { label: String },
    Subcommand,
    Bool,
    Float { bounds: Option<NumericArgProperties<f32>> },
    Double { bounds: Option<NumericArgProperties<f64>> },
    Int { bounds: Option<NumericArgProperties<i32>> },
    Long { bounds: Option<NumericArgProperties<i64>> },
    String { properties: Option<StringArgProperties> },
}

impl PartialEq for ArgType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Hash for ArgType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Eq for ArgType {}

impl ArgType {

}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Argument {
    name: String,
    arg_type: ArgType,
    is_mandatory: bool,
}

impl PartialOrd for Argument {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Argument {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Argument {

    pub fn new(name: String, arg_type: ArgType, is_mandatory: bool) -> Self {
        Self { name, arg_type, is_mandatory }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &ArgType {
        &self.arg_type
    }

    pub fn to_string(&self) -> String {
        format!("{self}")
    }

    pub fn is_mandatory(&self) -> bool {
        self.is_mandatory
    }

}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}



trait Properties {}

trait Numeric: Display + Copy {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NumericArgProperties<T: Numeric> {
    max: T,
    min: T
}

impl<T: Numeric> NumericArgProperties<T> {
    pub fn get_min(&self) -> T {
        self.min
    }

    pub fn get_max(&self) -> T {
        self.max
    }
}

impl<T: Numeric> Properties for NumericArgProperties<T> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct StringArgProperties {
    r#type: StringType
}

impl Properties for StringArgProperties {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum StringType {
    Word,
    Phrase,
    Greedy,
}

impl Display for StringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringType::Word => write!(f, "word"),
            StringType::Phrase => write!(f, "phrase"),
            StringType::Greedy => write!(f, "greedy"),
        }
    }
}