mod world;
mod behaviour;

use anyhow::Result;
use glam::Vec3;

type EntityCreateData = EntityData;

struct EntityData {
    name: String,
    position: Vec3,
    orientation: Vec3,
}

struct IntermediateEntity {
    id: u32,
}


trait Interface {
    fn create_entity_request(&mut self, data: EntityCreateData) -> Result<IntermediateEntity>;
    fn destroy_entity_request(&mut self, id: u32) -> bool;
    fn get_entity(&self, id: u32) -> Option<&EntityData>;
}

trait Actor {
    fn on_entity_created(&mut self, id: u32);
    fn on_entity_destroyed(&mut self, id: u32);
}