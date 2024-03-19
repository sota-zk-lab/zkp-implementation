use num_bigint::BigUint;

pub const MODULES_BIT_SIZE: usize = 255;

/// Return the order of the group(s) defined over elliptic curves in BLS12-381.
pub fn curve_order() -> BigUint {
    BigUint::parse_bytes(
        b"52435875175126190479447740508185965837690552500527637822603658699938581184513",
        10,
    )
        .unwrap()
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    #[test]
    fn check_pow() {
        let a: BigUint = BigUint::from(5u64);
        let c = BigUint::from(2u64);
        let b: BigUint = BigUint::from(5u64);
        let pow_of_root = c.modpow(&(&(b.clone() - BigUint::from(1u64)) / BigUint::from(2u64)), &b);

        assert_eq!(
            pow_of_root,
            BigUint::from(4usize)
        );
    }
}

// #[test]
// fn check_pow_2() {
//     let modulus = curve_order();
//     assert_eq!(
//
//     );
// }
