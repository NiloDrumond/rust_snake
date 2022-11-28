use bevy::prelude::Color;

pub const ARENA_WIDTH: u32 = 20;
pub const ARENA_HEIGHT: u32 = 20;

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const FOOD_COLOR: Color = Color::rgb(0.1, 0.8, 0.2);

pub const MOVEMENT_STEP: f64 = 0.100;
pub const FOOD_SPAWN_STEP: f64 = 1.000;
