use bevy::prelude::*;
use bevy::{
  app::{Plugin, Startup},
  ecs::system::Commands,
  winit::WinitSettings,
};

use self::components::Player;

mod components;

fn prepare(mut commands: Commands, asset_server: Res<AssetServer>) {
  // Spawna o player
  commands.spawn(Player::new());

  // Spawna a camera
  commands.spawn(Camera2dBundle::default());

  // Spawna o sprite
  commands.spawn(SpriteBundle {
    texture: asset_server.load("sprites/sprite_0.png"),
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
        width: Val::Px(150.0),
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

fn handle_input(buttons: Res<ButtonInput<MouseButton>>, mut players: Query<&mut Player>) {
  for mut player in &mut players {
    if buttons.just_pressed(MouseButton::Left) {
      player.coins += 1;
      player.clicks += 1;

      println!("coins: {} - clicks: {}", player.coins, player.clicks);
    }
  }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

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
) {
}

pub struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.insert_resource(WinitSettings::desktop_app());
    app.add_systems(Update, button_system);
    app.add_systems(Startup, prepare);
    app.add_systems(Update, handle_input);
  }
}

