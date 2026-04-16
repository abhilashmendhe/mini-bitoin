use num_bigint::{BigInt, Sign};
use sha2::{Digest, Sha256};

pub fn single_hash(data: String) -> BigInt {
    let data = data.as_bytes();
    let hash = Sha256::digest(data);
    BigInt::from_bytes_be(Sign::Plus, &hash)
}
pub fn double_hash(data: String) -> BigInt {
    let data = data.as_bytes();
    let hash = Sha256::digest(data);
    let hash1 = Sha256::digest(&hash);
    BigInt::from_bytes_be(Sign::Plus, &hash1)
}