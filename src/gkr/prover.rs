use crate::gkr::circuit::Circuit;
use ark_ff::Field;

pub struct Prover<F: Field> {
    circuit: Circuit<F>,
}

impl<F: Field> Prover<F> {
    // TODO: change for taking a circuit as input
    pub fn new(circuit: &Circuit<F>) -> Self {
        Self {
            circuit: circuit.clone(),
        }
    }
}
