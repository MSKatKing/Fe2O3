use std::fmt::{Display, Formatter};
use fe2o3_nbt::{compound_nbt, NBT};
use packet::{Buffer, Deserializable, Serializable};

pub struct Component {
    text: String,
    extra: Option<Vec<Component>>,
    style: Option<TextStyle>,
    color: Option<TextColor>
}

impl Component {
    pub fn append(&mut self, other: Component) -> &mut Self {
        if let Some(extra) = &mut self.extra {
            extra.push(other)
        } else {
            self.extra = Some(Vec::new());

            let extra = self.extra.as_mut().unwrap();
            extra.push(other);
        }

        self
    }

    pub fn style(&mut self, style: TextStyle) -> &mut Self {
        self.style = Some(style);

        self
    }

    pub fn color(&mut self, color: TextColor) -> &mut Self {
        self.color = Some(color);

        self
    }

    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            extra: None,
            style: None,
            color: None
        }
    }

    pub fn new_with_color(text: impl Into<String>, color: TextColor) -> Self {
        Self {
            text: text.into(),
            extra: None,
            style: None,
            color: Some(color)
        }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"text\":\"{}\"", self.text)?;

        if let Some(color) = &self.color {
            write!(f, ",\"color\":\"{}\"", color)?;
        }

        if let Some(style) = &self.style {
            write!(f, ",\"{}\":true", style)?;
        }

        if let Some(extra) = &self.extra {
            write!(f, ",\"extra\":[")?;

            for (i, component) in extra.iter().enumerate() {
                write!(f, "{}{}", if i == 0 { "" } else { "," }, component)?;
            }

            write!(f, "]")?;
        }

        write!(f, "}}")
    }
}

impl Serializable for Component {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(format!("{self}"));
    }
}

impl Deserializable for Component {
    fn deserialize(_: &mut Buffer) -> Self {
        todo!()
    }
}

#[repr(u8)]
pub enum TextColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    BrightGreen,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    Other(u32)
}

impl Display for TextColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextColor::Black => write!(f, "black"),
            TextColor::DarkBlue => write!(f, "dark_blue"),
            TextColor::DarkGreen => write!(f, "dark_green"),
            TextColor::DarkCyan => write!(f, "dark_cyan"),
            TextColor::DarkRed => write!(f, "dark_red"),
            TextColor::Purple => write!(f, "purple"),
            TextColor::Gold => write!(f, "gold"),
            TextColor::Gray => write!(f, "gray"),
            TextColor::DarkGray => write!(f, "dark_gray"),
            TextColor::Blue => write!(f, "blue"),
            TextColor::BrightGreen => write!(f, "bright_green"),
            TextColor::Cyan => write!(f, "cyan"),
            TextColor::Red => write!(f, "red"),
            TextColor::Pink => write!(f, "pink"),
            TextColor::Yellow => write!(f, "yellow"),
            TextColor::White => write!(f, "white"),
            TextColor::Other(hex) => write!(f, "#{:x}", hex)
        }
    }
}

pub enum TextStyle {
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic
}

impl Display for TextStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextStyle::Obfuscated => write!(f, "obfuscated"),
            TextStyle::Bold => write!(f, "bold"),
            TextStyle::Strikethrough => write!(f, "strikethrough"),
            TextStyle::Underline => write!(f, "underline"),
            TextStyle::Italic => write!(f, "italic")
        }
    }
}

impl Into<NBT> for Component {
    fn into(self) -> NBT {
        let mut nbt = NBT::new(true);

        nbt.write_tag("", compound_nbt!()); //TODO: this

        nbt
    }
}