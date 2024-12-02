use json::JsonValue;
use packet::{Buffer, Deserializable, Identifier, Serializable};
use crate::tag::NBTTag;

pub struct NBT {
    data: Buffer,
    network: bool
}

pub fn pack_entries(entries: &[i32], bpe: usize) -> Vec<i64> {
    let epl: usize = 64 / bpe;

    let mut packed_data = Vec::new();

    let mut i = 0;
    while i * epl < entries.len() {
        packed_data.push(0);
        for j in 0..epl {
            let val = (*entries.get(i * epl + j).unwrap_or(&0) as i64) << ((bpe * j) as i64);
            packed_data[i] |= val;
        }
        i += 1;
    }

    packed_data
}

impl NBT {
    pub fn new(network: bool) -> Self {
        Self {
            data: Buffer::new(),
            network
        }
    }

    pub fn from_json(json: &JsonValue, network: bool) -> Self {
        let mut nbt = NBT::new(network);

        if let JsonValue::Object(_) = json {
            nbt.write_tag("", json);
            nbt
        } else {
            panic!("Top-level JSON must be an object");
        }
    }

    pub fn from_registry(json: JsonValue, network: bool) -> (Identifier, Vec<(Identifier, NBT)>) {
        if let JsonValue::Object(obj) = json {
            let obj = obj.iter().collect::<Vec<_>>();
            let mut out = (Identifier::new("", ""), vec![]);
            let (registry_name, entries) = obj[0];

            out.0 = Identifier::from(registry_name.to_string());

            if let JsonValue::Object(obj) = entries {
                for (registry_id, nbt) in obj.iter() {
                    out.1.push((Identifier::from(registry_id.to_string()), NBT::from_json(nbt, network)));
                }
            } else {
                panic!("Expected an object of registry values!");
            }

            out
        } else {
            panic!("Top-level JSON must be an object!");
        }
    }

    pub fn write_tag(&mut self, name: &str, tag: impl Into<NBTTag>) {
        let tag = tag.into();

        self.data.write(tag.get_id());
        match tag {
            NBTTag::End => {},
            NBTTag::Byte(b) => {
                self.write_name(name);
                self.data.write(b);
            },
            NBTTag::Short(s) => {
                self.write_name(name);
                self.data.write(s);
            },
            NBTTag::Int(i) => {
                self.write_name(name);
                self.data.write(i);
            },
            NBTTag::Long(l) => {
                self.write_name(name);
                self.data.write(l);
            },
            NBTTag::Float(f) => {
                self.write_name(name);
                self.data.write(f);
            },
            NBTTag::Double(d) => {
                self.write_name(name);
                self.data.write(d);
            },
            NBTTag::List(tags) => {
                self.write_name(name);
                if tags.len() == 0 {
                    self.data.write(0);
                } else {
                    self.data.write(tags[0].get_id());
                }

                self.data.write(tags.len() as i32);
                tags.into_iter().for_each(|tag| self.write_list_tag(tag));
            },
            NBTTag::Compound(inner) => {
                if !self.network {
                    self.write_name(name);
                }
                self.network = false;

                inner.into_iter().for_each(|(name, tag)| self.write_tag(&name, tag));
                self.write_tag("", NBTTag::End);
            },
            NBTTag::CompoundSelfEnd => {
                if !self.network {
                    self.write_name(name);
                }
                self.network = false;
            },
            NBTTag::LongArray(arr) => {
                self.write_name(name);
                self.data.write(arr.len() as i32);
                arr.iter().for_each(|data| self.data.write(*data));
            },
            NBTTag::String(str) => {
                self.write_name(name);
                self.data.write(str.len() as u16);
                self.data.write(str.as_bytes());
            },
            NBTTag::IntArray(arr) => {
                self.write_name(name);
                self.data.write(arr.len() as i32);
                arr.iter().for_each(|data| self.data.write(*data));
            }
            NBTTag::ByteArray(arr) => {
                self.write_name(name);
                self.data.write(arr.len() as i32);
                arr.iter().for_each(|data| self.data.write(*data));
            }
        }
    }

    fn write_list_tag(&mut self, tag: NBTTag) {
        match tag {
            NBTTag::End => {}
            NBTTag::Byte(b) => self.data.write(b),
            NBTTag::Short(s) => self.data.write(s),
            NBTTag::Int(i) => self.data.write(i),
            NBTTag::Long(l) => self.data.write(l),
            NBTTag::Float(f) => self.data.write(f),
            NBTTag::Double(d) => self.data.write(d),
            NBTTag::ByteArray(arr) => {
                self.data.write(arr.len() as i32);
                arr.iter().for_each(|data| self.data.write(*data))
            }
            NBTTag::String(str) => {
                self.data.write(str.len() as u16);
                self.data.write(str.as_bytes());
            }
            NBTTag::List(tags) => {
                self.data.write(tags[0].get_id());
                self.data.write(tags.len() as i32);
                tags.into_iter().for_each(|tag| self.write_list_tag(tag));
            }
            NBTTag::Compound(inner) => {
                inner.into_iter().for_each(|(name, tag)| self.write_tag(&name, tag));
                self.write_tag("", NBTTag::End);
            }
            NBTTag::CompoundSelfEnd => {}
            NBTTag::IntArray(arr) => {
                self.data.write(arr.len() as i32);
                arr.iter().for_each(|data| self.data.write(*data))
            }
            NBTTag::LongArray(arr) => {
                self.data.write(arr.len() as i32);
                arr.iter().for_each(|data| self.data.write(*data))
            }
        }
    }

    fn write_name(&mut self, name: &str) {
        self.data.write(name.len() as u16);
        self.data.write(name.as_bytes());
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data.buffer
    }
}

impl Deserializable for NBT {
    fn deserialize(_: &mut Buffer) -> Self {
        // TODO: read
        Self::new(true)
    }
}

impl Serializable for NBT {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(self.as_bytes())
    }
}