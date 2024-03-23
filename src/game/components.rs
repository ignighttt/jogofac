use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player {
  pub coins: u64,
  pub clicks: u64,
  pub multiplier: u8,
  pub multiplier_cost: u64,
}

impl Player {
  pub fn new() -> Self {
    Self {
      coins: 0,
      clicks: 0,
      multiplier: 1,
      multiplier_cost: 10,
    }
  }
}
