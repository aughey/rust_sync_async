use anyhow::Result;
use rust_sync_async::behaviour;
use rust_sync_async::world;

#[tokio::main]
async fn main() -> Result<()> {
    let mut world = world::World::new();
    let simple_behavior = behaviour::SimpleBehavior::new();
    world.add_actor(simple_behavior);

    for _ in 0..10 {
        world.frame();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    #[allow(unreachable_code)]
    Ok(())
}
