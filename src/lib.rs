pub mod behaviour;
pub mod world;

use anyhow::Result;
use glam::Vec3;

type EntityCreateData = EntityData;

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
    fn on_pre_frame(&mut self, iface: &mut dyn Interface) {}
    fn on_post_frame(&mut self, iface: &mut dyn Interface) {}
    fn on_entity_created(&mut self, id: u32, iface: &mut dyn Interface);
    fn on_entity_destroyed(&mut self, id: u32, iface: &mut dyn Interface);
}
