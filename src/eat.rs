use bevy::prelude::*;

use crate::{
    core::Position,
    food::Food,
    player::{spawn_segment, LastTailPosition, SnakeHead, SnakeSegments},
};

pub struct EatPlugin;

impl Plugin for EatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GrowthEvent>();
    }
}

pub struct GrowthEvent;

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    let head_pos = head_positions.single();
    for (ent, food_pos) in food_positions.iter() {
        if food_pos == head_pos {
            commands.entity(ent).despawn();
            growth_writer.send(GrowthEvent);
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()))
    }
}
