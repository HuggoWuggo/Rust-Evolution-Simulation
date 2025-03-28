use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }
}
#[derive(Debug, Clone)]
struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self { neurons }
    }
}

#[derive(Debug, Clone)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug, Clone)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    mod random {
        use super::*;
        #[test]
        fn neuron_bias() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            assert_relative_eq!(neuron.bias, -0.6255188);
        }

        #[test]
        fn neuron_weights() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
            );
        }
    }

    mod propagate {
        use super::*;
        #[test]
        fn neuron_propagate() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

            assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
        }

        #[test]
        fn layer_propagate() {
            let neurons = (
                Neuron {
                    bias: 0.0,
                    weights: vec![0.1, 0.2, 0.3],
                },
                Neuron {
                    bias: 0.0,
                    weights: vec![0.4, 0.5, 0.6],
                },
            );

            let layer = Layer {
                neurons: vec![neurons.0.clone(), neurons.1.clone()],
            };

            let inputs = &[-0.5, 0.0, 0.5];

            let actual = layer.propagate(inputs.to_vec());
            let expect = vec![
                neurons.0.propagate(inputs.as_ref()),
                neurons.1.propagate(inputs.as_ref()),
            ];

            assert_relative_eq!(actual.as_slice(), expect.as_slice());
        }

        #[test]
        fn network_propagate() {
            let layers = (
                Layer {
                    neurons: (vec![
                        Neuron {
                            bias: 0.0,
                            weights: vec![-0.5, -0.4, -0.3],
                        },
                        Neuron {
                            bias: 0.0,
                            weights: vec![-0.2, -0.1, 0.0],
                        },
                    ]),
                },
                Layer {
                    neurons: (vec![Neuron {
                        bias: 0.0,
                        weights: vec![-0.5, 0.5],
                    }]),
                },
            );
            let network = Network {
                layers: (vec![layers.0.clone(), layers.1.clone()]),
            };

            let actual = network.propagate(vec![0.5, 0.6, 0.7]);
            let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
