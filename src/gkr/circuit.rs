use ark_ff::Field;

#[derive(Clone)]
pub enum GateType {
    Add,
    Mul,
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
}

#[derive(Clone)]
pub struct Circuit<F: Field> {
    layers: Vec<Layer<F>>,
    d: usize,
}

impl<F: Field> Circuit<F> {
    pub fn new(circuit_layer: Vec<Vec<GateType>>, inputs: &[F]) -> Self {
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
                    GateType::Mul => current_input_pair[0] * current_input_pair[1],
                };
                gates.push(Gate {
                    inputs: current_input_pair,
                    output,
                    gate_type: gate.clone(),
                });
                new_inputs.push(output);
            }
            layers.push(Layer { gates });
            cur_inputs = new_inputs;
        }

        Self {
            layers: layers.clone(),
            d: layers.len(),
        }
    }

    pub fn display(&self) {
        for (i, layer) in self.layers.iter().enumerate() {
            for (j, gate) in layer.gates.iter().enumerate() {
                println!(
                    "[gkr][layer {}|gate {}] {}.{}={}",
                    i, j, gate.inputs[0], gate.inputs[1], gate.output
                );
            }
        }
    }
}
