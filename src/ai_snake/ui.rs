use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};

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
    pub generation_number: u64,
    pub best_score: u64,
    pub average_score: u64,
    pub grid_size: u64,
    pub population_size: u64,
    pub current_moves: u64,
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
    app_state.generation_number = 0;
    app_state.best_score = 0;
    app_state.average_score = 0;
    app_state.grid_size = 32;
    app_state.population_size = 2000;
    app_state.current_moves = 0;
    app_state.allowed_moves = 300;
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
                    running_ui(ui, &mut next_state, app_config);
                }
                SimulationState::Paused => {
                    paused_ui(ui, &mut next_state, app_config);
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
    ui.add(egui::Slider::new(&mut app_config.population_size, 0..=5000).text("population size"));
    ui.add(
        egui::Slider::new(&mut app_config.allowed_moves, 0..=4096)
            .text("allowed moves before evolution"),
    );

    if ui.button("Start").clicked() {
        next_state.set(SimulationState::StartUp)
    }
}

fn running_ui(
    ui: &mut Ui,
    next_state: &mut NextState<SimulationState>,
    mut app_config: ResMut<AppConfig>,
) {
    ui.label("Simulation is running");
    ui.label("Agents number: ".to_owned() + &app_config.population_size.to_string());
    ui.label("Generation #".to_owned() + &app_config.generation_number.to_string());
    ui.label("Best Score: ".to_owned() + &app_config.best_score.to_string());
    ui.label("Average Score: ".to_owned() + &app_config.average_score.to_string());

    ui.add(egui::ProgressBar::new(
        app_config.current_moves as f32 / app_config.allowed_moves as f32,
    ));

    if ui.button("Pause").clicked() {
        next_state.set(SimulationState::Paused)
    }
}
fn paused_ui(
    ui: &mut Ui,
    next_state: &mut NextState<SimulationState>,
    mut app_config: ResMut<AppConfig>,
) {
    ui.label("Simulation is paused");
    if ui.button("Resume").clicked() {
        next_state.set(SimulationState::Running)
    }
    ui.add(
        egui::Slider::new(&mut app_config.allowed_moves, 0..=4096)
            .text("allowed moves before evolution"),
    );
}
fn ui_controls(
    state: ResMut<AppConfig>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Space) {}
}
