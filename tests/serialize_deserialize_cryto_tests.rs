use mini_bitoin::crypto::{
    crypto_utils::{encode_base58, to_32bytes_vec_big_endian},
    private_key::PrivateKey,
    s256_point::S256Point,
    signature::Signature,
};
use num_bigint::{BigInt, Sign};
use sha2::{Digest, Sha256};

#[test]
fn s256_point_serailize_deserialize_test() {
    let priv_key = PrivateKey::new("my secret");
    let uncompressed = &priv_key.point.clone().sec(false);
    assert!(S256Point::parse(uncompressed.clone()).inner == priv_key.point.inner);
}

#[test]
fn s256_point_serailize_deserialize_compress_test() {
    let priv_key = PrivateKey::new("my secret");
    let compressed = &priv_key.point.clone().sec(true);
    assert!(S256Point::parse(compressed.clone()).inner == priv_key.point.inner);
}

#[test]
fn signature_serialize_deserialize_test() {
    let hash = Sha256::digest("my message is");
    let z = BigInt::from_bytes_be(Sign::Plus, &hash);
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
    assert!(
        encode_base58(&a_bi_32_be) == "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6".to_string()
    );

    let b_hex_decode =
        hex::decode("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
    // let b = "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c".to_string();
    // let b_bi = BigInt::parse_bytes(b.as_bytes(), 16).unwrap();
    // let b_bi_32_be = to_32bytes_vec_big_endian(&b_bi);
    assert!(
        encode_base58(&b_hex_decode) == "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd".to_string()
    );

    let c = "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6".to_string();
    let c_bi = BigInt::parse_bytes(c.as_bytes(), 16).unwrap();
    let c_bi_32_be = to_32bytes_vec_big_endian(&c_bi);
    assert!(
        encode_base58(&c_bi_32_be) == "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7".to_string()
    );
}

#[test]
fn address_gen_test() {
    let priv_key = PrivateKey::new(BigInt::from(5002));
    let account_address = priv_key.point.address(false, true);
    // println!("{}", account_address);
    assert!(account_address == "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA");

    let priv_key = PrivateKey::new(BigInt::from(2020).pow(5));
    let account_address = priv_key.point.address(true, true);
    // println!("{}", account_address);
    assert!(account_address == "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH");

    let priv_key = PrivateKey::new(BigInt::parse_bytes(b"12345deadbeef", 16).unwrap());
    let account_address = priv_key.point.address(true, false);
    // println!("{}", account_address);
    assert!(account_address == "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1");
}

#[test]
fn wif_gen_test() {
    let priv_key = PrivateKey::new(BigInt::from(5003));
    let wif = priv_key.wif(true, true);
    assert!(wif == "cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK".to_string());

    let priv_key = PrivateKey::new(BigInt::from(2021).pow(5));
    let wif = priv_key.wif(false, true);
    assert!(wif == "91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic".to_string());

    let priv_key = PrivateKey::new(BigInt::parse_bytes(b"54321deadbeef", 16).unwrap());
    let wif = priv_key.wif(true, false);
    assert!(wif == "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a".to_string());
}
