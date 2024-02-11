use crate::{Actor, EntityCreateData, Interface};
use anyhow::Result;
use glam::Vec3;

pub struct SimpleBehavior {}
impl SimpleBehavior {
    pub fn new() -> Self {
        SimpleBehavior {}
    }
}
impl Actor for SimpleBehavior {
    //impl Actor for SimpleBehavior {
    fn on_pre_frame(&mut self, _iface: &mut dyn Interface) -> Result<()> {
        println!("SimpleBehavior on_frame");
        Ok(())
    }
    fn on_post_frame(&mut self, _iface: &mut dyn Interface) -> Result<()> {
        Ok(())
    }
    fn on_entity_created(&mut self, id: u32) -> Result<()> {
        println!("SimpleBehavior: Entity created: {}", id);
        Ok(())
    }
    fn on_entity_destroyed(&mut self, id: u32) -> Result<()> {
        println!("SimpleBehavior: Entity destroyed: {}", id);
        Ok(())
    }
}

pub struct MoreComplexBehavior {
    created: bool,
}
impl MoreComplexBehavior {
    pub fn new() -> Self {
        MoreComplexBehavior { created: false }
    }
}
impl Actor for MoreComplexBehavior {
    fn on_pre_frame(&mut self, iface: &mut dyn Interface) -> Result<()> {
        println!("MoreComplexBehavior on_frame");
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
    fn on_entity_created(&mut self, id: u32) -> Result<()> {
        println!("MoreComplexBehavior: Entity created: {}", id);
        Ok(())
    }
    fn on_entity_destroyed(&mut self, id: u32) -> Result<()> {
        println!("MoreComplexBehavior: Entity destroyed: {}", id);
        Ok(())
    }
}
