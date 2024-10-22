mod gkr;
mod sumcheck;
use crate::gkr::circuit::{Circuit, GateType};
use crate::sumcheck::{
    interactive::interactive_protocol, non_interactive::non_interactive_protocol,
    prover::Prover as SumCheckProver, verifier::Verifier,
};

use crate::gkr::prover::Prover as GKRProver;
use ark_bls12_381::Fq;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial, Polynomial,
};

fn main() {
    // g(x_0, x_1, x_2) = 2*x_0^3 + x_0*x_2 + x_1*x_2
    let g = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (Fq::from(2), SparseTerm::new(vec![(0, 3)])),
            (Fq::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (Fq::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );

    let p = SumCheckProver::new(&g).unwrap();
    let v = Verifier::new(&g);

    println!("[sumcheck] starting interactive protocol");
    let mut valid = interactive_protocol(&p, &v);
    if !valid {
        panic!("[sumcheck] interactive protocol is not valid");
    }
    println!("[sumcheck] interactive protocol is valid");

    println!("[sumcheck] starting non-interactive protocol");
    let proof = non_interactive_protocol(&p);
    valid = v.verify_non_interactive_proof(&p.g, &p.h, p.g.degree(), &proof);
    if !valid {
        panic!("[sumcheck] interactive protocol is not valid");
    }
    println!("[sumcheck] non-interactive protocol is valid");

    println!("[gkr] starting interactive gkr protocol");
    // todo create circuit and then call prover
    let circuit_layer = vec![
        vec![GateType::Add, GateType::Add, GateType::Add, GateType::Add], // First layer (after inputs)
        vec![GateType::Add, GateType::Add],                               // Second layer
        vec![GateType::Add],                                              // Top layer
    ];
    let inputs = [
        Fq::from(10),
        Fq::from(1),
        Fq::from(2),
        Fq::from(3),
        Fq::from(11),
        Fq::from(4),
        Fq::from(5),
        Fq::from(7),
    ];
    let circuit: Circuit<Fq> = Circuit::new(circuit_layer, &inputs);
    println!("{}", circuit);
    circuit.add_i_mle(1);
    let p: GKRProver<Fq> = GKRProver::new(&circuit);
}
