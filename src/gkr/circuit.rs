use std::fmt;

use ark_ff::Field;
use ark_poly::DenseMultilinearExtension;

#[derive(Clone, Copy, PartialEq)]
pub enum GateType {
    Add,
    Mult,
}

impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == GateType::Add {
            write!(f, "+")
        } else {
            write!(f, "*")
        }
    }
}

#[derive(Clone, Copy)]
struct Gate<F: Field> {
    inputs: [F; 2],
    output: F,
    gate_type: GateType,
}

#[derive(Clone)]
struct Layer<F: Field> {
    gates: Vec<Gate<F>>,
    k: usize,
}

#[derive(Clone)]
pub struct Circuit<F: Field> {
    layers: Vec<Layer<F>>,
    d: usize,
}

impl<F: Field> fmt::Display for Circuit<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, layer) in self.layers.iter().enumerate() {
            for (j, gate) in layer.gates.iter().enumerate() {
                writeln!(
                    f,
                    "[gkr][layer {}|gate {}] {}{}{}={}",
                    i, j, gate.inputs[0], gate.gate_type, gate.inputs[1], gate.output
                )?;
            }
        }
        Ok(())
    }
}

impl<F: Field> Circuit<F> {
    // create and evaluate a circuit based on his layer and inputs
    pub fn new(circuit_layer: Vec<Vec<GateType>>, inputs: &[F]) -> Self {
        //TODO: more checks based on the layer of the circuit
        if inputs.len() % 2 != 0 {
            panic!("[gkr] inputs len must be even")
        }
        let mut layers: Vec<Layer<F>> = vec![];
        let mut cur_inputs = inputs.to_vec();

        for layer in circuit_layer.iter() {
            let mut gates: Vec<Gate<F>> = vec![];
            let mut new_inputs = vec![];
            for gate in layer {
                let current_input_pair = [cur_inputs.pop().unwrap(), cur_inputs.pop().unwrap()];
                let output = match gate {
                    GateType::Add => current_input_pair[0] + current_input_pair[1],
                    GateType::Mult => current_input_pair[0] * current_input_pair[1],
                };
                gates.push(Gate {
                    inputs: current_input_pair,
                    output,
                    gate_type: gate.clone(),
                });
                new_inputs.push(output);
            }
            let s_i = gates.len();
            layers.push(Layer {
                gates,
                k: (s_i as f64).log2() as usize, //S_i = 2^k
            });
            cur_inputs = new_inputs;
        }

        // Reverse so the ouput layer is at index 0
        layers.reverse();

        Self {
            layers: layers.clone(),
            d: layers.len(),
        }
    }

    fn w_i(&self, i: usize, gate: usize) -> F {
        self.layers[i].gates[gate].output
    }

    fn add_i(&self, i: usize, a: usize, b: usize, c: usize) -> bool {
        // Return 1 if and only if:
        // 1. Gate 'a' in layer i is an addition gate
        // 2. Gate 'b' in layer i+1 is the first input to gate 'a'
        // 3. Gate 'c' in layer i+1 is the second input to gate 'a'
        if a >= self.layers[i].gates.len()
            || b >= self.layers[i + 1].gates.len()
            || c >= self.layers[i + 1].gates.len()
        {
            return false;
        }

        let gate_a = &self.layers[i].gates[a];
        let gate_b = &self.layers[i + 1].gates[b];
        let gate_c = &self.layers[i + 1].gates[c];

        // Check if:
        // 1. Gate a is an addition gate
        // 2. First input of gate a matches the output of gate b
        // 3. Second input of gate a matches the output of gate c
        gate_a.gate_type == GateType::Add
            && gate_a.inputs[0] == gate_b.output
            && gate_a.inputs[1] == gate_c.output
    }

    fn mult_i(&self, i: usize, a: usize, b: usize, c: usize) -> bool {
        // Similar to add_i but for multiplication gates
        if a >= self.layers[i].gates.len()
            || b >= self.layers[i + 1].gates.len()
            || c >= self.layers[i + 1].gates.len()
        {
            return false;
        }

        let gate_a = &self.layers[i].gates[a];
        let gate_b = &self.layers[i + 1].gates[b];
        let gate_c = &self.layers[i + 1].gates[c];

        gate_a.gate_type == GateType::Mult
            && gate_a.inputs[0] == gate_b.output
            && gate_a.inputs[1] == gate_c.output
    }

