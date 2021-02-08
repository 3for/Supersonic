use crate::groups::UnknownOrderGroup;
//use rand::CryptoRng;
//use rand::Rng;

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
pub struct PP<G: UnknownOrderGroup> {
    pub security_level: usize,
    pub disc: G::Elem,
    pub g: G::Elem,
    pub q: G::Elem,
    pub p: G::Elem,
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PolyComm {
    pub c: G::Elem,
}

impl PolyComm {
    /*pub fn KeyGen_RSAsetup<G, R>(rng: &mut R, _lambda: usize) -> PP 
    where 
        G: UnknownOrderGroup,
        R: CryptoRng + Rng,
    {

    }*/
}

