use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Clone)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}
#[derive(Clone)]

pub struct Layer {
    pub input_dim: usize,
    pub output_dim: usize,
    pub weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
    activation: ActivationFunction,
}
#[derive(Clone)]
pub enum ActivationFunction {
    Relu,
    Sigmoid,
    Softmax,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork { layers: vec![] }
    }

    pub fn add_layer(&mut self, layer: Layer) -> &mut Self {
        self.layers.push(layer);
        self
    }

    pub fn forward(&self, mut input: Vec<f64>) -> Vec<f64> {
        for layer in &self.layers {
            input = layer.forward(input);
        }
        self.normalize(input)
    }

    fn normalize(&self, v: Vec<f64>) -> Vec<f64> {
        let sum = v.iter().sum::<f64>();
        if sum > 0.0 {
            v.into_iter().map(|x| x / sum).collect()
        } else {
            v
        }
    }

    pub fn mutate(&mut self, mutation_factor: f64) {
        for layer in &mut self.layers {
            for i in 0..layer.output_dim {
                for j in 0..layer.input_dim {
                    let rand = rand::random::<f64>();
                    if rand < mutation_factor {
                        layer.weights[j][i] = rand::random::<f64>();
                    }
                    let rand = rand::random::<f64>();
                    if rand < mutation_factor {
                        layer.biases[i] = rand::random::<f64>();
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
        biases: Vec<f64>,
        activation: ActivationFunction,
    ) -> Self {
        Layer {
            input_dim,
            output_dim,
            weights,
            biases,
            activation,
        }
    }
    pub fn forward(&self, input: Vec<f64>) -> Vec<f64> {
        if input.is_empty() {
            return vec![];
        }
        let mut output = vec![0.0; self.output_dim];

        (0..self.output_dim).for_each(|j| {
            (0..self.input_dim).for_each(|k| {
                output[j] += input[k] * self.weights[k][j]; // + self.biases[j] * 0.15;
            });
        });

        match self.activation {
            ActivationFunction::Relu => {
                (0..self.output_dim).for_each(|i| {
                    if output[i] < 0.0 {
                        output[i] = 0.0;
                    }
                });
            }
            ActivationFunction::Sigmoid => {
                (0..self.output_dim).for_each(|i| {
                    output[i] = 1.0 / (1.0 + (-output[i]).exp());
                });
            }
            ActivationFunction::Softmax => {
                let mut sum = 0.0;
                (0..self.output_dim).for_each(|i| {
                    output[i] = (-output[i]).exp();
                    sum += output[i];
                });
                (0..self.output_dim).for_each(|i| {
                    output[i] /= sum;
                })
            }
        }
        output
    }
}
impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Layer : input_dim={}, output_dim={}",
            self.input_dim, self.output_dim
        )?;

        for i in 0..self.weights.len() {
            for j in 0..self.weights[i].len() {
                write!(f, "{:.2}, ", self.weights[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for ActivationFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActivationFunction::Relu => write!(f, "relu"),
            ActivationFunction::Sigmoid => write!(f, "sigmoid"),
            ActivationFunction::Softmax => write!(f, "softmax"),
        }
    }
}
impl fmt::Display for NeuralNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for layer in self.layers.iter() {
            writeln!(f, "{}", layer)?;
        }
        Ok(())
    }
}
