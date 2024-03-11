use bevy::{app::{Plugin, Startup}, ecs::system::Commands};
use bevy::prelude::*;

use self::components::Player;

mod components;

fn prepare(mut commands: Commands, asset_server: Res<AssetServer>) {
  // Spawna o player
  commands.spawn((Player::new()));

  // Spawna a camera
  commands.spawn(Camera2dBundle::default());

  // Spawna o sprite
  commands.spawn(SpriteBundle {
    texture: asset_server.load("cookie_texture.png"),
    transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
    ..default()
  });
}

fn handle_input(
  buttons: Res<ButtonInput<MouseButton>>,
  mut players: Query<&mut Player>,
) {
  for mut player in &mut players {
    if buttons.just_pressed(MouseButton::Left) {
      player.coins += 1;
      player.clicks += 1;

      println!("coins: {} - clicks: {}", player.coins, player.clicks);
    }
  }
}

pub struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_systems(Startup, prepare);
    app.add_systems(Update, handle_input);
  }
}