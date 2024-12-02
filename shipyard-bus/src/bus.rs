use std::ops::Deref;
use shipyard::{Component, EntityId, Get, View};
use shipyard::error::MissingComponent;

#[derive(Component)]
struct ShipyardBus<T: Component>(Vec<T>);

impl<T: Component> Deref for ShipyardBus<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}


pub trait GetBus<T: Component>: Get {
    fn get_bus(&self, id: EntityId) -> Result<Vec<T>, MissingComponent> {
        self.get(id)?
    }
}

pub struct ViewBus<'a, T: Component>(View<'a, ShipyardBus<T>>);