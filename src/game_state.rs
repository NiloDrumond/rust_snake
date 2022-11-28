use bevy::prelude::*;

use crate::{
    food::Food,
    player::{snake_movement, spawn_snake, SnakeHead, SnakeSegment, SnakeSegments},
};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_system(game_over.after(snake_movement));
    }
}

pub struct GameOverEvent;

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    head: Query<Entity, With<SnakeHead>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        let head_entity = head.single();
        commands.entity(head_entity).despawn();
        spawn_snake(commands, segments_res);
    }
}
