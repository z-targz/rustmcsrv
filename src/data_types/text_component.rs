use serde::{Deserialize, Serialize};

use super::{ToProtocol, NBT};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextComponent {
    is_text_only: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    translatable: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Vec<TextComponent>>,
}
#[derive(Serialize, Deserialize)]
pub struct ClickEvent {
    action: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct HoverEvent {
    action: String,
    contents: Contents,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Contents {
    #[serde(untagged)]
    String {
        #[serde(flatten)]
        component: TextComponent
    },
    #[serde(untagged)]
    ItemInfo {
        id: String, 
        count: i32,
        ///Optional custom sNBT used with give command
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
    },
    #[serde(untagged)]
    EntityInfo{
        ///Entity type identifier. If unrecognized, defaults to minecraft:pig
        r#type: String, 
        ///Entity UUID as String
        id: String,
        ///Optional custon name for entity
        name: Option<String>
    },
}


impl TextComponent {
    pub fn new(text: &str) -> Self {
        TextComponent {
            is_text_only: true,
            text : Some(text.to_string()),
            translatable : None,
            keybind : None,
            color : None,
            bold : None,
            italic : None,
            underlined : None,
            strikethrough : None,
            obfuscated : None,
            extra : None,
        }
    }

    pub fn color(mut self, hex_code: u8) -> Self {
        self.is_text_only = false;
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

    pub fn color_hex(mut self, color: u32) -> Self {
        self.is_text_only = false;
        if color > 0xffffff { panic!("color must be a 24 bit unsigned integer") }
        self.color = Some(format!("#{color:x}"));
        self
    }

    pub fn reset_fmt(mut self) -> Self {
        self.bold = Some(false);
        self.italic = Some(false);
        self.underlined = Some(false);
        self.strikethrough = Some(false);
        self.obfuscated = Some(false);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = Some(true);
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = Some(true);
        self
    }
    pub fn underlined(mut self) -> Self {
        self.underlined = Some(true);
        self
    }
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = Some(true);
        self
    }
    pub fn obfuscated(mut self) -> Self {
        self.obfuscated = Some(true);
        self
    }


    ///Appends another text component to this text component, returning this text component in a builder pattern
    pub fn add_extra(mut self, other: &TextComponent) -> Self {
        if self.extra.is_none() {
            self.extra = Some(Vec::with_capacity(1));
        }
        self.extra.as_mut().unwrap().push(other.clone());
        self
    }

    pub fn to_nbt(&self) -> NBT {
        quartz_nbt::serde::serialize(self, None, quartz_nbt::io::Flavor::Uncompressed).unwrap()
    }

    pub fn get_text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    pub fn get_color(&self) -> Option<&String> {
        self.color.as_ref()
    }
}

impl ToProtocol for TextComponent {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let the_nbt: NBT = quartz_nbt::serde::serialize(self, None, quartz_nbt::io::Flavor::Uncompressed).unwrap();
        //the_string.pop();
        //the_string.remove(0);
        the_nbt.to_protocol_bytes()
    }
}