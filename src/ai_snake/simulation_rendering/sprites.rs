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
        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
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
        let line_length = (1.0 + config.simulation.population.len() as f64).sqrt() as usize;

        // update snakes
        for (entity, sprite_id, mut transform) in query_body_sprites.iter_mut() {
            let mut updated = false;
            for model_index in 0..config.simulation.population.len() {
                let universe = &config.simulation.population[model_index].universe;

                if sprite_id.snake_id == config.simulation.population[model_index].id {
                    updated = true;
                    if let Some(snake) = config.simulation.population[model_index]
                        .universe
                        .get_snake(sprite_id.snake_id)
                    {
                        let (new_pos_x, new_pos_y) = snake.positions[sprite_id.body_id];
                        transform.translation = Vec3::new(
                            ((model_index * config.simulation.population.len()) as f32
                                + new_pos_x as f32)
                                * config.grid_config.cell_size
                                - config.grid_config.cell_size * universe.width as f32 / 2.0,
                            ((model_index * config.simulation.population.len()) as f32
                                + new_pos_y as f32)
                                * config.grid_config.cell_size
                                - config.grid_config.cell_size * universe.height as f32 / 2.0,
                            0.,
                        );
                    }
                }
            }
            if !updated {
                commands.entity(entity).despawn();
            }
        }
        for model_index in 0..config.simulation.population.len() {
            let universe = &config.simulation.population[model_index].universe;

            for (i, snake) in config.simulation.population[model_index]
                .universe
                .snakes
                .iter()
                .enumerate()
            {
                if query_body_sprites
                    .iter()
                    .map(|(_, id, _)| Some(id.snake_id == i))
                    .len()
                    < snake.positions.len()
                {
                    if let Some(pos) = snake.positions.last() {
                        let x_grid_offset = ((model_index % line_length)
                            * config.grid_config.width as usize)
                            as f32
                            * config.grid_config.cell_size;
                        let y_grid_offset = ((model_index / line_length)
                            * config.grid_config.width as usize)
                            as f32
                            * config.grid_config.cell_size;

                        let x = x_grid_offset + (pos.0 as f32) * config.grid_config.cell_size
                            - config.grid_config.cell_size * universe.width as f32 / 2.0;
                        let y = y_grid_offset + (pos.1 as f32) * config.grid_config.cell_size
                            - config.grid_config.cell_size * universe.height as f32 / 2.0;
                        let new_sprite = create_sprite(Color::WHITE, &config, x, y);

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
                    let x = ((model_index * config.simulation.population.len()) as f32
                        + pos.0 as f32)
                        * config.grid_config.cell_size
                        - config.grid_config.cell_size * universe.width as f32 / 2.0;
                    let y = ((model_index * config.simulation.population.len()) as f32
                        + pos.1 as f32)
                        * config.grid_config.cell_size
                        - config.grid_config.cell_size * universe.height as f32 / 2.0;

                    let new_sprite = create_sprite(Color::GREEN, &config, x, y);
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
