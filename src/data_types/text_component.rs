use serde::{Deserialize, Serialize};

use super::{ToProtocol, NBT};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextComponent {

    #[serde(skip)]
    default_formatting: Formatting,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    translate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    keybind: Option<String>,

    #[serde(flatten)]
    formatting: Formatting,

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
    pub fn new(
        default_formatting: Formatting,
        text: Option<String>,
        translate: Option<String>,
        keybind: Option<String>,
        formatting: Formatting,
        extra : Option<Vec<TextComponent>>,
    ) -> Self {
        Self {
            default_formatting : default_formatting,
            text : text,
            translate : translate,
            keybind : keybind,
            formatting : formatting,
            extra : extra,
        }
    }

    pub fn default() -> Self {
        Self {
            default_formatting : Formatting::default(),
            ..Default::default()
        }
    }

    pub fn default_formatting(mut self, formatting: Formatting) -> Self {
        self.default_formatting = formatting;
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn translate(mut self, translate: &str) -> Self {
        self.translate = Some(translate.to_string());
        self
    }

    pub fn keybind(mut self, keybind: &str) -> Self {
        self.keybind = Some(keybind.to_string());
        self
    }

    pub fn formatting(mut self, formatting: Formatting) -> Self {
        self.formatting = formatting;
        self
    }

    pub fn color(mut self, hex_code: u8) -> Self {
        self.formatting.set_color(hex_code);
        self
    }

    pub fn color_rgb(mut self, color: u32) -> Self {
        self.formatting.set_color_rgb(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.formatting.set_bold(true);
        self
    }
    pub fn italic(mut self) -> Self {
        self.formatting.set_italic(true);
        self
    }
    pub fn underlined(mut self) -> Self {
        self.formatting.set_underlined(true);
        self
    }
    pub fn strikethrough(mut self) -> Self {
        self.formatting.set_underlined(true);
        self
    }
    pub fn obfuscated(mut self) -> Self {
        self.formatting.set_obfuscated(true);
        self
    }

    pub fn reset_fmt(mut self) -> Self {
        self.formatting = self.default_formatting.clone();
        self
    }

    ///Appends another text component to this text component, returning this text component in a builder pattern
    pub fn add_extra(mut self, other: &TextComponent) -> Self {
        if self.extra.is_none() {
            self.extra = Some(Vec::with_capacity(1));
        }
        self.extra.as_mut().unwrap().push(other.clone().default_formatting(self.default_formatting.clone()));
        self
    }

    pub fn to_nbt(&self) -> NBT {
        quartz_nbt::serde::serialize(self, None, quartz_nbt::io::Flavor::Uncompressed).unwrap()
    }

    pub fn get_text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    pub fn get_translate(&self) -> Option<&String> {
        self.translate.as_ref()
    }

    pub fn get_keybind(&self) -> Option<&String> {
        self.keybind.as_ref()
    }

    pub fn get_formatting(&self) -> &Formatting {
        &self.formatting
    }

    

    
}

impl ToProtocol for TextComponent {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_nbt().to_protocol_bytes()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Formatting {
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

impl Formatting {

    pub fn new(
        color: Option<String>,
        bold: Option<bool>,
        italic: Option<bool>,
        underlined: Option<bool>,
        strikethrough: Option<bool>,
        obfuscated: Option<bool>,
    ) -> Self {
        Self {
            color : color,
            bold : bold,
            italic : italic,
            underlined : underlined,
            strikethrough : strikethrough,
            obfuscated : obfuscated,
        }
    }

    pub fn default() -> Self {
        Self {
            color : Some("white".to_string()),
            bold : Some(false),
            italic : Some(false),
            underlined : Some(false),
            strikethrough : Some(false),
            obfuscated : Some(false),
        }
    }

    pub fn set_color(&mut self, hex_code: u8) {
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
    }

    pub fn set_color_rgb(&mut self, color: u32) {
        if color > 0xffffff { panic!("color must be a 24 bit unsigned integer") }
        self.color = Some(format!("#{color:x}"));
    }

    pub fn set_bold(&mut self, val: bool) {
        self.bold = Some(val);
    }
    pub fn set_italic(&mut self, val: bool) {
        self.italic = Some(val);
    }
    pub fn set_underlined(&mut self, val: bool) {
        self.underlined = Some(val);
    }
    pub fn set_strikethrough(&mut self, val: bool) {
        self.strikethrough = Some(val);
    }
    pub fn set_obfuscated(&mut self, val: bool) {
        self.obfuscated = Some(val);
    }

    pub fn get_color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    pub fn get_bold(&self) -> Option<bool> {
        self.bold
    }

    pub fn get_italic(&self) -> Option<bool> {
        self.italic
    }

    pub fn get_underlined(&self) -> Option<bool> {
        self.underlined
    }

    pub fn get_strikethrough(&self) -> Option<bool> {
        self.strikethrough
    }

    pub fn get_obfuscated(&self) -> Option<bool> {
        self.obfuscated
    }
}