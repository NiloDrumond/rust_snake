use bevy::{prelude::*, time::FixedTimestep};
use rand::random;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::rgb(0.1, 0.8, 0.2);

const MOVEMENT_STEP: f64 = 0.200;
const FOOD_SPAWN_STEP: f64 = 0.050;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

struct GameOverEvent;

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Resource, Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

#[derive(Resource, Default)]
struct LastTailPosition(Option<Position>);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct Food;

struct GrowthEvent;

#[derive(Resource)]
struct PlayerInput {
    direction: Direction,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
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

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let convert = |pos: f32, bound_window: f32, bound_game: f32| -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    };

    let window = windows.get_primary().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
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

fn snake_movement(
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

fn get_random_position(retries: u32, occupied_positions: Vec<&Position>) -> Option<Position> {
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

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
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

fn snake_eating(
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

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()))
    }
}

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

fn main() {
    App::new()
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .insert_resource(PlayerInput {
            direction: Direction::Right,
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(LastTailPosition::default())
        .insert_resource(SnakeSegments::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rusty Snake".to_string(),
                width: 500.0,
                height: 500.0,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement_input.before(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(MOVEMENT_STEP))
                .with_system(snake_movement)
                .with_system(snake_eating.after(snake_movement))
                .with_system(snake_growth.after(snake_eating)),
        )
        .add_system(game_over.after(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FOOD_SPAWN_STEP))
                .with_system(food_spawner),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .run();
}
