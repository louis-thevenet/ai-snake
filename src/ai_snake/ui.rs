use bevy::prelude::*;
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

#[derive(Default, States, Debug, Hash, Eq, Clone, Copy, PartialEq)]

pub enum RenderingState {
    #[default]
    Enabled,
    Disabled,
}
#[derive(Default, Resource)]
pub struct AppConfig {
    pub generation_number: u64,
    pub best_score: u64,
    pub average_score: u64,
    pub grid_size: u64,
    pub population_size: u64,
    pub current_moves: u64,
    pub allowed_moves: u64,
    pub last_merged: u64,
    pub mutation_factor: f64,
    pub keep_x_best: f64,
    pub vision_range: i64,
    pub food_amount: u64,

    pub print_input: bool,
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppConfig>()
            .init_state::<SimulationState>()
            .init_state::<RenderingState>()
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
    app_state.population_size = 3000;
    app_state.current_moves = 0;
    app_state.allowed_moves = 800;
    app_state.last_merged = 0;
    app_state.mutation_factor = 0.4;
    app_state.keep_x_best = 0.02;
    app_state.vision_range = app_state.grid_size as i64;
    app_state.food_amount = 10;
    app_state.print_input = false;
}

fn build_ui(
    mut contexts: EguiContexts,
    mut app_config: ResMut<AppConfig>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    rendering_state: Res<State<RenderingState>>,
    mut next_rendering_state: ResMut<NextState<RenderingState>>,
) {
    egui::SidePanel::left("Menu")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            match sim_state.get() {
                SimulationState::StartUp => {}
                SimulationState::Evolving => {}
                SimulationState::Running => {
                    running_ui(ui, &mut next_sim_state, &app_config);
                }

                SimulationState::Paused | SimulationState::Stopped => {
                    stopped_ui(ui, &mut app_config, sim_state, &mut next_sim_state);
                }
            }

            ui.collapsing("Controls", |ui| {
                ui.label("Camera controls: WASD/ZQSD/Arrows");
                ui.label("Zoom: Q,E/PageUp,PageDown");
                ui.label("Play/Pause: Space");
            });

            ui.collapsing("Advanced", |ui| {
                if ui.button("Enable/Disable Sprites Update").clicked() {
                    match rendering_state.get() {
                        RenderingState::Disabled => {
                            next_rendering_state.set(RenderingState::Enabled);
                        }
                        RenderingState::Enabled => {
                            next_rendering_state.set(RenderingState::Disabled);
                        }
                    }
                };
                ui.checkbox(&mut app_config.print_input, "Print I/O for model #0");
            });

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
}

fn stopped_ui(
    ui: &mut Ui,
    app_config: &mut ResMut<AppConfig>,
    sim_state: Res<State<SimulationState>>,
    next_state: &mut NextState<SimulationState>,
) {
    match *sim_state.get() {
        SimulationState::Stopped => {
            ui.heading("Config");
            if ui.button("Start").clicked() {
                next_state.set(SimulationState::StartUp)
            }
            ui.add(egui::Slider::new(&mut app_config.grid_size, 0..=128).text("grid size"));
            ui.add(
                egui::Slider::new(&mut app_config.population_size, 0..=10000)
                    .text("population size"),
            );
        }
        SimulationState::Paused => {
            ui.heading("Paused");
            if ui.button("Resume").clicked() {
                next_state.set(SimulationState::Running)
            }

            ui.add_enabled(
                false,
                egui::Slider::new(&mut app_config.grid_size, 0..=128).text("grid size"),
            );
            ui.add_enabled(
                false,
                egui::Slider::new(&mut app_config.population_size, 0..=10000)
                    .text("population size"),
            );
        }
        _ => (),
    }

    ui.add(
        egui::Slider::new(&mut app_config.allowed_moves, 0..=2000)
            .text("allowed moves before evolution"),
    );

    ui.add(egui::Slider::new(&mut app_config.mutation_factor, 0.0..=1.0).text("Mutation factor"));

    ui.add(
        egui::Slider::new(&mut app_config.keep_x_best, 0.0..=0.2)
            .text("Selection factor (merged if score > x*best_score)"),
    );
    ui.add(egui::Slider::new(&mut app_config.vision_range, 0..=256).text("Vision range"));
    ui.add(egui::Slider::new(&mut app_config.food_amount, 0..=256).text("Food ammount"));
}

fn running_ui(
    ui: &mut Ui,
    next_state: &mut NextState<SimulationState>,
    app_config: &ResMut<AppConfig>,
) {
    ui.heading("Running");
    if ui.button("Pause").clicked() {
        next_state.set(SimulationState::Paused)
    }
    ui.label("Agents number: ".to_owned() + &app_config.population_size.to_string());
    ui.label("Generation #".to_owned() + &app_config.generation_number.to_string());
    ui.label("Best Score: ".to_owned() + &app_config.best_score.to_string());
    ui.label("Average Score: ".to_owned() + &app_config.average_score.to_string());
    ui.label("Last Merged: ".to_owned() + &app_config.last_merged.to_string());

    ui.add(egui::ProgressBar::new(
        app_config.current_moves as f32 / app_config.allowed_moves as f32,
    ));
}

fn ui_controls(
    keys: Res<ButtonInput<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match sim_state.get() {
            SimulationState::Stopped => next_sim_state.set(SimulationState::StartUp),
            SimulationState::Paused => next_sim_state.set(SimulationState::Running),
            SimulationState::Running => next_sim_state.set(SimulationState::Paused),
            _ => (),
        }
    }
}
