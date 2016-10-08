use std::vec::Drain;
use dependencies::rand::{Rng, thread_rng};
use network::{NeuralNetwork};


pub struct EvolutionaryTrainer {
    next_generation: Vec<NeuralNetwork>,
    generation: Vec<Species>,
}

impl EvolutionaryTrainer {
    pub fn new(first_generation: Vec<NeuralNetwork>) -> EvolutionaryTrainer {
        EvolutionaryTrainer {
            next_generation: first_generation,
            generation: vec!(),
        }
    }

    pub fn train(&mut self, mut rewards: Vec<i64>) {
        assert_eq!(self.next_generation.len(), rewards.len());

        let mut rewards_iter: Drain<i64> = rewards.drain(..);

        for network in self.next_generation.drain(..) {
            self.generation.push(Species::new(rewards_iter.next().unwrap_or_else(|| panic!("Rewards iter next was none")), network));
        }

        self.generation.sort_by_key(|species| species.get_fitness());

        let first = self.generation.pop().unwrap_or_else(|| panic!("Generation pop was none"));
        let second = self.generation.pop().unwrap_or_else(|| panic!("Generation pop was none"));

        for _ in 0..4 {
            self.next_generation.push(first.cross(&second));
        }
        for species in &self.generation {
            self.next_generation.push(first.cross(species));
        }
    }

    pub fn get_next_generation(&self) -> &Vec<NeuralNetwork> {
        &self.next_generation
    }
}

struct Species {
    fitness: i64,
    network: NeuralNetwork,
}

impl Species {
    fn new(fitness: i64, network: NeuralNetwork) -> Species {
        Species {
            fitness: fitness,
            network: network,
        }
    }

    fn get_fitness(&self) -> i64 {
        self.fitness
    }

    fn get_network(&self) -> &NeuralNetwork {
        &self.network
    }

    fn cross(&self, other: &Species) -> NeuralNetwork {
        let mut net_weights_1 = self.get_network().get_weights_and_bias();
        let mut net_weights_2 = other.get_network().get_weights_and_bias();
        assert_eq!(net_weights_1.len(), net_weights_2.len());

        let mut net_iter_1: Drain<Vec<Vec<f64>>> = net_weights_1.drain(..);
        let mut net_iter_2: Drain<Vec<Vec<f64>>> = net_weights_2.drain(..);

        let mut child_net = vec!();

        let mut rng = thread_rng();

        for mut layer_weights_1 in net_iter_1 {
            let mut layer_weights_2 = net_iter_2.next().unwrap_or_else(|| panic!("Net 2 iter next was none"));
            assert_eq!(layer_weights_1.len(), layer_weights_2.len());

            let mut child_layer = vec!();

            let mut layer_iter_1: Drain<Vec<f64>> = layer_weights_1.drain(..);
            let mut layer_iter_2: Drain<Vec<f64>> = layer_weights_2.drain(..);

            for mut neuron_weights_1 in layer_iter_1 {
                let mut neuron_weights_2 = layer_iter_2.next().unwrap_or_else(|| panic!("Layer iter 2 next was none"));
                assert_eq!(neuron_weights_1.len(), neuron_weights_2.len());

                let mut child_neuron = vec!();

                let mut neuron_iter_1: Drain<f64> = neuron_weights_1.drain(..);
                let mut neuron_iter_2: Drain<f64> = neuron_weights_2.drain(..);

                for weight_1 in neuron_iter_1 {
                    let weight_2 = neuron_iter_2.next().unwrap_or_else(|| panic!("Neuron iter 2 next was none"));

                    let mutation_mult = {
                        if rng.gen_range(0, 20) == 0 {
                            rng.gen_range(0.5, 2.0)
                        } else {
                            1.0
                        }
                    };

                    child_neuron.push(*rng.choose(&[weight_1, weight_2]).unwrap_or_else(|| panic!("Not fucking possible")) * mutation_mult);
                }

                child_layer.push(child_neuron);
            }
            child_net.push(child_layer);
        }
        NeuralNetwork::new(child_net)
    }
}
