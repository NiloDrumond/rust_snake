use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    config::*,
    core::{Position, Size},
    utils::get_random_position,
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FOOD_SPAWN_STEP))
                .with_system(food_spawner),
        );
    }
}

#[derive(Component)]
pub struct Food;

fn food_spawner(mut commands: Commands, q: Query<&Position>) {
    let occupied_positions = q.iter().collect();
    if let Some(position) = get_random_position(10, occupied_positions) {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Food)
            .insert(position)
            .insert(Size::square(0.8));
    }
}
