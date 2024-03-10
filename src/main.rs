use bevy::{app::{App, Startup, Update}, ecs::{bundle, component::Component, query::With, system::{Commands, Query}}, scene::ron::Number, DefaultPlugins};

fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Hp(u32);

#[derive(Component)]
struct Monster;

#[derive(Component)]
struct Name(String);

fn add_monster(mut commands: Commands){
    commands.spawn((Monster, Hp(100)));
}


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
}

fn hpmonster(mut query: Query<&mut Hp, With<Monster>>){
    for mut life in &mut query{
        life.0 = life.0 - 1;
        println!("Vida: {}", life.0);
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don’t need to change any other names
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_people, add_monster))
        .add_systems(Update, (hello_world, update_people, greet_people, hpmonster))
        .run();
}
