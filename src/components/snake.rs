use super::food::Food;
use super::grid;
use super::grid::{ARENA_HEIGHT, ARENA_WIDTH};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Snake {
    direction: Direction,
}

pub struct GrowthEvent;
pub struct GameOverEvent;

#[derive(Default, Resource)]
pub struct LastTailPosition(Option<grid::Position>);

impl Default for Snake {
    fn default() -> Self {
        Snake {
            direction: Direction::Up,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
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

#[derive(Component)]
pub struct SnakeTail;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SnakeTails(Vec<Entity>);

const SNAKE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_TAIL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

fn spawn_tail(mut commands: Commands, position: grid::Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_TAIL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeTail)
        .insert(position)
        .insert(grid::Size::square(0.6))
        .id()
}

pub fn spawn_snake(mut commands: Commands, mut body: ResMut<SnakeTails>) {
    *body = SnakeTails(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(Snake { ..default() })
            .insert(grid::Position { x: 5, y: 5 })
            .insert(grid::Size::square(0.8))
            .id(),
        spawn_tail(commands, grid::Position { x: 5, y: 4 }),
    ])
}

pub fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_position: Query<&mut Snake>,
) {
    if let Some(mut snake) = snake_position.iter_mut().next() {
        let dir = if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
            Direction::Left
        } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
            Direction::Right
        } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
            Direction::Down
        } else if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
            Direction::Up
        } else {
            snake.direction
        };
        if dir != snake.direction.opposite() {
            snake.direction = dir;
        }
    }
}

pub fn snake_movement(
    body: ResMut<SnakeTails>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut tails: Query<(Entity, &Snake)>,
    mut positions: Query<&mut grid::Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((snake_entity, snake)) = tails.iter_mut().next() {
        let tail_pos: Vec<grid::Position> = body
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect();
        let mut pos = positions.get_mut(snake_entity).unwrap();
        match &snake.direction {
            Direction::Left => {
                pos.x -= 1;
            }
            Direction::Right => {
                pos.x += 1;
            }
            Direction::Up => {
                pos.y += 1;
            }
            Direction::Down => {
                pos.y -= 1;
            }
        };
        if pos.x < 0 || pos.y < 0 || pos.x as u32 >= ARENA_WIDTH || pos.y as u32 >= ARENA_HEIGHT {
            game_over_writer.send(GameOverEvent);
        }
        if tail_pos.contains(&pos) {
            game_over_writer.send(GameOverEvent);
        }
        tail_pos
            .iter()
            .zip(body.iter().skip(1))
            .for_each(|(pos, e)| {
                *positions.get_mut(*e).unwrap() = *pos;
            });
        *last_tail_position = LastTailPosition(Some(*tail_pos.last().unwrap()));
    }
}

pub fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&grid::Size, &mut Transform)>,
) {
    let window = windows.get_single().expect("Window should be received");
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

pub fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&grid::Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_single().expect("Window should be received");
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

pub fn snake_eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_pos: Query<(Entity, &grid::Position), With<Food>>,
    snake_pos: Query<&grid::Position, With<Snake>>,
) {
    for s_pos in snake_pos.iter() {
        for (ent, food_pos) in food_pos.iter() {
            if food_pos == s_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_grow(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut tails: ResMut<SnakeTails>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        tails.push(spawn_tail(commands, last_tail_position.0.unwrap()));
    }
}

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    tails: ResMut<SnakeTails>,
    food: Query<Entity, With<Food>>,
    snake: Query<Entity, With<Snake>>,
    body: Query<Entity, With<SnakeTail>>,
) {
    if reader.iter().next().is_some() {
        for ent in snake.iter().chain(food.iter()).chain(body.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, tails);
        reader.clear();
    }
}
