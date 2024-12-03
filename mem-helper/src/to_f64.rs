pub trait ToF64 {
    fn transmute_to_f64(self) -> f64;
}

impl ToF64 for u8 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self as u64)
        }
    }
}
impl ToF64 for u16 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self as u64)
        }
    }
}
impl ToF64 for u32 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self as u64)
        }
    }
}
impl ToF64 for u64 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self as u64)
        }
    }
}

impl ToF64 for i8 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self as i64)
        }
    }
}
impl ToF64 for i16 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self as i64)
        }
    }
}
impl ToF64 for i32 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self as i64)
        }
    }
}
impl ToF64 for i64 {
    fn transmute_to_f64(self) -> f64 {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl ToF64 for f32 {
    fn transmute_to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for f64 {
    fn transmute_to_f64(self) -> f64 {
        self
    }
}