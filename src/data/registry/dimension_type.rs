use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LightLevel {
    fixed(i32),
    distribution(IntegerDistribution)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegerDistribution {
    r#type: String,
    value: Uniform, //Only uniform is ever used, this is a potential BUG in future versions
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DistributionInfo {
    constant(i32),
    uniform(Uniform),

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uniform {
    min_inclusive: i32,
    max_inclusive: i32,
}