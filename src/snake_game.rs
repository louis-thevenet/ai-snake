use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::{on_timer, repeating_after_real_delay},
};

#[derive(Component)]
pub struct MainCamera;
use crate::snake::{snake::Snake, universe::Universe};
#[derive(Resource)]
pub struct Configuration {
    pub width: u64,
    pub height: u64,
    pub cell_size: f32,
}

#[derive(Component)]
struct SpriteId {
    snakeId: usize,
    bodyId: usize,
}

pub struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_game, spawn_camera).chain())
            .add_systems(
                Update,
                (update_sprites, display_grid, debug_print, camera_controls),
            )
            .add_systems(
                Update,
                (snake_controls).run_if(on_timer(Duration::from_millis(150))),
            );
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

    let snake = Snake::new(50, 50, 0);

    for (x, y) in snake.positions.iter() {
        let new_sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(config.cell_size, config.cell_size)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                *x as f32 * config.cell_size - config.cell_size * width as f32 / 2.0,
                *y as f32 * config.cell_size - config.cell_size * height as f32 / 2.0,
                0.,
            )),
            ..default()
        };
        commands.spawn((
            new_sprite,
            SpriteId {
                snakeId: snake.id,
                bodyId: 0,
            },
        ));
    }
    let universe = Universe::new(width, height, vec![snake]);

    commands.insert_resource(universe);
    commands.insert_resource(config);
}

fn debug_print(universe: Res<Universe>) {
    //println!("{:#?}", universe);
}

fn update_sprites(
    mut query: Query<(&SpriteId, &mut Transform)>,
    universe: Res<Universe>,
    config: Res<Configuration>,
) {
    for (spriteId, mut transform) in query.iter_mut() {
        let snake = universe.get_snake(spriteId.snakeId);
        let (new_pos_x, new_pos_y) = snake.positions[spriteId.bodyId];
        transform.translation = Vec3::new(
            new_pos_x as f32 * config.cell_size - config.cell_size * universe.width as f32 / 2.0,
            new_pos_y as f32 * config.cell_size - config.cell_size * universe.height as f32 / 2.0,
            0.,
        );
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
fn snake_controls(keys: Res<ButtonInput<KeyCode>>, mut universe: ResMut<Universe>) {
    let dir = universe.get_snake(0).direction.clone();

    if keys.pressed(KeyCode::KeyW) && !matches!(dir, crate::snake::universe::Direction::Down) {
        universe.move_snake(0, crate::snake::universe::Direction::Up);
    } else if keys.pressed(KeyCode::KeyA)
        && !matches!(dir, crate::snake::universe::Direction::Right)
    {
        universe.move_snake(0, crate::snake::universe::Direction::Left);
    } else if keys.pressed(KeyCode::KeyS) && !matches!(dir, crate::snake::universe::Direction::Up) {
        universe.move_snake(0, crate::snake::universe::Direction::Down);
    } else if keys.pressed(KeyCode::KeyD) && !matches!(dir, crate::snake::universe::Direction::Left)
    {
        universe.move_snake(0, crate::snake::universe::Direction::Right);
    } else {
        universe.move_snake(0, dir);
    }
}

fn spawn_camera(mut commands: Commands, config: Res<Configuration>) {
    let cell_size = config.cell_size;

    let cam = Camera2dBundle {
        transform: Transform::from_xyz(cell_size / 2.0, cell_size / 2.0, 10.0),
        projection: OrthographicProjection {
            ..Default::default()
        },
        ..default()
    };

    commands.spawn((cam, MainCamera));
}

fn camera_controls(
    keys: Res<ButtonInput<KeyCode>>,
    timer: ResMut<Time>,
    mut query_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut query_transform_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut projection = query_camera.single_mut();

    let translation_speed = 400.0 * projection.scale;
    let boost = if keys.pressed(KeyCode::ShiftLeft) {
        3.0
    } else {
        1.0
    };

    if keys.pressed(KeyCode::ArrowUp) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, 1.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(-1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, -1.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }

    if keys.pressed(KeyCode::NumpadAdd) {
        projection.scale /= 1.1;
    }

    if keys.pressed(KeyCode::NumpadSubtract) {
        projection.scale *= 1.1;
    }
}
