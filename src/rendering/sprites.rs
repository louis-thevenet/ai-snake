use bevy::app::{FixedPostUpdate, Plugin};

use crate::snake_core::universe::{Food, Universe};
use crate::snake_game::game::Configuration;
use bevy::prelude::*;
pub struct RenderSpritePlugin;
#[derive(Component)]
struct BodySpriteId {
    snake_id: usize,
    body_id: usize,
}

#[derive(Component)]
struct FoodSpriteId {
    food: Food,
}

impl Plugin for RenderSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, update_sprites);
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
fn update_sprites(
    mut query_body_sprites: Query<(&BodySpriteId, &mut Transform)>,
    food_sprites: Query<(Entity, &FoodSpriteId)>,
    universe: ResMut<Universe>,
    config: Res<Configuration>,
    mut commands: Commands,
) {
    // update snakes
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
            if let Some(pos) = snake.positions.last() {
                let new_sprite = create_sprite(Color::WHITE, &config, pos.0 as f32, pos.1 as f32);

                commands.spawn((
                    new_sprite,
                    BodySpriteId {
                        snake_id: snake.id,
                        body_id: snake.positions.len() - 1,
                    },
                ));
            }
        }
    }

    // update food
    for (entity, food_sprite_id) in food_sprites.into_iter() {
        if !universe.food.contains(&food_sprite_id.food) {
            commands.entity(entity).despawn();
        }
    }

    if food_sprites.iter().count() < universe.food.len() {
        if let Some(pos) = universe.food.last() {
            let new_sprite = create_sprite(Color::GREEN, &config, pos.0 as f32, pos.1 as f32);
            commands.spawn((
                new_sprite,
                FoodSpriteId {
                    food: Food(pos.0, pos.1),
                },
            ));
        }
    }
}
