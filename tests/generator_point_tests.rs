use mini_bitoin::{
    elliptic_curve::ecc_point::Point,
    finite_fields::{field_element::FieldElement, modulo_helper::modulo},
};
use num_bigint::BigInt;
use once_cell::sync::Lazy;

const N: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
});

const P: Lazy<BigInt> =
    Lazy::new(|| BigInt::from(2).pow(256) - BigInt::from(2).pow(32) - BigInt::from(977));

#[test]
fn generator_point_on_curve() {
    // 1. Check generator point, G, is on the curve y2 = x3 + 7:
    let gx = BigInt::parse_bytes(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let gy = BigInt::parse_bytes(
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();
    let p = P;
    let lhs = modulo(gy.pow(2), (*p).clone());
    let rhs = modulo(gx.pow(3) + BigInt::from(7), (*p).clone());
    assert_eq!(lhs, rhs);
}

#[test]
fn generator_point_hash_order_n() {
    // 2. Verify generator point G, has order n
    let gx = BigInt::parse_bytes(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let gy = BigInt::parse_bytes(
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();
    let p = P;
    let n = N;
    let x = FieldElement::new(gx, (*p).clone());
    let y = FieldElement::new(gy, (*p).clone());
    let seven = FieldElement::new(BigInt::from(7), (*p).clone());
    let zero = FieldElement::new(BigInt::from(0), (*p).clone());
    let g = Point::new(x, y, zero, seven);
    assert_eq!(g * (*n).clone(), Point::inifinity());
}
