use crate::snake_core::universe::Universe;

use super::neural_network::NeuralNetwork;
pub struct Model {
    pub universe: Universe,
    pub brain: NeuralNetwork,
    pub score: u32,
}

impl Model {
    pub fn new(width: u64, height: u64, brain: NeuralNetwork) -> Self {
        let universe = Universe::new_empty(width, height);
        let score = 0;
        Model {
            universe,
            brain,
            score,
        }
    }
}
