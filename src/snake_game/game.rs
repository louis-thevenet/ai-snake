use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::{
    camera::{camera_controls, spawn_camera},
    game_rendering::sprites::RenderSpritePlugin,
};
use crate::snake_core::{snake::Snake, universe::Universe};
#[derive(Resource)]
pub struct Configuration {
    pub width: u64,
    pub height: u64,
    pub cell_size: f32,
}

pub struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderSpritePlugin);
        app.add_systems(Startup, (setup_game, spawn_camera).chain())
            .add_systems(Update, (camera_controls, display_grid))
            .add_systems(
                FixedUpdate,
                (snake_controls).run_if(on_timer(Duration::from_millis(125))),
            );
    }
}

fn setup_game(mut commands: Commands) {
    let width = 32;
    let height = 32;

    let config = Configuration {
        width,
        height,
        cell_size: 16.0,
    };

    let snake = Snake::new(width, height, 0);
    let mut universe = Universe::new(width, height, vec![snake]);
    universe.spawn_food();
    commands.insert_resource(universe);
    commands.insert_resource(config);
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
    if let Some(snake) = universe.get_snake(0) {
        let current_direction = snake.direction.clone();
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
}
