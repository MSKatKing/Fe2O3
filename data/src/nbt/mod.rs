use crate::nbt::tag::NBTTag;
use crate::queue::Queue;
use flate2::bufread::{GzDecoder, ZlibDecoder};
use std::fmt::{Display, Formatter};
use std::io::Read;

// TODO: better error handling for parsing a Vec<u8>,
// TODO: SNBT saving and parsing

pub mod tag;

#[derive(Debug, PartialEq)]
pub struct NBT {
    root_name: Option<String>,
    root_tag: NBTTag,
}

impl NBT {
    pub fn new(name: impl Into<String>, root_tag: NBTTag) -> Self {
        Self {
            root_name: Some(name.into()),
            root_tag,
        }
    }

    pub fn new_network(root_tag: NBTTag) -> Self {
        Self {
            root_name: None,
            root_tag,
        }
    }

    pub fn as_network(&mut self) {
        self.root_name = None;
    }

    pub fn as_normal(&mut self, name: impl Into<String>) {
        self.root_name = Some(name.into());
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(self.root_tag.id());
        if let Some(name) = &self.root_name {
            out.extend((name.len() as u16).to_be_bytes());
            out.extend(name.bytes());
        }

        out.extend(self.root_tag.as_bytes());

        out
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>, network: bool) -> Option<Self> {
        let mut bytes = bytes.into();
        if bytes.starts_with(&[0x1F, 0x8B]) {
            let copied = bytes.clone();
            let mut decoder = GzDecoder::new(&copied[..]);
            bytes.clear();

            decoder.read_to_end(&mut bytes).ok()?;
        } else if bytes.starts_with(&[0x78, 0x01]) || bytes.starts_with(&[0x78, 0x9C]) || bytes.starts_with(&[0x78, 0xDA]) {
            let copied = bytes.clone();
            let mut decoder = ZlibDecoder::new(&copied[..]);
            bytes.clear();

            decoder.read_to_end(&mut bytes).ok()?;
        }

        let mut queue: Queue = bytes.into();
        let id = queue.pop::<u8>().expect("");
        let mut name = None;

        if !network {
            let name_len = queue.pop::<u16>().expect("") as usize;
            name = Some(queue.pop_str(name_len).expect(""));
        }

        Some(Self {
            root_name: name,
            root_tag: NBTTag::from_bytes(&mut queue, id).expect(""),
        })
    }
}

impl Display for NBT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NBT({:?}): {}", self.root_name, self.root_tag)
    }
}
