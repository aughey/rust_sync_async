use crate::{Actor, EntityCreateData, Interface};
use anyhow::Result;
use glam::Vec3;

pub struct SimpleBehavior {
    created: bool,
}
impl SimpleBehavior {
    pub fn new() -> Self {
        SimpleBehavior { created: false }
    }
}
impl Actor for SimpleBehavior {
    //impl Actor for SimpleBehavior {
    fn on_pre_frame(&mut self, iface: &mut dyn Interface) -> Result<()> {
        println!("SimpleBehavior on_frame");
        if !self.created {
            self.created = true;
            iface.create_entity_request(EntityCreateData {
                name: "SimpleBehavior".to_string(),
                position: Vec3::new(0.0, 0.0, 0.0),
                orientation: Vec3::new(0.0, 0.0, 0.0),
            })?;
        }
        Ok(())
    }
    fn on_post_frame(&mut self, _iface: &mut dyn Interface) -> Result<()> {
        Ok(())
    }
    fn on_entity_created(&mut self, id: u32, _iface: &mut dyn Interface) -> Result<()> {
        println!("Entity created: {}", id);
        Ok(())
    }
    fn on_entity_destroyed(&mut self, id: u32, _iface: &mut dyn Interface) -> Result<()> {
        println!("Entity destroyed: {}", id);
        Ok(())
    }
}
