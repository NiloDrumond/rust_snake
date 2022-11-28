use crate::{core::Direction, player::snake_movement};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput {
            direction: Direction::Right,
        })
        .add_system(snake_movement_input.before(snake_movement));
    }
}

#[derive(Resource)]
pub struct PlayerInput {
    pub direction: Direction,
}

fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    player_input.direction = if keyboard_input.pressed(KeyCode::Left) {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::Up) {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::Right) {
        Direction::Right
    } else if keyboard_input.pressed(KeyCode::Down) {
        Direction::Down
    } else {
        player_input.direction
    };
}
