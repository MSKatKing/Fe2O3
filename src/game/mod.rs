use std::ops::Add;
use crate::networking::packet::play::SetPlayerPosition;

pub mod entities;
pub mod world;
pub mod registry;

pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z,
            yaw: 0.0,
            pitch: 0.0
        }
    }

    pub fn relative(&self, other: &Location) -> Location {
        Location {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
            yaw: other.yaw - self.yaw,
            pitch: other.pitch - self.pitch
        }
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
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            yaw: self.yaw + rhs.yaw,
            pitch: self.pitch + rhs.pitch,
        }
    }
}