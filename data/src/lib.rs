pub mod nbt;
mod queue;
mod text;

#[cfg(test)]
mod tests {
    use crate::nbt::tag::NBTTag;
    use crate::nbt::NBT;
    use std::fs;

    #[test]
    fn test_nbt_save() {
        let nbt = NBT::new("hello world", NBTTag::Compound(vec![("name".to_string(), NBTTag::String("Bananrama".to_string()))]));
        let serialize = nbt.as_bytes();

        fs::write("test1.nbt", serialize.clone()).expect("failed");

        assert_eq!(serialize, vec![0x0a, 0x00, 0x0b, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x08, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x09, 0x42, 0x61, 0x6e, 0x61, 0x6e, 0x72, 0x61, 0x6d, 0x61, 0x00]);
    }

    #[test]
    fn test_nbt_read() {
        let bytes = fs::read("test3.nbt").expect("failed");
        let nbt = NBT::from_bytes(bytes, false).expect("fad");

        assert_eq!(nbt, NBT::new("hello world", NBTTag::Compound(vec![("name".to_string(), NBTTag::String("Bananramas".to_string())), ("my bytes".to_string(), NBTTag::ByteArray(vec![0, 0, 0, 0, 0, 0])), ("10000".to_string(), NBTTag::Short(10000))])))
    }
}
