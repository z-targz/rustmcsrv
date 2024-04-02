//TODO: PLEASE verify that this works since changing it.

use std::marker::PhantomData;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::{ToProtocol, NBT};

trait TextComponentType {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Nbt;

impl TextComponentType for Nbt {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Json;

impl TextComponentType for Json {}

#[allow(private_bounds)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
//TODO: Implement serverbound `nbt` content
pub struct TextComponent<T> 
    where T: TextComponentType
{
    #[serde(skip)]
    output_type: PhantomData<T>,

    #[serde(skip)]
    default_formatting: Formatting,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    translate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    keybind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    formatting: Option<Formatting>,

    #[serde(skip_serializing_if = "Option::is_none")]
    insertion: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hoverEvent")]
    hover_event: Option<HoverEvent<T>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clickEvent")]
    click_event: Option<ClickEvent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Vec<TextComponent<T>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClickEvent {
    action: ClickEventAction,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClickEventAction {
    OpenUrl,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

impl ClickEvent {
    pub fn new(action: ClickEventAction, value: &str) -> Self {
        Self {
            action : action,
            value : value.to_owned(),
        }
    }
}

//TODO: Replace this with an enum
#[allow(private_bounds)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HoverEvent<T> 
    where T: TextComponentType
{
    action: HoverEventAction,
    contents: Contents<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HoverEventAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

#[allow(private_bounds)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Contents<T> 
    where T: TextComponentType
{
    #[serde(untagged)]
    String {
        #[serde(flatten)]
        component: Box<TextComponent<T>>
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

#[allow(private_bounds)]
impl<T> HoverEvent<T> 
    where T: TextComponentType
{
    pub fn show_text(text_component: TextComponent<T>) -> Self {
        Self {
            action : HoverEventAction::ShowText,
            contents : Contents::String {
                component : Box::new(text_component),
            }
        }
    }

    pub fn show_item(item_identifier: String, count: i32, snbt: Option<String>) -> Self {
        Self {
            action : HoverEventAction::ShowItem,
            contents : Contents::ItemInfo { 
                id: item_identifier, 
                count: count, 
                tag: snbt,
            }
        }
    }

    pub fn show_entity(entity_identifier: String, entity_uuid: String, custom_name: Option<String>) -> Self {
        Self {
            action : HoverEventAction::ShowEntity,
            contents : Contents::EntityInfo { 
                r#type : entity_identifier,
                id : entity_uuid,
                name : custom_name,
            }
        }
    }
}

#[allow(private_bounds)]
impl<T> TextComponent<T> 
    where T: Debug + Clone + Default + TextComponentType
{
    fn new(
        default_formatting: Formatting,
        text: Option<String>,
        translate: Option<String>,
        keybind: Option<String>,
        formatting: Option<Formatting>,
        insertion: Option<String>,
        hover_event: Option<HoverEvent<T>>,
        click_event: Option<ClickEvent>,
        extra : Option<Vec<TextComponent<T>>>,
    ) -> Self {
        Self {
            output_type: PhantomData,
            default_formatting : default_formatting,
            text : text,
            translate : translate,
            keybind : keybind,
            formatting : formatting,
            insertion : insertion,
            hover_event : hover_event,
            click_event : click_event,
            extra : extra,
        }
    }

    pub fn builder<'a>() -> TextComponentBuilder<'a, NoData, T> {
        TextComponentBuilder::new()
    }

    fn default_formatting(mut self, formatting: Formatting) -> Self {
        self.default_formatting = formatting;
        self
    }

    ///Appends another text component to this text component, returning this text component
    fn add_extra(mut self, other: &TextComponent<T>) -> Self {
        if self.extra.is_none() {
            self.extra = Some(Vec::with_capacity(1));
        }
        self.extra.as_mut().unwrap().push(other.clone().default_formatting(self.default_formatting.clone()));
        self
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

    pub fn get_formatting(&self) -> Option<&Formatting> {
        self.formatting.as_ref()
    }

    pub fn get_extra(&self) -> Option<&Vec<TextComponent<T>>> {
        self.extra.as_ref()
    }

    pub fn has_extra(&self) -> bool {
        self.extra.is_some()
    }

}

impl TextComponent<Nbt> {
    pub fn to_nbt(&self) -> NBT {
        quartz_nbt::serde::serialize(self, None, quartz_nbt::io::Flavor::Uncompressed).unwrap()
    }
}

impl ToProtocol for TextComponent<Nbt> {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_nbt().to_protocol_bytes()
    }
}

impl TextComponent<Json> {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToProtocol for TextComponent<Json> {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_json_string().to_protocol_bytes()
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

    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<String>,
}

impl Formatting {
    fn new(
        color: Option<String>,
        bold: Option<bool>,
        italic: Option<bool>,
        underlined: Option<bool>,
        strikethrough: Option<bool>,
        obfuscated: Option<bool>,
        font: Option<String>,
    ) -> Self {
        Self {
            color : color,
            bold : bold,
            italic : italic,
            underlined : underlined,
            strikethrough : strikethrough,
            obfuscated : obfuscated,
            font : font,
        }
    }

    pub fn builder<'a>() -> FormattingBuilder<'a> {
        FormattingBuilder::new()
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

#[derive(Default, Debug, PartialEq)]
pub struct NoData;

#[allow(private_bounds)]
#[derive(Default, Debug)]
pub struct TextComponentBuilder<'a, S, T> 
    where T: Debug + Default + TextComponentType + Clone,
{
    default_formatting: Formatting,
    text: Option<S>,
    translate: Option<S>,
    keybind: Option<S>,
    formatting: Option<Formatting>,
    insertion: Option<&'a str>,
    hover_event: Option<HoverEvent<T>>,
    click_event: Option<ClickEvent>,
    extra: Option<Vec<TextComponent<T>>>,
}

#[allow(private_bounds)]
impl<'a, T> TextComponentBuilder<'a, NoData, T> 
    where T: Debug + Default + TextComponentType + Clone
{
    fn new() -> TextComponentBuilder<'a, NoData, T> {
        TextComponentBuilder {
            default_formatting: Formatting {
                color : Some("white".into()),
                bold : Some(false),
                italic : Some(false),
                underlined : Some(false),
                strikethrough : Some(false),
                obfuscated : Some(false),
                font : Some("minecraft:default".into()),
            },
            ..Default::default()
        }
    }

    pub fn text(self, text: &str) -> TextComponentBuilder<&str, T> {
        TextComponentBuilder {
            text: Some(text),
            ..Default::default()
        }
    }

    pub fn translate(self, translate: &str) -> TextComponentBuilder<&str, T> {
        TextComponentBuilder {
            translate : Some(translate),
            ..Default::default()
        }
    }

    pub fn keybind(self, keybind: &str) ->  TextComponentBuilder<&str, T> {
        TextComponentBuilder {
            keybind : Some(keybind),
            ..Default::default()
        }
    }

    pub fn default_formatting(mut self, formatting: Formatting) -> Self {
        self.default_formatting = formatting;
        self
    }

    
}

#[allow(private_bounds)]
impl<'a, T> TextComponentBuilder<'a, &str, T> 
    where T: Debug + Default + TextComponentType + Clone
{
    pub fn formatting(mut self, formatting: Formatting) -> Self {
        self.formatting = Some(formatting);
        self
    }

    pub fn reset_fmt(mut self) -> Self {
        self.formatting = Some(self.default_formatting.clone());
        self
    }

    fn add_extra_unchecked(mut self, other: TextComponent<T>) -> Self {
        if self.extra.is_none() {
            self.extra = Some(Vec::with_capacity(1));
        }
        self.extra.as_mut().unwrap().push(other.default_formatting(self.default_formatting.clone()));
        self
    }

    /// Adds an extra text component to this text component chain.\
    /// If the text component being added has its own chain,
    /// it will be flattened and appended to this one, and formatting
    /// will be preserved.
    pub fn add_extra(mut self, mut other: TextComponent<T>) -> Self {
        if !other.has_extra() {
            return self.add_extra_unchecked(other);
        }

        //Remove the extra text components from `other.extra` so we can append them to `self.extra` following `other`
        let other_extra = other.extra.take().unwrap();

        if self.extra.is_none() {
            self.extra = Some(Vec::with_capacity(other_extra.len() + 1));
        }

        let self_extra = self.extra.as_mut().unwrap();

        self_extra.push(other.default_formatting(self.default_formatting.clone()));
        let other_ref = self_extra.last().unwrap();

        //set the default formatting of the children to the formatting or default formatting of `other`
        //we use an extra clone here, but testing will tell if it makes any tangible difference
        let default_formatting = match &other_ref.formatting {
            Some(formatting) => formatting.clone(),
            None => other_ref.default_formatting.clone()
        };
        for extra in other_extra {
            self_extra.push(extra.default_formatting(default_formatting.clone()));
        }
        self
    }

    pub fn build(self) -> TextComponent<T> {
        TextComponent::new(
            self.default_formatting,
            match self.text {
                Some(slice) => Some(slice.to_owned()),
                None => None,
            },
            match self.translate {
                Some(slice) => Some(slice.to_owned()),
                None => None,
            },
            match self.keybind {
                Some(slice) => Some(slice.to_owned()),
                None => None,
            },
            self.formatting,
            match self.insertion {
                Some(slice) => Some(slice.to_owned()),
                None => None,
            },
            self.hover_event,
            self.click_event,
            self.extra,
        )
    }
}


#[derive(Default, Clone)]
pub struct FormattingBuilder<'a> {
    color: Option<String>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    font: Option<&'a str>,
}

impl<'a> FormattingBuilder<'a> {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
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

    pub fn color_rgb(mut self, color: u32) -> Self {
        if color > 0xffffff { panic!("color must be a 24 bit unsigned integer") }
        self.color = Some(format!("#{color:x}"));
        self
    }

    pub fn bold(mut self, val: bool) -> Self {
        self.bold = Some(val);
        self
    }

    pub fn italic(mut self, val: bool) -> Self {
        self.italic = Some(val);
        self
    }

    pub fn underlined(mut self, val: bool) -> Self {
        self.underlined = Some(val);
        self
    }

    pub fn strikethrough(mut self, val: bool) -> Self {
        self.strikethrough = Some(val);
        self
    }

    pub fn obfuscated(mut self, val: bool) -> Self {
        self.obfuscated = Some(val);
        self
    }

    pub fn font(mut self, font: &'a str) -> Self {
        self.font = Some(font);
        self
    }

    pub fn build(self) -> Formatting {
        Formatting {
            color : self.color,
            bold : self.bold,
            underlined : self.underlined,
            italic : self.italic,
            strikethrough : self.strikethrough,
            obfuscated : self.obfuscated,
            font : match self.font {
                Some(slice) => Some(slice.to_owned()),
                None => None,
            },
        }
    }

}