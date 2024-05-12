use crate::ai_snake::simulation::Configuration;
use bevy::{
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::Extent3d},
};

#[derive(Resource)]
pub struct MainSpriteId(AssetId<Image>);

fn get_image_dimensions(config: &Res<Configuration>) -> (u32, u32) {
    let row_length = (1.0 + config.simulation.population.len() as f64).sqrt() as u32;
    let column_length = row_length
        + (config.simulation.population.len() as u32 - row_length * row_length) % row_length;

    let cell_size = config.grid_config.cell_size as u32;

    let width = config.grid_config.width as u32 * cell_size * row_length;
    let height = config.grid_config.height as u32 * cell_size * column_length;
    (width, height)
}
pub fn setup_sprites(
    mut commands: Commands,
    config: Option<Res<Configuration>>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(config) = config {
        let (width, height) = get_image_dimensions(&config);
        let cell_size = config.grid_config.cell_size;

        let pixels = vec![0; (width * height * 4) as usize];
        println!("width: {}, height: {}", width, height);
        let img = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            pixels,
            bevy::render::render_resource::TextureFormat::Rgba8Unorm,
            RenderAssetUsages::default(),
        );

        let image_handle = images.add(img);
        commands.insert_resource(MainSpriteId(image_handle.id()));

        let bot_left_x = 0.
            - (cell_size
                * (config.grid_config.width as f32 + 1.)
                * f32::sqrt(config.simulation.population.len() as f32))
                / 2.;
        let bot_left_y = 0.
            - 2. * (cell_size
                * (config.grid_config.height as f32)
                * f32::sqrt(config.simulation.population.len() as f32));
        println!("bot left x: {}, bot left y: {}", bot_left_x, bot_left_y);
        commands.spawn((SpriteBundle {
            texture: image_handle,
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::Center,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },));
    }
}

pub fn update_sprites(
    config: Option<Res<Configuration>>,

    sprite_id: Option<Res<MainSpriteId>>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(config) = config {
        if let Some(sprite_id) = sprite_id {
            let (width, height) = get_image_dimensions(&config);
            let img = images.get_mut(sprite_id.0).unwrap();
            let population = &config.simulation.population;
            let line_length = (1.0 + population.len() as f64).sqrt() as usize;
            let cell_size = config.grid_config.cell_size as usize;

            img.data = vec![0; (width * height * 4) as usize];
            (0..population.len()).for_each(|index| {
                let x_offset =
                    ((index % line_length) * config.grid_config.width as usize) * cell_size;
                let y_offset =
                    ((index / line_length) * config.grid_config.height as usize) * cell_size;
                for snake in population[index].universe.snakes.iter() {
                    for body in snake.positions.iter() {
                        for k in 0..cell_size {
                            for l in 0..cell_size {
                                let x = (body.0 as usize * cell_size) + x_offset + k;
                                let y = ((config.grid_config.height - 1 - body.1) as usize
                                    * cell_size)
                                    + y_offset
                                    + l;

                                let pixel_index = 4 * (y as u32 * width + x as u32) as usize;
                                img.data[pixel_index] = 255;
                                img.data[pixel_index + 3] = 255;
                            }
                        }
                    }
                }

                for food in population[index].universe.food.iter() {
                    for k in 0..config.grid_config.cell_size as usize {
                        for l in 0..config.grid_config.cell_size as usize {
                            let x = (food.0 as usize * cell_size) + x_offset + k;
                            let y = ((config.grid_config.height - 1 - food.1) as usize * cell_size)
                                + y_offset
                                + l;

                            let pixel_index = 4 * (y as u32 * width + x as u32) as usize;

                            img.data[pixel_index + 1] = 255;
                            img.data[pixel_index + 3] = 255;
                        }
                    }
                }
            });
        }
    }
}
