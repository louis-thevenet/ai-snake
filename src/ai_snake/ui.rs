use std::any::Any;

use bevy::ecs::system::RunSystemOnce;
use bevy::window::PrimaryWindow;
use bevy::{ecs::system, prelude::*};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};

use super::simulation::Configuration;
use super::simulation_rendering::camera::camera_update;

#[derive(Default, States, Debug, Hash, Eq, Clone, Copy, PartialEq)]

pub enum SimulationState {
    #[default]
    Stopped,
    StartUp,
    Evolving,
    Running,
    Paused,
}
#[derive(Default, Resource)]
pub struct AppConfig {
    pub grid_size: u64,
    pub population_size: u64,
    pub allowed_moves: u32,
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppConfig>()
            .init_state::<SimulationState>()
            .add_plugins(EguiPlugin)
            .add_systems(Startup, configure_app_state)
            .add_systems(Update, (build_ui, ui_controls));
    }
}
fn configure_app_state(mut app_state: ResMut<AppConfig>) {
    app_state.grid_size = 32;
    app_state.population_size = 1;
    app_state.allowed_moves = 30;
}

fn build_ui(
    mut contexts: EguiContexts,
    app_config: ResMut<AppConfig>,
    sim_state: ResMut<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    egui::SidePanel::left("Menu")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Controls");
            ui.label("Camera controls : WASD");
            ui.label("Zoom : Q, E");

            ui.heading("Configuration");
            match sim_state.get() {
                SimulationState::StartUp => {}
                SimulationState::Evolving => {}
                SimulationState::Running => {
                    running_ui(ui, &mut next_state);
                }
                SimulationState::Paused => {
                    paused_ui(ui, &mut next_state);
                }
                SimulationState::Stopped => {
                    stopped_ui(ui, app_config, &mut next_state);
                }
            }

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
}

fn stopped_ui(
    ui: &mut Ui,
    mut app_config: ResMut<AppConfig>,

    next_state: &mut NextState<SimulationState>,
) {
    ui.label("Simulation is not running");
    ui.add(egui::Slider::new(&mut app_config.grid_size, 0..=128).text("grid size"));
    ui.add(egui::Slider::new(&mut app_config.population_size, 0..=1000).text("population size"));
    ui.add(
        egui::Slider::new(&mut app_config.allowed_moves, 0..=256)
            .text("allowed moves before evolution"),
    );

    if ui.button("Start").clicked() {
        next_state.set(SimulationState::StartUp)
    }
}

fn running_ui(ui: &mut Ui, next_state: &mut NextState<SimulationState>) {
    ui.label("Simulation is running");
    if ui.button("Pause").clicked() {
        next_state.set(SimulationState::Paused)
    }
}
fn paused_ui(ui: &mut Ui, next_state: &mut NextState<SimulationState>) {
    ui.label("Simulation is paused");
    if ui.button("Resume").clicked() {
        next_state.set(SimulationState::Running)
    }
}
fn ui_controls(
    state: ResMut<AppConfig>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Space) {}
}
