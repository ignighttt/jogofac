use bevy::{
    app::{App, Startup, Update},
    ecs::{
        bundle,
        component::Component,
        query::With,
        system::{Commands, Query},
    },
    scene::ron::Number,
    winit::WinitSettings,
    DefaultPlugins,
};
use game::ClickerGamePlugin;

mod game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ClickerGamePlugin))
        .run();
}
