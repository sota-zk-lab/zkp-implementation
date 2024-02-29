use num_bigint::BigUint;

pub const MODULES_BIT_SIZE: usize = 255;
pub const PRIMITIVE_ROOT: usize = 5;



/// Return the order of the group(s) defined over elliptic curves in BLS12-281. The r in
/// Fr.
pub fn curve_order() -> BigUint {
    BigUint::parse_bytes(
        b"52435875175126190479447740508185965837690552500527637822603658699938581184513",
        10,
    )
    .unwrap()
}

//
// assert pow(PRIMITIVE_ROOT, (MODULUS - 1) // 2, MODULUS) != 1
// assert pow(PRIMITIVE_ROOT, MODULUS - 1, MODULUS) == 1
//

#[test]
fn check_pow() {
    let modulus = u32::
    let pow_of_root = PRIMITIVE_ROOT.pow(&(modulus.clone() - 1) / 2) % &modulus;

    assert_eq!(
        pow_of_root,
        BigUint::from(1usize)
    );

}

// #[test]
// fn check_pow_2() {
//     let modulus = curve_order();
//     assert_eq!(
//
//     );
// }
