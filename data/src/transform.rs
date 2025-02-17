#[derive(Default)]
pub struct Position {
    pub x: i32,
    pub z: i32,
    pub y: i16
}

impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Self {
            x, y, z
        }
    }
}

/// This struct holds an angle as a 1/256 of a full turn
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct Angle(pub u8);

impl Angle {
    pub fn new(angle: u8) -> Self {
        Self(angle)
    }
}