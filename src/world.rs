use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Actor, EntityCreateData, EntityData, Interface};

struct IntermediateEntity {
    data: EntityCreateData,
    id: u32,
    wait_time: u32,
}

pub struct World {
    intermediate_entities: Vec<IntermediateEntity>,
    live_entities: HashMap<u32, EntityData>,
    actors: Rc<RefCell<Vec<Box<dyn Actor>>>>,
    next_id: u32,
}
impl World {
    pub fn new() -> Self {
        World {
            intermediate_entities: Vec::new(),
            live_entities: HashMap::new(),
            // One way to solve this, but a bit messy to have to clone the actors every time
            // and borrow.  Somewhat sketchy.
            actors: Rc::new(RefCell::new(Vec::new())),
            next_id: 0,
        }
    }

    pub fn add_actor<A>(&mut self, actor: A)
    where
        A: Actor + 'static,
    {
        self.actors.borrow_mut().push(Box::new(actor));
    }
    pub fn frame(&mut self) {
        // go through intermediate entities, decrement their wait time, and if zero, promote them to live entities
        let actors = self.actors.clone();
        let mut actors = actors.borrow_mut();
        for a in actors.iter_mut() {
            a.on_pre_frame(self);
        }
        let new_indices = self
            .intermediate_entities
            .iter_mut()
            .enumerate()
            .filter_map(|(index, entity)| {
                entity.wait_time -= 1;
                if entity.wait_time == 0 {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for index in new_indices {
            let entity = self.intermediate_entities.remove(index);
            self.live_entities.insert(entity.id, entity.data);
        }
        for a in actors.iter_mut() {
            a.on_post_frame(self);
        }
    }
}

impl Interface for World {
    fn create_entity_request(&mut self, data: EntityCreateData) -> Result<u32> {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.intermediate_entities.push(IntermediateEntity {
            data,
            id,
            wait_time: 60,
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
