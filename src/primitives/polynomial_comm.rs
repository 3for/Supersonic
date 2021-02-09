use crate::groups::{UnknownOrderGroup, HashPrime, ElemFrom};
use rand::Rng;
use rug::Integer;
use std::marker::PhantomData;

/// [Follow the idea in `https://github.com/ZenGo-X/class/src/primitives`]
/// AND [Follow the idea in `https://github.com/dignifiedquire/rust-accumulator/src/accumulator`]
/// Polynomial commitment as given in the paper: Transparent SNARKs from DARK Compilers
/// (https://eprint.iacr.org/2019/1229.pdf), subsection 4.2 and 4.3
/// The following algorithms are implemented:
/// Setup: generates public parameters
/// Commit: to commit to a polynomial
/// Open: open and verify a commitment
/// Encode: stand alone code to encode a polynomial as an integer
/// Decode: converts integer to a unique polynomial
/// Eval_prover: NI proof that y = f(z) for a committed polynomial f()
/// Eval_verify: NI verifier for eval_proof.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PP<G: UnknownOrderGroup, T> {
    phantom: PhantomData<*const T>,
    pub disc: G::Elem,
    pub g: G::Elem,
    pub q: G::Elem,
    pub p: G::Elem,
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PolyComm<G: UnknownOrderGroup, T> {
    phantom: PhantomData<T>,
    pub c: G::Elem,
}

impl<G:  ElemFrom<(Integer, Integer, Integer)> + HashPrime + UnknownOrderGroup, T> PolyComm<G, T> {    
    // `d_max` is the max degree of the polynomial
    pub fn setup( _lambda: usize, d_max: usize) -> PP<G, T>
    {
        let disc = G::unknown_order_elemnew();

        let g = G::unknown_order_elemnew();

        let mut random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        let a = G::pick_prime_integer(&random_bytes);
        random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        let b = G::pick_prime_integer(&random_bytes);
        random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        let c = G::pick_prime_integer(&random_bytes);

        let p = G::elemnew((a, b, c));

        let bound =  2 * (((d_max + 1) as f64).log2() as usize) + 1;
        let q = G::exp(&p, &Integer::from(bound)).unwrap();
        PP::<G, T>  { phantom: PhantomData, disc, g, p, q}

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::classgroupsti::ClassyGroup;

    #[test]
    fn test_poly_setup() {
        let d_max = 2;
        let _lambda = 2048;
        let _a = ClassyGroup::unknown_order_elem();
        let _pp = PolyComm::<ClassyGroup, Integer>::setup(_lambda, d_max);
        
    }
}