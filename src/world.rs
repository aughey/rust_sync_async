use std::collections::HashMap;
use anyhow::Result;

use crate::{EntityCreateData, EntityData, Interface, IntermediateEntity};

pub struct World {
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

impl Interface for World {
    fn create_entity_request(&mut self, data: EntityCreateData) -> Result<IntermediateEntity> {
        let id = self.next_id;
        self.next_id += 1;
        self.live_entities.insert(id, data);
        Ok(IntermediateEntity { id })
    }

    fn destroy_entity_request(&mut self, id: u32) -> bool {
        self.live_entities.remove(&id).is_some()
    }

    fn get_entity(&self, id: u32) -> Option<&EntityData> {
        self.live_entities.get(&id)
    }
}