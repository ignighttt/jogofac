use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player {
  pub coins: i64,
  pub clicks: u64,
}

impl Player {
  pub fn new() -> Self {

    Self { 
      coins: 0,
      clicks: 0,
    }
  }
}