use std::fmt::{Display, Formatter};
use crate::queue::Queue;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Debug)]
pub enum NBTTag {
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
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

pub enum NBTTagDeserializeError {
    MissingID,
    MissingName,
    MissingLength,
    MissingData,
}

impl NBTTag {
    pub fn id(&self) -> u8 {
        match self {
            Self::Byte(_) => 1,
            Self::Short(_) => 2,
            Self::Int(_) => 3,
            Self::Long(_) => 4,
            Self::Float(_) => 5,
            Self::Double(_) => 6,
            Self::ByteArray(_) => 7,
            Self::String(_) => 8,
            Self::List(_) => 9,
            Self::Compound(_) => 10,
            Self::IntArray(_) => 11,
            Self::LongArray(_) => 12,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Byte(_) => "Byte",
            Self::Short(_) => "Short",
            Self::Int(_) => "Int",
            Self::Long(_) => "Long",
            Self::Float(_) => "Float",
            Self::Double(_) => "Double",
            Self::ByteArray(_) => "ByteArray",
            Self::String(_) => "String",
            Self::List(_) => "List",
            Self::Compound(_) => "Compound",
            Self::IntArray(_) => "IntArray",
            Self::LongArray(_) => "LongArray",
        }
    }

    pub fn get(&self, str: impl Into<String>) -> Option<&NBTTag> {
        let Self::Compound(inner) = self else {
            return None;
        };

        let str = str.into();

        for (name, data) in inner {
            if name == &str {
                return Some(data);
            }
        }

        None
    }

    pub fn get_mut(&mut self, str: impl Into<String>) -> Option<&mut NBTTag> {
        let Self::Compound(inner) = self else {
            return None;
        };

        let str = str.into();

        for (name, data) in inner {
            if name == &str {
                return Some(data);
            }
        }

        None
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            NBTTag::Byte(b) => b.to_be_bytes().to_vec(),
            NBTTag::Short(s) => s.to_be_bytes().to_vec(),
            NBTTag::Int(i) => i.to_be_bytes().to_vec(),
            NBTTag::Long(l) => l.to_be_bytes().to_vec(),
            NBTTag::Float(f) => f.to_be_bytes().to_vec(),
            NBTTag::Double(d) => d.to_be_bytes().to_vec(),
            NBTTag::ByteArray(ba) => {
                let mut out = Vec::with_capacity(ba.len() + size_of::<i32>());
                out.extend((ba.len() as i32).to_be_bytes());

                for b in ba {
                    out.push(*b);
                }

                out
            }
            NBTTag::String(s) => {
                let mut out = Vec::with_capacity(s.len() + size_of::<u16>());

                out.extend((s.len() as u16).to_be_bytes());
                out.extend(s.bytes());

                out
            }
            NBTTag::List(l) => {
                let list_id = l.get(0).and_then(|tag| Some(tag.id())).unwrap_or_default();

                let mut out = Vec::new();
                out.push(list_id);
                out.extend((l.len() as i32).to_be_bytes());

                for tag in l {
                    if tag.id() != list_id {
                        panic!("Every NBT Tag in a List Tag must be the same type!");
                    }

                    out.extend(tag.as_bytes());
                }

                out
            }
            NBTTag::Compound(c) => {
                let mut out = Vec::new();

                for (name, tag) in c {
                    out.push(tag.id());
                    out.extend((name.len() as u16).to_be_bytes());
                    out.extend(name.bytes());
                    out.extend(tag.as_bytes());
                }

                out.push(0);

                out
            }
            NBTTag::IntArray(ia) => {
                let mut out = Vec::with_capacity((ia.len() * size_of::<i32>()) + size_of::<i32>());
                out.extend((ia.len() as i32).to_be_bytes());

                for i in ia {
                    out.extend(i.to_be_bytes())
                }

                out
            }
            NBTTag::LongArray(la) => {
                let mut out = Vec::with_capacity((la.len() * size_of::<i64>()) + size_of::<i32>());
                out.extend((la.len() as i32).to_be_bytes());

                for l in la {
                    out.extend(l.to_be_bytes())
                }

                out
            }
        }
    }

