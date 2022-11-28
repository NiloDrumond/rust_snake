use bevy::prelude::*;

use eat::EatPlugin;
use food::FoodPlugin;
use game_state::GameStatePlugin;
use input::InputPlugin;
use player::PlayerPlugin;
use render::RenderPlugin;

pub mod config;
pub mod core;
pub mod eat;
pub mod food;
pub mod game_state;
pub mod input;
pub mod player;
pub mod render;
pub mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rusty Snake".to_string(),
                width: 600.0,
                height: 600.0,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RenderPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(EatPlugin)
        .add_plugin(FoodPlugin)
        .run();
}
