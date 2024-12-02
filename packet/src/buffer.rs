use std::mem;
use std::ops::Deref;
use num_traits::{FromBytes, ToBytes};
use crate::deserializable::Deserializable;
use crate::serializable::Serializable;

#[derive(Default, shipyard::Component)]
pub struct Buffer {
    pub buffer: Vec<u8>,
    pub cursor: usize
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            cursor: 0
        }
    }

    pub fn has_space_left(&self, size: usize) -> Result<(), usize> {
        let new_len = self.cursor + size;

        if new_len > self.buffer.len() {
            Err(new_len - self.buffer.len())
        } else {
            Ok(())
        }
    }

    pub(crate) fn read_numeric<T: ToBytes + FromBytes>(&mut self) -> T where <T as FromBytes>::Bytes: From<<T as ToBytes>::Bytes> {
        let size = size_of::<T>();

        if let Err(overflow) = self.has_space_left(size) {
            self.buffer.resize(self.buffer.len() + overflow, 0);
        }

        let value = unsafe {
            let mut value = mem::zeroed::<T>();
            std::ptr::copy_nonoverlapping(
                self.buffer.as_ptr().add(self.cursor),
                &mut value as *mut T as *mut u8,
                size,
            );
            value
        };

        self.cursor += size;

        T::from_be_bytes(&value.to_le_bytes().into())
    }
    pub(crate) fn write_numeric<T: ToBytes>(&mut self, value: T) {
        let size = size_of::<T>();

        let bytes = value.to_be_bytes();

        if let Err(overflow) = self.has_space_left(size) {
            self.buffer.resize(self.buffer.len() + overflow, 0);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ref().as_ptr(),
                self.buffer.as_mut_ptr().add(self.cursor),
                size
            )
        }

        self.cursor += size;
    }

    pub fn read<T: Deserializable>(&mut self) -> T {
        T::deserialize(self)
    }
    pub fn write<T: Serializable>(&mut self, value: T) {
        value.serialize(self);
    }
}

impl From<&[u8]> for Buffer {
    fn from(value: &[u8]) -> Self {
        Self {
            cursor: 0,
            buffer: Vec::from(value)
        }
    }
}

impl From<&mut Buffer> for Buffer {
    fn from(value: &mut Buffer) -> Self {
        Self {
            cursor: value.cursor,
            buffer: value.buffer.clone()
        }
    }
}

impl Deref for Buffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}