use crate::{Actor, Interface};

pub struct SimpleBehavior {
    created: bool,
}
impl SimpleBehavior {
    pub fn new() -> Self {
        SimpleBehavior { created: false }
    }
}
impl Actor for SimpleBehavior {
    fn on_pre_frame(&mut self, iface: &mut dyn Interface) {
        println!("SimpleBehavior on_frame");
        if !self.created {}
    }
    fn on_entity_created(&mut self, id: u32, iface: &mut dyn Interface) {
        println!("Entity created: {}", id);
    }
    fn on_entity_destroyed(&mut self, id: u32, iface: &mut dyn Interface) {
        println!("Entity destroyed: {}", id);
    }
}
