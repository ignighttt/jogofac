use std::fmt;

use bevy::prelude::*;
use bevy::{
  app::{Plugin, Startup},
  ecs::system::Commands,
  winit::WinitSettings,
};

use self::components::Player;
use self::sprite::GameSprites;

mod sprite;
mod components;

fn prepare(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut game_sprites: ResMut<GameSprites>,
) {
  // Spawna o player
  commands.spawn(Player::new());

  // Spawna a camera
  commands.spawn(Camera2dBundle::default());

  // Faz o load dos sprites
  game_sprites.idle = asset_server.load("sprites/sprite_0.png");
  game_sprites.prepare = asset_server.load("sprites/sprite_1.png");
  game_sprites.impact = asset_server.load("sprites/sprite_2.png");

  // Spawna o sprite
  commands.spawn(SpriteBundle {
    texture: game_sprites.idle.clone(),
    transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
    ..default()
  });
  commands
  .spawn(NodeBundle {
    style: Style {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      ..default()

    },
    ..default()
  })
  .with_children(|parent| {
    parent
    .spawn(ButtonBundle {
      style: Style {
        width: Val::Px(450.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      border_color: BorderColor(Color::BLACK),
      background_color: NORMAL_BUTTON.into(),
      ..default()
    })
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Button",
        TextStyle {
          font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
          font_size: 40.0,
          color: Color::rgb(0.9, 0.9, 0.9),
        },
      ));
    });
  });
}

fn handle_input(
  buttons: Res<ButtonInput<MouseButton>>,
  mut players: Query<&mut Player>,
) {
  for mut player in &mut players {
    if buttons.just_pressed(MouseButton::Left) {
      player.coins += player.multiplier as u64 * 1;
      player.clicks += 1;
      println!("coins: {} - clicks: {}", player.coins, player.clicks);
    }
  }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


fn button_system(
  mut interaction_query: Query<
  (
    &Interaction,
    &mut BackgroundColor,
    &mut BorderColor,
    &Children,
  ),
  (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
  mut player_query: Query<&mut Player>,
) {
  for (interaction, mut color, mut border_color, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Pressed => {
        *color = PRESSED_BUTTON.into();
        border_color.0 = Color::RED;
        for mut player in &mut player_query {
          if player.coins >= player.multiplier_cost {
            player.coins -= player.multiplier_cost;
            player.multiplier_cost += (player.multiplier_cost as f64 * 0.1) as u64;
            player.multiplier += 1;
          }
        }
        for player in &mut player_query {
          text.sections[0].value = format!("Multiplier x{} - Next: {}", player.multiplier, player.multiplier_cost);
        }
      }
      Interaction::Hovered => {
        for player in &mut player_query {
          text.sections[0].value = format!("Multiplier x{} - Next: {}", player.multiplier, player.multiplier_cost);
        }
        *color = HOVERED_BUTTON.into();
        border_color.0 = Color::WHITE;
      }
      Interaction::None => {
        for player in &mut player_query {
          text.sections[0].value = format!("Multiplier x{} - Next: {}", player.multiplier, player.multiplier_cost);
        }
        *color = NORMAL_BUTTON.into();
        border_color.0 = Color::BLACK;
      }
    }
  }
}

pub struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.insert_resource(WinitSettings::desktop_app());
    app.insert_resource(GameSprites::default());
    app.add_systems(Update, button_system);
    app.add_systems(Startup, prepare);
    app.add_systems(Update, handle_input);
  }
}

