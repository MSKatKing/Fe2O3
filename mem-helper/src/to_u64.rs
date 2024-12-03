pub trait ToU64 {
    fn transmute_to_u64(self) -> u64;
}

impl ToU64 for u8 {
    fn transmute_to_u64(self) -> u64 { self as u64 }
}
impl ToU64 for u16 {
    fn transmute_to_u64(self) -> u64 { self as u64 }
}
impl ToU64 for u32 {
    fn transmute_to_u64(self) -> u64 { self as u64 }
}
impl ToU64 for u64 {
    fn transmute_to_u64(self) -> u64 { self }
}

impl ToU64 for i8 {
    fn transmute_to_u64(self) -> u64 {
        unsafe {
            std::mem::transmute(self as i64)
        }
    }
}
impl ToU64 for i16 {
    fn transmute_to_u64(self) -> u64 {
        unsafe {
            std::mem::transmute(self as i64)
        }
    }
}
impl ToU64 for i32 {
    fn transmute_to_u64(self) -> u64 {
        unsafe {
            std::mem::transmute(self as i64)
        }
    }
}
impl ToU64 for i64 {
    fn transmute_to_u64(self) -> u64 {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl ToU64 for f32 {
    fn transmute_to_u64(self) -> u64 {
        unsafe {
            std::mem::transmute(self as f64)
        }
    }
}
impl ToU64 for f64 {
    fn transmute_to_u64(self) -> u64 {
        unsafe {
            std::mem::transmute(self)
        }
    }
}