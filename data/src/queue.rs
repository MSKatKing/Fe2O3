use std::mem;
use num_traits::{FromBytes, ToBytes};

/// A byte queue that allows for numbers to be popped off the end
///
/// This struct can only be built from a Vec<u8> or anything that impls Into<Vec<u8>>, and does not allow data to be pushed to the end.
///
/// Data for the queue is held in heap memory for the entire lifetime of the Queue
pub struct Queue {
    data: Box<[u8]>,
    cursor: usize,
}

impl Queue {
    /// Returns the number of bytes remaining in the queue
    pub fn bytes_left(&self) -> usize {
        self.data.len() - self.cursor
    }

    /// Pops a number from the beginning of the Queue
    ///
    /// T is a data type that impls num_traits::FromBytes
    ///
    /// This function returns `None` when there is not enough space remaining in the Queue for the number to be popped off
    pub fn pop<T: ToBytes + FromBytes>(&mut self) -> Option<T> where <T as FromBytes>::Bytes: From<<T as ToBytes>::Bytes> {
        if size_of::<T>() > self.bytes_left() {
            return None;
        }

        let value = unsafe {
            let mut value = mem::zeroed::<T>();
            std::ptr::copy_nonoverlapping(
                self.data.as_ptr().add(self.cursor),
                &mut value as *mut T as *mut u8,
                size_of::<T>()
            );
            value
        };

        self.cursor += size_of::<T>();

        Some(T::from_be_bytes(&value.to_le_bytes().into()))
    }

    pub fn pop_str(&mut self, len: usize) -> Option<String> {
        let mut bytes = vec![0u8; len];
        for b in &mut bytes {
            *b = self.pop()?;
        }

        String::from_utf8(bytes).ok()
    }
}

impl<T: Into<Vec<u8>>> From<T> for Queue {
    fn from(value: T) -> Self {
        let value = value.into();

        Self {
            data: value.into_boxed_slice(),
            cursor: 0,
        }
    }
}

impl Into<Vec<u8>> for Queue {
    fn into(self) -> Vec<u8> {
        self.data.to_vec()
    }
}