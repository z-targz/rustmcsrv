

use serde::{Serialize, Deserialize};

use super::{text_component::Formatting, TextComponent, ToProtocol};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JSONTextComponent {
    #[serde(flatten)]
    pub text_component: TextComponent,
}

#[allow(unused)]
impl JSONTextComponent {
    pub fn new(text_component: TextComponent) -> Self {
        Self {
            text_component: text_component, 
        }
    }

    pub fn default_formatting(mut self, formatting: Formatting) -> Self {
        self.text_component = self.text_component.default_formatting(formatting);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text_component = self.text_component.text(text);
        self
    }

    pub fn translate(mut self, translate: &str) -> Self {
        self.text_component = self.text_component.translate(translate);
        self
    }

    pub fn keybind(mut self, keybind: &str) -> Self {
        self.text_component = self.text_component.keybind(keybind);
        self
    }

    pub fn color(mut self, hex_code: u8) -> Self {
        self.text_component = self.text_component.color(hex_code);
        self
    }

    pub fn color_rgb(mut self, color: u32) -> Self {
        self.text_component = self.text_component.color_rgb(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.text_component = self.text_component.bold();
        self
    }
    pub fn italic(mut self) -> Self {
        self.text_component = self.text_component.italic();
        self
    }
    pub fn underlined(mut self) -> Self {
        self.text_component = self.text_component.underlined();
        self
    }
    pub fn strikethrough(mut self) -> Self {
        self.text_component = self.text_component.strikethrough();
        self
    }
    pub fn obfuscated(mut self) -> Self {
        self.text_component = self.text_component.obfuscated();
        self
    }

    pub fn formatting(mut self, formatting: Formatting) -> Self {
        self.text_component = self.text_component.formatting(formatting);
        self
    }

    pub fn reset_fmt(mut self) -> Self {
        self.text_component = self.text_component.reset_fmt();
        self
    }

    ///Appends another text component to this text component, returning this text component in a builder pattern
    pub fn add_extra(mut self, other: &JSONTextComponent) -> Self {
        self.text_component = self.text_component.add_extra(&other.text_component);
        self
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self.text_component).unwrap()
    }

}

impl ToProtocol for JSONTextComponent {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_string().to_protocol_bytes()
    }
}

impl From<TextComponent> for JSONTextComponent {
    fn from(value: TextComponent) -> Self {
        Self::new(value)
    }
}