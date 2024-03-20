use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatType {
    chat: Decoration,
    narration: Decoration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Decoration {
    translation_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
    parameters: Vec<String>,
}

//TODO: probably replace this with a unified text component struct
// but this isn't even used by default for the vanilla registry for 1.20.4
// but may be in the future
#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    //TODO
}

