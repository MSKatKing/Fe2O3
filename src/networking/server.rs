use std::net::TcpListener;
use std::ops::{Deref, DerefMut};
use shipyard::Unique;

#[derive(Unique)]
pub struct NetworkingHandler {
    listener: TcpListener
}

impl NetworkingHandler {
    pub fn new(addr: impl Into<String>) -> Self {
        let listener = TcpListener::bind(addr.into())
            .expect("Couldn't bind to address");

        Self {
            listener
        }
    }
}

impl Deref for NetworkingHandler {
    type Target = TcpListener;

    fn deref(&self) -> &Self::Target {
        &self.listener
    }
}

impl DerefMut for NetworkingHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.listener
    }
}