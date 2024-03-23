use bevy::{asset::Handle, ecs::system::Resource, render::texture::Image};

#[derive(Resource)]
pub struct GameSprites {
  pub idle: Handle<Image>,
  pub prepare: Handle<Image>,
  pub impact: Handle<Image>,
}

impl Default for GameSprites {
  fn default() -> Self {
    Self {
      idle: Default::default(),
      prepare: Default::default(),
      impact: Default::default(),
    }
  }
}
