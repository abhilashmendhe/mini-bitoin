use mini_bitoin::crypto::{s256_field::S256Field, s256_point::S256Point, signature::Signature};
use num_bigint::BigInt;
use once_cell::sync::Lazy;

const G: Lazy<S256Point> = Lazy::new(|| {
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
    let gx = S256Field::new(gx, None);
    let gy = S256Field::new(gy, None);
    S256Point::new(gx, gy, None, None)
});

#[test]
fn verify_bigint_sign() {
    let g = G;

    // Test 1
    let z = BigInt::parse_bytes(
        b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
        16,
    )
    .unwrap();
    let r = BigInt::parse_bytes(
        b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
        16,
    )
    .unwrap();
    let s = BigInt::parse_bytes(
        b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
        16,
    )
    .unwrap();
    let sign = Signature::new(r, s.clone());
    let px = BigInt::parse_bytes(
        b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
        16,
    )
    .unwrap();
    let py = BigInt::parse_bytes(
        b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
        16,
    )
    .unwrap();
    let px = S256Field::new(px, None);
    let py = S256Field::new(py, None);
    let point = S256Point::new(px, py, None, None);
    let f = point.verify(z, sign, (*g).clone());
    assert!(f);

    let px = BigInt::parse_bytes(
        b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
        16,
    )
    .unwrap();
    let py = BigInt::parse_bytes(
        b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
        16,
    )
    .unwrap();
    let px = S256Field::new(px, None);
    let py = S256Field::new(py, None);
    let point = S256Point::new(px, py, None, None);

    // Test 2
    let z = BigInt::parse_bytes(
        b"ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
        16,
    )
    .unwrap();
    let r = BigInt::parse_bytes(
        b"ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
        16,
    )
    .unwrap();
    let s = BigInt::parse_bytes(
        b"68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
        16,
    )
    .unwrap();
    let sign = Signature::new(r, s.clone());
    let f = point.clone().verify(z, sign, (*g).clone());
    assert!(f);

    // Test 3
    let z = BigInt::parse_bytes(
        b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
        16,
    )
    .unwrap();
    let r = BigInt::parse_bytes(
        b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
        16,
    )
    .unwrap();
    let s = BigInt::parse_bytes(
        b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
        16,
    )
    .unwrap();
    let sign = Signature::new(r, s.clone());
    let f = point.verify(z, sign, (*g).clone());
    assert!(f);
}
