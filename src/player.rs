use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    config::*,
    core::{Direction, Position, Size},
    eat::{snake_eating, snake_growth},
    game_state::GameOverEvent,
    input::PlayerInput,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastTailPosition::default())
            .insert_resource(SnakeSegments::default())
            .add_startup_system(spawn_snake)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(MOVEMENT_STEP))
                    .with_system(snake_movement)
                    .with_system(snake_eating.after(snake_movement))
                    .with_system(snake_growth.after(snake_eating)),
            );
    }
}

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource, Default)]
pub struct LastTailPosition(pub Option<Position>);

pub fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Right,
            })
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 2, y: 3 }),
    ])
}

pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    player_input: Res<PlayerInput>,
    mut heads: Query<(Entity, &mut SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    let (head_entity, mut head) = heads.single_mut();

    let segment_positions = segments
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();
    let mut head_pos = positions.get_mut(head_entity).unwrap();

    if head.direction.opposite() != player_input.direction {
        head.direction = player_input.direction;
    }

    match &head.direction {
        Direction::Left => head_pos.x -= 1,
        Direction::Up => head_pos.y += 1,
        Direction::Right => head_pos.x += 1,
        Direction::Down => head_pos.y -= 1,
    }

    if head_pos.x < 0
        || head_pos.y < 0
        || head_pos.x as u32 >= ARENA_WIDTH
        || head_pos.y as u32 >= ARENA_HEIGHT
    {
        game_over_writer.send(GameOverEvent)
    }

    if segment_positions.contains(&head_pos) {
        game_over_writer.send(GameOverEvent)
    }

    segment_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });

    *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()))
}
