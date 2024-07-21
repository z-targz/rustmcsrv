use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum LightLevel {
    fixed(i32),
    distribution(IntegerDistribution)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegerDistribution {
    r#type: String,
    #[serde(flatten)]
    value: Uniform, //Only uniform is ever used, this is a potential BUG in future versions
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum DistributionInfo {
    constant(i32),
    uniform(Uniform),

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Uniform {
    min_inclusive: i32,
    max_inclusive: i32, 
}