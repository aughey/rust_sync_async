pub mod behaviour;
pub mod world;

use anyhow::Result;
use glam::Vec3;

type EntityCreateData = EntityData;

#[derive(Clone)]
pub struct EntityData {
    pub name: String,
    pub position: Vec3,
    pub orientation: Vec3,
}

pub trait Interface {
    fn create_entity_request(&mut self, data: EntityCreateData) -> Result<u32>;
    fn destroy_entity_request(&mut self, id: u32) -> bool;
    fn get_entity(&self, id: u32) -> Option<&EntityData>;
}

pub trait Actor {
    fn on_pre_frame(&mut self, _iface: &mut dyn Interface) -> Result<()>;
    fn on_post_frame(&mut self, _iface: &mut dyn Interface) -> Result<()>;
    fn on_entity_created(&mut self, id: u32) -> Result<()>;
    fn on_entity_destroyed(&mut self, id: u32) -> Result<()>;
}

/// we implement the box directly so that dynamic dispatch can be used
impl Actor for Box<dyn Actor> {
    fn on_pre_frame(&mut self, iface: &mut dyn Interface) -> Result<()> {
        self.as_mut().on_pre_frame(iface)
    }
    fn on_post_frame(&mut self, iface: &mut dyn Interface) -> Result<()> {
        self.as_mut().on_post_frame(iface)
    }
    fn on_entity_created(&mut self, id: u32) -> Result<()> {
        self.as_mut().on_entity_created(id)
    }
    fn on_entity_destroyed(&mut self, id: u32) -> Result<()> {
        self.as_mut().on_entity_destroyed(id)
    }
}
