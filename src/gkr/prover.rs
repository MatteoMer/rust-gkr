use crate::gkr::circuit::Circuit;
use ark_ff::Field;

pub struct Prover<'a, F: Field> {
    circuit: &'a Circuit<F>,
}

impl<'a, F: Field> Prover<'a, F> {
    pub fn new(circuit: &'a Circuit<F>) -> Self {
        Self { circuit }
    }
}