    // TODO: change the return to a Result with the error given
    pub fn from_bytes(queue: &mut Queue, id: u8) -> Option<NBTTag> {
        match id {
            1 => Some(NBTTag::Byte(queue.pop().expect(""))),
            2 => Some(NBTTag::Short(queue.pop().expect(""))),
            3 => Some(NBTTag::Int(queue.pop().expect(""))),
            4 => Some(NBTTag::Long(queue.pop().expect(""))),
            5 => Some(NBTTag::Float(queue.pop().expect(""))),
            6 => Some(NBTTag::Double(queue.pop().expect(""))),
            7 => {
                let len = queue.pop::<i32>().expect("") as usize;
                let mut bytes = vec![0u8; len];
                for b in &mut bytes {
                    *b = queue.pop().expect("");
                }

                Some(NBTTag::ByteArray(bytes))
            }
            8 => {
                let len = queue.pop::<u16>().expect("") as usize;
                Some(NBTTag::String(queue.pop_str(len).expect("")))
            }
            9 => {
                let list_id = queue.pop::<u8>().expect("");
                let len = queue.pop::<i32>().expect("");
                if len <= 0 {
                    Some(NBTTag::List(vec![]))
                } else {
                    let mut list = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        list.push(Self::from_bytes(queue, list_id).expect(""));
                    }

                    Some(NBTTag::List(list))
                }
            }
            10 => {
                let mut vec = Vec::new();
                let mut popped_id = queue.pop::<u8>().expect("");
                while popped_id != 0 {
                    let name_len = queue.pop::<u16>().expect("") as usize;
                    let name = queue.pop_str(name_len).expect("");

                    vec.push((name, Self::from_bytes(queue, popped_id).expect("")));

                    popped_id = queue.pop::<u8>().expect("");
                }

                Some(NBTTag::Compound(vec))
            }
            11 => {
                let len = queue.pop::<i32>().expect("") as usize;
                let mut ints = vec![0i32; len];
                for i in &mut ints {
                    *i = queue.pop().expect("");
                }

                Some(NBTTag::IntArray(ints))
            }
            12 => {
                {
                    let len = queue.pop::<i32>().expect("") as usize;
                    let mut longs = vec![0i64; len];
                    for l in &mut longs {
                        *l = queue.pop().expect("");
                    }

                    Some(NBTTag::LongArray(longs))
                }
            }
            _ => None
        }
    }
}

impl Display for NBTTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NBTTag::Byte(b) => write!(f, "{b}"),
            NBTTag::Short(s) => write!(f, "{s}"),
            NBTTag::Int(i) => write!(f, "{i}"),
            NBTTag::Long(l) => write!(f, "{l}"),
            NBTTag::Float(f2) => write!(f, "{f2}"),
            NBTTag::Double(d) => write!(f, "{d}"),
            NBTTag::ByteArray(ba) => write!(f, "{ba:?}"),
            NBTTag::String(s) => write!(f, "'{s}'"),
            NBTTag::List(l) => {
                writeln!(f, "{} entries {{", l.len())?;

                for tag in l {
                    writeln!(f, "\t{}: {}", tag.name(), tag)?;
                }

                write!(f, "}}")
            }
            NBTTag::Compound(c) => {
                writeln!(f, "{} entries {{", c.len())?;

                for (name, tag) in c {
                    writeln!(f, "\t{}('{name}'): {}", tag.name(), tag)?;
                }

                write!(f, "}}")
            }
            NBTTag::IntArray(ia) => write!(f, "{ia:?}"),
            NBTTag::LongArray(la) => write!(f, "{la:?}"),
        }
    }
}

impl Index<&str> for NBTTag {
    type Output = NBTTag;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Self::Compound(inner) => {
                for (name, nbt_tag) in inner {
                    if name == index {
                        return nbt_tag;
                    }
                }
            },
            _ => panic!("Attempted to index into a non-compound NBT Tag.")
        }

        panic!("Could not find tag '{}' in NBT Tag.", index);
    }
}
impl IndexMut<&str> for NBTTag {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match self {
            Self::Compound(inner) => {
                for (name, nbt_tag) in inner {
                    if name == index {
                        return nbt_tag;
                    }
                }
            },
            _ => panic!("Attempted to index into a non-compound NBT Tag.")
        }

        panic!("Could not find tag '{}' in NBT Tag.", index);
    }
}