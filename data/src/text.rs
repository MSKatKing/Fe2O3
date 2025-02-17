use crate::nbt::tag::NBTTag;
use crate::resource::Identifier;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

// TODO: figure out how the NBT values Text Component type works and implement it here
enum TextComponentType {
    Text { text: String },
    Translatable { translate: String, fallback: Option<String>, with: Option<Vec<TextComponent>> },
    ScoreboardValue { score_holder: String, objective: String },
    EntityNames { selector: String, separator: Option<TextComponent> },
    Keybind { keybind: String },
}

pub enum TextColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Custom(u32),
}

impl Display for TextColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextColor::Black => write!(f, "black"),
            TextColor::DarkBlue => write!(f, "dark_blue"),
            TextColor::DarkGreen => write!(f, "dark_green"),
            TextColor::DarkAqua => write!(f, "dark_aqua"),
            TextColor::DarkRed => write!(f, "dark_red"),
            TextColor::DarkPurple => write!(f, "dark_purple"),
            TextColor::Gold => write!(f, "gold"),
            TextColor::Gray => write!(f, "gray"),
            TextColor::DarkGray => write!(f, "dark_gray"),
            TextColor::Blue => write!(f, "blue"),
            TextColor::Green => write!(f, "green"),
            TextColor::Aqua => write!(f, "aqua"),
            TextColor::Red => write!(f, "red"),
            TextColor::LightPurple => write!(f, "light_purple"),
            TextColor::Yellow => write!(f, "yellow"),
            TextColor::White => write!(f, "white"),
            TextColor::Custom(hex) => write!(f, "#{hex:x}")
        }
    }
}

pub enum HoverEvent {
    ShowText { text: TextComponent },
    ShowItem { id: Identifier, count: u8, },
    ShowEntity { name: Option<TextComponent>, ty: Identifier, id: Uuid }
}

pub enum ClickEvent {
    OpenURL(String),
    OpenFile(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(String),
    CopyToClipboard(String),
}

pub struct TextComponent {
    content: TextComponentType,
    extra: Option<Vec<TextComponent>>,
    color: Option<TextColor>,
    font: Option<Identifier>,
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
    include: u8,
    insertion: Option<String>,
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,
}

impl TextComponent {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: TextComponentType::Text { text: text.into() },
            .. Default::default()
        }
    }
    pub fn text_color(text: impl Into<String>, color: TextColor) -> Self {
        Self {
            content: TextComponentType::Text { text: text.into() },
            color: Some(color),
            .. Default::default()
        }
    }
    pub fn translatable(translate: impl Into<String>, fallback: Option<impl Into<String>>, with: Option<Vec<TextComponent>>) -> Self {
        Self {
            content: TextComponentType::Translatable { translate: translate.into(), fallback: fallback.and_then(|fallback| Some(fallback.into())), with },
            .. Default::default()
        }
    }
    pub fn scoreboard(score_holder: impl Into<String>, objective: impl Into<String>) -> Self {
        Self {
            content: TextComponentType::ScoreboardValue { score_holder: score_holder.into(), objective: objective.into() },
            .. Default::default()
        }
    }
    pub fn keybind(keybind: impl Into<String>) -> Self {
        Self {
            content: TextComponentType::Keybind { keybind: keybind.into() },
            .. Default::default()
        }
    }

    pub fn append(mut self, other: TextComponent) -> Self { self.extra.as_mut().and_then(|extras| Some(extras.push(other))); self }
    pub fn with_color(mut self, color: TextColor) -> Self { self.color = Some(color); self }
    pub fn with_font(mut self, font: Identifier) -> Self { self.font = Some(font); self }
    pub fn bold(mut self) -> Self { self.bold = true; self.include |= 0x1; self }
    pub fn not_bold(mut self) -> Self { self.bold = false; self.include |= 0x1; self }
    pub fn italic(mut self) -> Self { self.italic = true; self.include |= 0x2; self }
    pub fn not_italic(mut self) -> Self { self.italic = false; self.include |= 0x2; self }
    pub fn underlined(mut self) -> Self { self.underlined = true; self.include |= 0x4; self }
    pub fn not_underlined(mut self) -> Self { self.underlined = false; self.include |= 0x4; self }
    pub fn strikethrough(mut self) -> Self { self.strikethrough = true; self.include |= 0x8; self }
    pub fn not_strikethrough(mut self) -> Self { self.strikethrough = false; self.include |= 0x8; self }
    pub fn obfuscated(mut self) -> Self { self.obfuscated = true; self.include |= 0x10; self }
    pub fn not_obfuscated(mut self) -> Self { self.obfuscated = false; self.include |= 0x10; self }
    pub fn on_hover(mut self, event: HoverEvent) -> Self { self.hover_event = Some(event); self }
    pub fn on_click(mut self, event: ClickEvent) -> Self { self.click_event = Some(event); self }
    pub fn shift_click(mut self, insertion: impl Into<String>) -> Self { self.insertion = Some(insertion.into()); self }
}