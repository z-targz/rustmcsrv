use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pos {
    x: f64,
    y: f64,
    z: f64,
}