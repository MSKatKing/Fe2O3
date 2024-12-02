use packet::{Buffer, Deserializable, Serializable, VarInt};

#[derive(Default, Copy, Clone)]
#[repr(u8)]
pub enum ChatMode {
    #[default]
    All,
    CommandsOnly,
    Hidden
}

impl Deserializable for ChatMode {
    fn deserialize(buffer: &mut Buffer) -> Self {
        Self::from(buffer.read::<VarInt>().0)
    }
}

impl Serializable for ChatMode {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(VarInt(self as u8 as i32));
    }
}

impl From<i32> for ChatMode {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::All,
            1 => Self::CommandsOnly,
            2 => Self::Hidden,
            _ => Self::All
        }
    }
}

pub struct ChatSettings {
    pub mode: ChatMode,
    pub show_colors: bool
}

impl Default for ChatSettings {
    fn default() -> Self {
        Self {
            mode: ChatMode::default(),
            show_colors: true
        }
    }
}

impl Serializable for ChatSettings {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(self.mode);
        buffer.write(self.show_colors);
    }
}

impl Deserializable for ChatSettings {
    fn deserialize(buffer: &mut Buffer) -> Self {
        Self {
            mode: buffer.read(),
            show_colors: buffer.read()
        }
    }
}