use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    id: String, //TODO: replace with a list of items

}