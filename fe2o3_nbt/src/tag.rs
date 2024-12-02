use std::mem::discriminant;
use json::JsonValue;

pub enum NBTTag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NBTTag>),
    Compound(Vec<(String, NBTTag)>),
    CompoundSelfEnd,
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

impl NBTTag {
    pub fn get_id(&self) -> u8 {
        match self {
            NBTTag::End => 0,
            NBTTag::Byte(_) => 1,
            NBTTag::Short(_) => 2,
            NBTTag::Int(_) => 3,
            NBTTag::Long(_) => 4,
            NBTTag::Float(_) => 5,
            NBTTag::Double(_) => 6,
            NBTTag::ByteArray(_) => 7,
            NBTTag::String(_) => 8,
            NBTTag::List(_) => 9,
            NBTTag::Compound(_) => 10,
            NBTTag::CompoundSelfEnd => 10,
            NBTTag::IntArray(_) => 11,
            NBTTag::LongArray(_) => 12,
        }
    }
}

impl Into<NBTTag> for String {
    fn into(self) -> NBTTag {
        NBTTag::String(self)
    }
}

impl Into<NBTTag> for &str {
    fn into(self) -> NBTTag {
        NBTTag::String(self.to_string())
    }
}

impl Into<NBTTag> for u8 {
    fn into(self) -> NBTTag {
        NBTTag::Byte(self as i8)
    }
}

impl Into<NBTTag> for i8 {
    fn into(self) -> NBTTag {
        NBTTag::Byte(self)
    }
}

impl Into<NBTTag> for bool {
    fn into(self) -> NBTTag {
        NBTTag::Byte(self as i8)
    }
}

impl Into<NBTTag> for &JsonValue {
    fn into(self) -> NBTTag {
        match self {
            JsonValue::Null => NBTTag::End,
            JsonValue::Boolean(b) => (*b).into(),
            JsonValue::Number(_) => {
                if let Some(num) = self.as_i64() {
                    if num >= i32::MAX as i64 {
                        NBTTag::Long(num)
                    } else {
                        NBTTag::Int(num as i32)
                    }
                } else {
                    NBTTag::Double(self.as_f64().unwrap())
                }
            },
            JsonValue::String(s) => s.clone().into(),
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    return NBTTag::List(vec![])
                }

                let first_tag: &NBTTag = &(&arr[0]).into();
                let list = arr
                    .iter()
                    .map(|value| value.into())
                    .collect::<Vec<NBTTag>>();

                if list.iter().all(|tag| discriminant(tag) == discriminant(first_tag)) {
                    NBTTag::List(list)
                } else {
                    NBTTag::String("Expected each value in the list to be the same type!".to_string())
                }
            },
            JsonValue::Object(obj) => {
                let out = obj.iter().map(|(name, value)| (name.to_string(), value.into())).collect::<Vec<(String, NBTTag)>>();
                NBTTag::Compound(out)
            },
            JsonValue::Short(s) => NBTTag::String(s.to_string())
        }
    }
}