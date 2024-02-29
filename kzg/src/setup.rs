use num_bigint::BigUint;
use oblast::{curve_order};

/// Generate trusted setup, in coefficient form.
/// For data availability we always need to compute the polynomials anyway,
/// so it makes little sense to do things in Lagrange space.

#[derive(Debug, PartialEq, Eq)]
pub struct Setup {
    pub in_g1: Vec<P1>,
    pub in_ge: P2,
}

