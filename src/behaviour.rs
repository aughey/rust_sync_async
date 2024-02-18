use crate::{Actor, EntityCreateData, Interface};
use anyhow::Result;
use glam::Vec3;
use std::time::{Duration, Instant};

pub struct SimpleBehavior {}
impl SimpleBehavior {
    pub fn new() -> Self {
        SimpleBehavior {}
    }
}
impl Actor for SimpleBehavior {
    //impl Actor for SimpleBehavior {
    fn on_pre_frame(&mut self, _iface: &mut dyn Interface) -> Result<()> {
        //println!("SimpleBehavior on_frame");
        Ok(())
    }
    fn on_post_frame(&mut self, _iface: &mut dyn Interface) -> Result<()> {
        Ok(())
    }
    fn on_entity_created(&mut self, id: u32, _iface: & dyn Interface) -> Result<()> {
        println!("SimpleBehavior: Entity created: {}", id);
        Ok(())
    }
    fn on_entity_destroyed(&mut self, id: u32) -> Result<()> {
        println!("SimpleBehavior: Entity destroyed: {}", id);
        Ok(())
    }
}

enum ComplexState {
    /// Initialization
    SetupTimer,
    /// Timer to wait before entity creation
    WaitForCreate(Instant),
    /// Entity create request, waiting for it to show up
    CreatePending(Instant),
    /// Entity created, waiting before we destroy it
    WaitForDestroy(u32,Instant),
}

pub struct MoreComplexBehavior {
    state: ComplexState
}
impl MoreComplexBehavior {
    pub fn new() -> Self {
        MoreComplexBehavior { state: ComplexState::SetupTimer }
    }
}
impl Actor for MoreComplexBehavior {
    fn on_pre_frame(&mut self, iface: &mut dyn Interface) -> Result<()> {
        match self.state {
            ComplexState::SetupTimer => {
                println!("setting up timer");
                self.state = ComplexState::WaitForCreate(Instant::now());
            }
            ComplexState::WaitForCreate(time) => {
                if time.elapsed() > Duration::from_secs(2) {
                    iface.create_entity_request(EntityCreateData {
                        name: "SimpleBehavior".to_string(),
                        position: Vec3::new(0.0, 0.0, 0.0),
                        orientation: Vec3::new(0.0, 0.0, 0.0),
                    })?;
                    self.state = ComplexState::CreatePending(Instant::now());
                  println!("create request sent, waiting for response");
                }
            }
            ComplexState::CreatePending(time) => {
                if time.elapsed() > Duration::from_secs(5) {
                    // We didn't get the entity created callback in time, so we'll just wait for the next frame
                    panic!("Didn't create entity in time");
                }
            }
            ComplexState::WaitForDestroy(id,time) => {
                if time.elapsed() > Duration::from_secs(2) {
                    // Send the destroy
                    iface.destroy_entity_request(id);
                    self.state = ComplexState::SetupTimer;
                    println!("destroy request sent, going back to top of state");
                }
            }
        }
        Ok(())
    }
    fn on_post_frame(&mut self, _iface: &mut dyn Interface) -> Result<()> {
        Ok(())
    }
    fn on_entity_created(&mut self, id: u32, _iface: & dyn Interface) -> Result<()> {
        println!("MoreComplexBehavior: Entity created: {}", id);
        self.state = ComplexState::WaitForDestroy(id,Instant::now());
        Ok(())
    }
    fn on_entity_destroyed(&mut self, id: u32) -> Result<()> {
        println!("MoreComplexBehavior: Entity destroyed: {}", id);
        Ok(())
    }
}
