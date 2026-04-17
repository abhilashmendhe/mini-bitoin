use mini_bitoin::crypto::{crypto_utils::{base58, to_32bytes_vec_big_endian}, hash_helper::single_hash, private_key::PrivateKey, s256_point::S256Point, signature::Signature};
use num_bigint::BigInt;

#[test]
fn s256_point_serailize_deserialize_test() {
    let priv_key = PrivateKey::new("my secret");
    let uncompressed = &priv_key.point.clone().sec(false);
    assert!(S256Point::parse(uncompressed.clone()).inner==priv_key.point.inner);
}

#[test]
fn s256_point_serailize_deserialize_compress_test() {
    let priv_key = PrivateKey::new("my secret");
    let compressed = &priv_key.point.clone().sec(true);
    assert!(S256Point::parse(compressed.clone()).inner == priv_key.point.inner);
}

#[test]
fn signature_serialize_deserialize_test() {
    let (z, _) = single_hash("my message is".to_string());
    let priv_key = PrivateKey::new("my secret");
    let sign = priv_key.sign(z.clone());
    let sign_der = sign.der();
    let (r, s) = Signature::un_der(sign_der);
    assert_eq!(r, sign.r);
    assert_eq!(s, sign.s);
}

#[test]
fn base58_test() {
    let a = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d".to_string();
    let a_bi = BigInt::parse_bytes(a.as_bytes(), 16).unwrap();
    let a_bi_32_be = to_32bytes_vec_big_endian(&a_bi);
    assert!(base58(&a_bi_32_be)=="9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6".to_string());

    let b = "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c".to_string();
    let b_bi = BigInt::parse_bytes(b.as_bytes(), 16).unwrap();
    let b_bi_32_be = to_32bytes_vec_big_endian(&b_bi);
    assert!(base58(&b_bi_32_be)=="4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd".to_string());

    let c = "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6".to_string();
    let c_bi = BigInt::parse_bytes(c.as_bytes(), 16).unwrap();
    let c_bi_32_be = to_32bytes_vec_big_endian(&c_bi);
    assert!(base58(&c_bi_32_be)=="EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7".to_string());
}