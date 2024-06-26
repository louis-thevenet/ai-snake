use super::game::Configuration;
use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands, config: Res<Configuration>) {
    let cell_size = config.cell_size;

    let cam = Camera2dBundle {
        transform: Transform::from_xyz(cell_size / 2.0, cell_size / 2.0, 1.0),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical((config.height + 1) as f32 * cell_size),
            ..Default::default()
        },
        ..default()
    };

    commands.spawn((cam, MainCamera));
}

pub fn camera_controls(
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
