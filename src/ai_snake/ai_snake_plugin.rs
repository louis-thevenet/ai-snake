use bevy::app::{App, Plugin};

use super::{
    simulation::SimulationPlugin, simulation_rendering::render_sim_plugin::RenderSimulationPlugin,
    ui::UIPlugin,
};

pub struct AISnakePlugin;

impl Plugin for AISnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenderSimulationPlugin, SimulationPlugin, UIPlugin));
    }
}
