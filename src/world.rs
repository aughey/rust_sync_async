use anyhow::Result;
use std::collections::HashMap;

use crate::{Actor, EntityCreateData, EntityData, Interface};

struct IntermediateEntity {
    data: EntityCreateData,
    id: u32,
    wait_time: u32,
}

pub struct World {
    intermediate_entities: Vec<IntermediateEntity>,
    live_entities: HashMap<u32, EntityData>,
    next_id: u32,
}
impl World {
    pub fn new() -> Self {
        World {
            intermediate_entities: Vec::new(),
            live_entities: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn frame<'a, A>(&mut self, actors: &mut [A]) -> Result<()>
    where
        A: Actor + 'static,
    {
        // go through intermediate entities, decrement their wait time, and if zero, promote them to live entities
        let mut created = Vec::new();
        self.intermediate_entities.retain_mut(|entity| {
            entity.wait_time -= 1;
            if entity.wait_time == 0 {
                let id = entity.id;
                self.live_entities.insert(id, entity.data.clone());
                created.push(id);
                false
            } else {
                true
            }
        });

        let mut errors = Vec::new();
        for id in created {
            for actor in actors.iter_mut() {
                if let Err(e) = actor.on_entity_created(id, self) {
                    errors.push(e);
                }
            }
        }

        Ok(())
    }
}

impl Interface for World {
    fn create_entity_request(&mut self, data: EntityCreateData) -> Result<u32> {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.intermediate_entities.push(IntermediateEntity {
            data,
            id,
            wait_time: 2,
        });
        Ok(id)
    }

    fn destroy_entity_request(&mut self, id: u32) -> bool {
        self.live_entities.remove(&id).is_some()
    }

    fn get_entity(&self, id: u32) -> Option<&EntityData> {
        self.live_entities.get(&id)
    }
}
