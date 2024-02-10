use std::collections::HashMap;

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

struct World {
    intermediate_entities: Vec<IntermediateEntity>,
    live_entities: HashMap<u32, EntityData>,
    next_id: u32,
}
impl World {
    fn new() -> Self {
        World {
            intermediate_entities: Vec::new(),
            live_entities: HashMap::new(),
            next_id: 0,
        }
    }
}

trait Interface {
    fn create_entity(&mut self, data: EntityCreateData) -> Result<IntermediateEntity>;
    fn destroy_entity(&mut self, id: u32) -> bool;
    fn get_entity(&self, id: u32) -> Option<&EntityData>;
}

impl Interface for World {
    fn create_entity(&mut self, data: EntityCreateData) -> Result<IntermediateEntity> {
        let id = self.next_id;
        self.next_id += 1;
        self.live_entities.insert(id, data);
        Ok(IntermediateEntity { id })
    }

    fn destroy_entity(&mut self, id: u32) -> bool {
        self.live_entities.remove(&id).is_some()
    }

    fn get_entity(&self, id: u32) -> Option<&EntityData> {
        self.live_entities.get(&id)
    }
}
