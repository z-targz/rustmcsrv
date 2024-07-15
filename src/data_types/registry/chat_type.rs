use serde::{Serialize, Deserialize};

use crate::data_types::{text_component::Json, TextComponent};


#[derive(Serialize, Deserialize, Debug)]
pub struct Decoration {
    translation_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<TextComponent<Json>>,
    parameters: Vec<String>,
}

//TODO: probably replace this with a unified text component struct
// but this isn't even used by default for the vanilla registry for 1.20.4
// but may be in the future
#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    underlined: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strikethrough: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    obfuscated: Option<i8>,

}

