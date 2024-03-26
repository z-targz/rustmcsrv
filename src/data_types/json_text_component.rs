use serde::{Deserialize, Serialize};

use super::ToProtocol;

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONTextComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    translate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    keybind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    underlined: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    strikethrough: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    obfuscated: Option<bool>,
}


impl JSONTextComponent {
    pub fn new(
            text: Option<String>, 
            translate: Option<String>, 
            keybind: Option<String>, 
            color: Option<String>, 
            bold: Option<bool>,
            italic: Option<bool>,
            underlined: Option<bool>,
            strikethrough: Option<bool>,
            obfuscated: Option<bool>,
        ) -> Self {
        JSONTextComponent {
            text : text,
            translate : translate,
            keybind : keybind,
            color : color,
            bold : bold,
            italic : italic,
            underlined : underlined,
            strikethrough : strikethrough,
            obfuscated : obfuscated,
        }
    }

    pub fn from_str(the_str: &str) -> Self {
        JSONTextComponent::new(
            Some(the_str.to_string()), 
            None, 
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn from_string(the_string: &String) -> Self {
        JSONTextComponent::new(
            Some(the_string.clone()), 
            None, 
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn color(mut self, hex_code: u8) -> Self {
        assert!(hex_code <= 0xf);
        let color = match hex_code {
            0x0 => "black",
            0x1 => "dark_blue",
            0x2 => "dark_green",
            0x3 => "dark_aqua",
            0x4 => "dark_red",
            0x5 => "dark_purple",
            0x6 => "gold",
            0x7 => "gray",
            0x8 => "dark_gray",
            0x9 => "blue",
            0xa => "green",
            0xb => "aqua",
            0xc => "red",
            0xd => "light_purple",
            0xe => "yellow",
            0xf => "white",
            _ => panic!("cosmic rays"),
        };
        self.color = Some(color.to_string());
        self
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ToProtocol for JSONTextComponent {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let the_string = serde_json::to_string(self).unwrap();
        //the_string.pop();
        //the_string.remove(0);
        the_string.to_protocol_bytes()
    }
}