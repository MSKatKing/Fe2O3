use std::fmt::{Display, Formatter};
use std::ops::Add;
use mem_helper::{ToF64, ToU64};
use crate::networking::packet::play::SetPlayerPosition;

pub mod entities;
pub mod world;
pub mod registry;

#[derive(Hash, PartialEq, Eq)]
pub struct Location {
    x: u64,
    y: u64,
    z: u64,
    yaw: u32,
    pitch: u32,
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x.transmute_to_u64(),
            y: y.transmute_to_u64(),
            z: z.transmute_to_u64(),
            yaw: 0,
            pitch: 0
        }
    }

    pub fn relative(&self, other: &Location) -> Location {
        Location {
            x: (other.x() - self.x()).transmute_to_u64(),
            y: (other.y() - self.y()).transmute_to_u64(),
            z: (other.z() - self.z()).transmute_to_u64(),
            yaw: other.yaw - self.yaw,
            pitch: other.pitch - self.pitch
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn x(&self) -> f64 {
        self.x.transmute_to_f64()
    }

    pub fn y(&self) -> f64 {
        self.y.transmute_to_f64()
    }

    pub fn z(&self) -> f64 {
        self.z.transmute_to_f64()
    }

    pub fn yaw(&self) -> f32 {
        self.yaw as f32
    }

    pub fn pitch(&self) -> f32 {
        self.pitch as f32
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw as u32;
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch as u32;
    }
}

impl From<SetPlayerPosition> for Location {
    fn from(value: SetPlayerPosition) -> Self {
        Location::new(value.x, value.y, value.z)
    }
}

impl Add<&Location> for &Location {
    type Output = Location;

    fn add(self, rhs: &Location) -> Self::Output {
        Location {
            x: (self.x() + rhs.x()).transmute_to_u64(),
            y: (self.y() + rhs.y()).transmute_to_u64(),
            z: (self.z() + rhs.z()).transmute_to_u64(),
            yaw: (self.yaw() + rhs.yaw()) as u32,
            pitch: (self.pitch() + rhs.pitch()) as u32,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}