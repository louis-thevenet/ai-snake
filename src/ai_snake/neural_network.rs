#[derive(Clone)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}
#[derive(Clone)]

pub struct Layer {
    input_dim: usize,
    output_dim: usize,
    pub weights: Vec<Vec<f64>>,
    //biases: Vec<f64>,
    activation: ActionFunction,
}
#[derive(Clone)]
pub enum ActionFunction {
    Relu,
    Sigmoid,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork { layers: vec![] }
    }

    pub fn add_layer(&mut self, layer: Layer) -> &Self {
        self.layers.push(layer);
        self
    }

    pub fn forward(&self, mut input: Vec<f64>) -> Vec<f64> {
        for layer in &self.layers {
            input = layer.forward(input);
        }
        input
    }

    pub fn mutate(&mut self, mutation_factor: f64) {
        for layer in &mut self.layers {
            for i in 0..layer.output_dim {
                for j in 0..layer.input_dim {
                    let mut rand = rand::random::<f64>();
                    if rand < mutation_factor {
                        layer.weights[i][j] += rand::random::<f64>() * 2. - 1.;
                    }
                }
            }
        }
    }
}

impl Layer {
    pub fn new(
        input_dim: usize,
        output_dim: usize,
        weights: Vec<Vec<f64>>,
        //biases: Vec<f64>,
        activation: ActionFunction,
    ) -> Self {
        Layer {
            input_dim,
            output_dim,
            weights,
            //biases,
            activation,
        }
    }
    pub fn forward(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output = vec![0.0; self.output_dim];

        for i in 0..self.output_dim {
            for j in 0..self.input_dim {
                output[i] += input[j] * self.weights[i][j];
            }
        }

        match self.activation {
            ActionFunction::Relu => {
                (0..self.output_dim).for_each(|i| {
                    if output[i] < 0.0 {
                        output[i] = 0.0;
                    }
                });
            }
            ActionFunction::Sigmoid => {
                (0..self.output_dim).for_each(|i| {
                    output[i] = 1.0 / (1.0 + (-output[i]).exp());
                });
            }
        }
        output
    }
}