    fn extract_index(value: usize, start_bit: usize, num_bits: usize) -> usize {
        if num_bits == 0 {
            return 0;
        }
        (value >> start_bit) & ((1 << num_bits) - 1)
    }

    // eval add_i MLE at r_i point
    // to create the MLE we need to encode wiring predicates and put them to 1
    // while all other points should be 0 in the domain.
    pub fn add_i_mle(&self, layer_i: usize) -> DenseMultilinearExtension<F> {
        let current_layer = &self.layers[layer_i];
        let next_layer = &self.layers[layer_i + 1];

        // Calculate the number of variables needed for each layer
        let current_layer_bits = if current_layer.gates.len() > 1 {
            ark_std::log2(current_layer.gates.len().next_power_of_two()) as usize
        } else {
            1
        };

        let next_layer_bits = if next_layer.gates.len() > 1 {
            ark_std::log2(next_layer.gates.len().next_power_of_two()) as usize
        } else {
            1
        };

        let num_variables = current_layer_bits + 2 * next_layer_bits;

        // Create evaluations for all possible combinations
        let num_points = 1 << num_variables;
        let mut evaluations = Vec::with_capacity(num_points);

        println!("Layer {} MLE construction:", layer_i);
        println!("Current layer bits: {}", current_layer_bits);
        println!("Next layer bits: {}", next_layer_bits);
        println!("Total variables: {}", num_variables);
        println!("Number of points: {}", num_points);

        for i in 0..num_points {
            // Extract gate indices from the binary representation
            let a = Self::extract_index(i, 0, current_layer_bits);
            let b = Self::extract_index(i, current_layer_bits, next_layer_bits);
            let c = Self::extract_index(i, current_layer_bits + next_layer_bits, next_layer_bits);

            let is_valid = self.add_i(layer_i, a, b, c);

            if is_valid {
                println!("Valid wiring found: gate {} in layer {} connected to gates {} and {} in layer {}", 
                    a, layer_i, b, c, layer_i + 1);
            }

            evaluations.push(if is_valid { F::one() } else { F::zero() });
        }

        DenseMultilinearExtension::from_evaluations_vec(num_variables, evaluations)
    }

    // Similar to add_i_mle
    pub fn mult_i_mle(&self, layer_i: usize) -> DenseMultilinearExtension<F> {
        let current_layer = &self.layers[layer_i];
        let next_layer = &self.layers[layer_i + 1];

        let current_layer_bits = if current_layer.gates.len() > 1 {
            ark_std::log2(current_layer.gates.len().next_power_of_two()) as usize
        } else {
            1
        };

        let next_layer_bits = if next_layer.gates.len() > 1 {
            ark_std::log2(next_layer.gates.len().next_power_of_two()) as usize
        } else {
            1
        };

        let num_variables = current_layer_bits + 2 * next_layer_bits;

        let num_points = 1 << num_variables;
        let mut evaluations = Vec::with_capacity(num_points);

        println!("Layer {} MLE construction:", layer_i);
        println!("Current layer bits: {}", current_layer_bits);
        println!("Next layer bits: {}", next_layer_bits);
        println!("Total variables: {}", num_variables);
        println!("Number of points: {}", num_points);

        for i in 0..num_points {
            let a = Self::extract_index(i, 0, current_layer_bits);
            let b = Self::extract_index(i, current_layer_bits, next_layer_bits);
            let c = Self::extract_index(i, current_layer_bits + next_layer_bits, next_layer_bits);

            let is_valid = self.mult_i(layer_i, a, b, c);

            if is_valid {
                println!("Valid wiring found: gate {} in layer {} connected to gates {} and {} in layer {}", 
                    a, layer_i, b, c, layer_i + 1);
            }

            evaluations.push(if is_valid { F::one() } else { F::zero() });
        }

        DenseMultilinearExtension::from_evaluations_vec(num_variables, evaluations)
    }
}
