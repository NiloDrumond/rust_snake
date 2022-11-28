use rand::random;

use crate::{config::*, core::Position};

pub fn get_random_position(retries: u32, occupied_positions: Vec<&Position>) -> Option<Position> {
    for _ in 0..retries {
        let position = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        };

        if !occupied_positions.contains(&&position) {
            return Some(position);
        }
    }
    None
}
