use std::time::Duration;

use bevy::{ecs::query, prelude::*, time::common_conditions::on_timer};

use crate::snake_core::{snake::Snake, universe::Universe};

use super::camera::{camera_controls, spawn_camera};
#[derive(Resource)]
pub struct Configuration {
    pub width: u64,
    pub height: u64,
    pub cell_size: f32,
}

#[derive(Component)]
struct BodySpriteId {
    snake_id: usize,
    body_id: usize,
}

pub struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_game, spawn_camera).chain())
            .add_systems(Update, (update_sprites, display_grid, camera_controls))
            .add_systems(
                Update,
                (snake_controls).run_if(on_timer(Duration::from_millis(150))),
            );
    }
}
fn create_sprite(color: Color, config: &Configuration, x: f32, y: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(config.cell_size, config.cell_size)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            x * config.cell_size - config.cell_size * config.width as f32 / 2.0,
            y * config.cell_size - config.cell_size * config.height as f32 / 2.0,
            0.,
        )),
        ..default()
    }
}
fn setup_game(mut commands: Commands) {
    let width = 50;
    let height = 50;

    let config = Configuration {
        width,
        height,
        cell_size: 16.0,
    };

    let snake = Snake::new(width, height, 0);

    for (i, (x, y)) in snake.positions.iter().enumerate() {
        let new_sprite = create_sprite(Color::WHITE, &config, *x as f32, *y as f32);
        commands.spawn((
            new_sprite,
            BodySpriteId {
                snake_id: snake.id,
                body_id: i,
            },
        ));
    }
    let universe = Universe::new(width, height, vec![snake]);

    commands.insert_resource(universe);
    commands.insert_resource(config);
}

fn update_sprites(
    mut query_body_sprites: Query<(&BodySpriteId, &mut Transform)>,
    universe: Res<Universe>,
    config: Res<Configuration>,
    mut commands: Commands,
) {
    for (sprite_id, mut transform) in query_body_sprites.iter_mut() {
        let snake = universe.get_snake(sprite_id.snake_id);
        let (new_pos_x, new_pos_y) = snake.positions[sprite_id.body_id];
        transform.translation = Vec3::new(
            new_pos_x as f32 * config.cell_size - config.cell_size * universe.width as f32 / 2.0,
            new_pos_y as f32 * config.cell_size - config.cell_size * universe.height as f32 / 2.0,
            0.,
        );
    }

    for (i, snake) in universe.snakes.iter().enumerate() {
        if query_body_sprites
            .iter()
            .map(|(id, _)| Some(id.snake_id == i))
            .len()
            < snake.positions.len()
        {
            println!("Missing sprite for snake {}", i);
            let pos = snake.positions.last().unwrap();
            let new_sprite = create_sprite(Color::WHITE, &config, pos.0 as f32, pos.1 as f32);
            commands.spawn((
                new_sprite,
                BodySpriteId {
                    snake_id: snake.id,
                    body_id: i,
                },
            ));
        }
    }
}
fn display_grid(config: Res<Configuration>, universe: ResMut<Universe>, mut gizmos: Gizmos) {
    let uni = universe;
    for i in 1..uni.width {
        gizmos.line_2d(
            Vec2::new(
                i as f32 * config.cell_size
                    - (config.cell_size + config.cell_size * uni.width as f32) / 2.0,
                -(config.cell_size + config.cell_size * uni.height as f32) / 2.0,
            ),
            Vec2::new(
                i as f32 * config.cell_size
                    - (config.cell_size + config.cell_size * uni.width as f32) / 2.0,
                (config.cell_size + config.cell_size * (uni.height - 2) as f32) / 2.0,
            ),
            Color::GRAY,
        );
    }

    for i in 1..uni.height {
        gizmos.line_2d(
            Vec2::new(
                -(config.cell_size + config.cell_size * uni.width as f32) / 2.0,
                i as f32 * config.cell_size
                    - (config.cell_size + config.cell_size * uni.height as f32) / 2.0,
            ),
            Vec2::new(
                (config.cell_size + config.cell_size * (uni.width - 2) as f32) / 2.0,
                i as f32 * config.cell_size
                    - (config.cell_size + config.cell_size * uni.height as f32) / 2.0,
            ),
            Color::GRAY,
        );
    }
}

fn spawn_body_sprite(
    mut commands: Commands,
    config: Res<Configuration>,
    universe: ResMut<Universe>,
    snake_id: usize,
) {
    let snake = universe.get_snake(snake_id);
    let body_sprite = snake.positions[snake.positions.len() - 1];
    let new_sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(config.cell_size, config.cell_size)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            body_sprite.0 as f32 * config.cell_size - config.cell_size * config.width as f32 / 2.0,
            body_sprite.1 as f32 * config.cell_size - config.cell_size * config.height as f32 / 2.0,
            0.,
        )),
        ..default()
    };
    commands.spawn((
        new_sprite,
        BodySpriteId {
            snake_id: snake.id,
            body_id: snake.positions.len() - 1,
        },
    ));
}

fn snake_controls(keys: Res<ButtonInput<KeyCode>>, mut universe: ResMut<Universe>) {
    let current_direction = universe.get_snake(0).direction.clone();
    universe.spawn_food();
    let direction = if keys.pressed(KeyCode::KeyW)
        && !matches!(
            current_direction,
            crate::snake_core::universe::Direction::Down
        ) {
        crate::snake_core::universe::Direction::Up
    } else if keys.pressed(KeyCode::KeyA)
        && !matches!(
            current_direction,
            crate::snake_core::universe::Direction::Right
        )
    {
        crate::snake_core::universe::Direction::Left
    } else if keys.pressed(KeyCode::KeyS)
        && !matches!(
            current_direction,
            crate::snake_core::universe::Direction::Up
        )
    {
        crate::snake_core::universe::Direction::Down
    } else if keys.pressed(KeyCode::KeyD)
        && !matches!(
            current_direction,
            crate::snake_core::universe::Direction::Left
        )
    {
        crate::snake_core::universe::Direction::Right
    } else {
        current_direction
    };

    let ate = universe.move_snake(0, direction);
    if ate {
        universe.spawn_food();
    }
}
