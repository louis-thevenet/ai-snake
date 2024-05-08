use bevy::{ecs::system::Res, gizmos::gizmos::Gizmos, math::Vec2, render::color::Color};

use crate::ai_snake::simulation::Configuration;

pub fn display_grid(config: Option<Res<Configuration>>, mut gizmos: Gizmos) {
    if let Some(config) = config {
        let population = &config.simulation.population;
        let line_length = (1.0 + config.simulation.population.len() as f64).sqrt() as usize;

        for index in 0..population.len() {
            let x = ((index % line_length) * config.grid_config.width as usize) as f32
                * config.grid_config.cell_size;

            let y = (((1 + line_length) - (index / line_length))
                * config.grid_config.width as usize) as f32
                * config.grid_config.cell_size;

            for i in 1..config.grid_config.width {
                gizmos.line_2d(
                    Vec2::new(
                        x + i as f32 * config.grid_config.cell_size
                            - (config.grid_config.cell_size
                                + config.grid_config.cell_size * config.grid_config.width as f32)
                                / 2.0,
                        y + -(config.grid_config.cell_size
                            + config.grid_config.cell_size * config.grid_config.height as f32)
                            / 2.0,
                    ),
                    Vec2::new(
                        x + i as f32 * config.grid_config.cell_size
                            - (config.grid_config.cell_size
                                + config.grid_config.cell_size * config.grid_config.width as f32)
                                / 2.0,
                        y + (config.grid_config.cell_size
                            + config.grid_config.cell_size
                                * (config.grid_config.height - 3) as f32)
                            / 2.0,
                    ),
                    Color::GRAY,
                );
            }

            for i in 1..config.grid_config.height {
                gizmos.line_2d(
                    Vec2::new(
                        x + -(config.grid_config.cell_size
                            + config.grid_config.cell_size * config.grid_config.width as f32)
                            / 2.0,
                        y + i as f32 * config.grid_config.cell_size
                            - (config.grid_config.cell_size
                                + config.grid_config.cell_size * config.grid_config.height as f32)
                                / 2.0,
                    ),
                    Vec2::new(
                        x + (config.grid_config.cell_size
                            + config.grid_config.cell_size * (config.grid_config.width - 3) as f32)
                            / 2.0,
                        y + i as f32 * config.grid_config.cell_size
                            - (config.grid_config.cell_size
                                + config.grid_config.cell_size * config.grid_config.height as f32)
                                / 2.0,
                    ),
                    Color::GRAY,
                );
            }
        }
    }
}
