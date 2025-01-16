use packet::{Identifier, Position};
use packet_proc::{Deserializable, Serializable};
use text_component::Component;
use crate::game::Location;

pub mod player;

#[derive(Serializable, Deserializable)]
pub struct DeathLocation {
    pub dimension: Identifier,
    pub position: Position
}

pub struct EntityManager;

pub trait Entity: Positionable + Nameable {

}

pub trait Positionable {
    fn position(&self) -> &Location;

    fn x(&self) -> f64 {
        self.position().x()
    }
    fn y(&self) -> f64 {
        self.position().y()
    }
    fn z(&self) -> f64 {
        self.position().z()
    }

    fn yaw(&self) -> f32 {
        self.position().yaw()
    }
    fn pitch(&self) -> f32 {
        self.position().pitch()
    }

    fn move_relative(&mut self, relative: Location);
    fn move_absolute(&mut self, absolute: Location);
    fn teleport(&mut self, location: Location);
}
pub trait Disconnectable {
    fn kick(&mut self, message: impl Into<Component>);
}
pub trait Nameable {
    fn name(&self) -> &str;
    fn display_name(&self) -> &str;

    fn set_display_name(&mut self, display_name: impl Into<String>);
}