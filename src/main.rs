use anyhow::Result;
use rust_sync_async::behaviour;
use rust_sync_async::world;
use rust_sync_async::Actor;

#[tokio::main]
async fn main() -> Result<()> {
    let mut world = world::World::new();

    let simple_behavior = behaviour::SimpleBehavior::new();
    let mut actors = vec![simple_behavior];

    for _ in 0..10 {
        for actor in actors.iter_mut() {
            actor.on_pre_frame(&mut world)?;
        }
        world.frame()?;
        for actor in actors.iter_mut() {
            actor.on_post_frame(&mut world)?;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    #[allow(unreachable_code)]
    Ok(())
}
