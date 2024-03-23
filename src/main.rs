use bevy::{
  app::App,
  DefaultPlugins,
};
use game::ClickerGamePlugin;

mod game;

fn main() {
  App::new()
  .add_plugins((DefaultPlugins, ClickerGamePlugin))
  .run();
}
