use crate::nbt::tag::NBTTag;

pub struct TextComponent {
    pub text: String,
}

impl Into<NBTTag> for TextComponent {
    fn into(self) -> NBTTag {
        NBTTag::Compound(
            vec![
                ("type".to_string(), NBTTag::String("text".to_string())),
                ("text".to_string(), NBTTag::String(self.text)),
            ]
        )
    }
}