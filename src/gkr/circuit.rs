use std::fmt;

use ark_ff::Field;
use ark_poly::DenseMultilinearExtension;

#[derive(Clone, PartialEq)]
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

#[derive(Clone)]
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

        Self {
            layers: layers.clone(),
            d: layers.len(),
        }
    }

    fn w_i(&self, i: usize, gate: usize) -> F {
        self.layers[i].gates[gate].output
    }

    fn add_i(&self, i: usize, a: usize, b: usize, c: usize) -> bool {
        self.layers[i].gates[a].gate_type == GateType::Add
            && self.layers[i].gates[a].inputs[0] == self.layers[i + 1].gates[b].output
            && self.layers[i].gates[a].inputs[0] == self.layers[i + 1].gates[c].output
    }

    fn mult_i(&self, i: usize, a: usize, b: usize, c: usize) -> bool {
        self.layers[i].gates[a].gate_type == GateType::Mult
            && self.layers[i].gates[a].inputs[0] == self.layers[i + 1].gates[b].output
            && self.layers[i].gates[a].inputs[0] == self.layers[i + 1].gates[c].output
    }

    // eval add_i MLE at r_i point
    // to create the MLE we need to encode wiring predicates and put them to 1
    // while all other points should be 0 in the domain.
    pub fn add_i_mle(&self, i: usize) -> DenseMultilinearExtension<F> {
        let mut evals: Vec<F> = vec![];
        for gate in 0..2_usize.pow(self.layers[i].k as u32) {
            for gate_next in 0..2usize.pow(self.layers[i + 1].k as u32) {}
        }

        todo!()
    }
}
