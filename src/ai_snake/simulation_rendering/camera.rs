use bevy::{prelude::*, render::camera::ScalingMode};

use crate::ai_snake::simulation::Configuration;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    let cam = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(1.0),
            ..Default::default()
        },
        ..default()
    };

    commands.spawn((cam, MainCamera));
}

pub fn camera_update(
    config: Option<Res<Configuration>>,
    mut query_projection_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut query_transform_camera: Query<&mut Transform, With<MainCamera>>,
) {
    if let Some(config) = config {
        query_projection_camera.single_mut().scaling_mode = ScalingMode::FixedVertical(
            (config.grid_config.height + 1) as f32
                * config.grid_config.cell_size
                * f32::sqrt(config.simulation.population.len() as f32),
        );
        query_transform_camera.single_mut().translation = Vec3::new(
            config.grid_config.cell_size / 2.0,
            config.grid_config.cell_size / 2.0,
            1.0,
        );
    }
}

pub fn camera_controls(
    keys: Res<ButtonInput<KeyCode>>,
    timer: ResMut<Time>,
    mut query_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut query_transform_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut projection = query_camera.single_mut();

    let translation_speed = 100.0 * projection.scale;
    let boost = if keys.pressed(KeyCode::ShiftLeft) {
        3.0
    } else {
        1.0
    };

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, 1.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(-1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, -1.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }

    if keys.pressed(KeyCode::KeyE)
        || keys.pressed(KeyCode::NumpadAdd)
        || keys.pressed(KeyCode::PageUp)
    {
        projection.scale /= 1.1;
    }

    if keys.pressed(KeyCode::KeyQ)
        || keys.pressed(KeyCode::NumpadSubtract)
        || keys.pressed(KeyCode::PageDown)
    {
        projection.scale *= 1.1;
    }
}
