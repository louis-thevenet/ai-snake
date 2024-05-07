use crate::{ai_snake::simulation::Configuration, snake_core::universe::Food};
use bevy::prelude::*;

#[derive(Component)]
pub struct BodySpriteId {
    snake_id: usize,
    body_id: usize,
}

#[derive(Component)]
pub struct FoodSpriteId {
    food: Food,
}

fn create_sprite(color: Color, config: &Configuration, x: f32, y: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(
                config.grid_config.cell_size,
                config.grid_config.cell_size,
            )),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            x * config.grid_config.cell_size
                - config.grid_config.cell_size * config.grid_config.width as f32 / 2.0,
            y * config.grid_config.cell_size
                - config.grid_config.cell_size * config.grid_config.height as f32 / 2.0,
            0.,
        )),
        ..default()
    }
}
pub fn update_sprites(
    mut query_body_sprites: Query<(Entity, &BodySpriteId, &mut Transform)>,
    food_sprites: Query<(Entity, &FoodSpriteId)>,
    config: Option<Res<Configuration>>,
    mut commands: Commands,
) {
    if let Some(config) = config {
        for model_index in 0..config.simulation.population.len() {
            let universe = &config.simulation.population[model_index].universe;

            // update snakes
            for (entity, sprite_id, mut transform) in query_body_sprites.iter_mut() {
                match universe.get_snake(sprite_id.snake_id) {
                    Some(snake) => {
                        let (new_pos_x, new_pos_y) = snake.positions[sprite_id.body_id];
                        transform.translation = Vec3::new(
                            new_pos_x as f32 * config.grid_config.cell_size
                                - config.grid_config.cell_size * universe.width as f32 / 2.0,
                            new_pos_y as f32 * config.grid_config.cell_size
                                - config.grid_config.cell_size * universe.height as f32 / 2.0,
                            0.,
                        );
                    }
                    None => {
                        commands.entity(entity).despawn();
                    }
                }
            }
            for (i, snake) in universe.snakes.iter().enumerate() {
                if query_body_sprites
                    .iter()
                    .map(|(_, id, _)| Some(id.snake_id == i))
                    .len()
                    < snake.positions.len()
                {
                    if let Some(pos) = snake.positions.last() {
                        let new_sprite =
                            create_sprite(Color::WHITE, &config, pos.0 as f32, pos.1 as f32);

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
                    let new_sprite =
                        create_sprite(Color::GREEN, &config, pos.0 as f32, pos.1 as f32);
                    commands.spawn((
                        new_sprite,
                        FoodSpriteId {
                            food: Food(pos.0, pos.1),
                        },
                    ));
                }
            }
        }
    }
}
