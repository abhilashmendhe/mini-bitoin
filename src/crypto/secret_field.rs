use num_bigint::{BigInt, Sign};

use crate::crypto::hash_helper::hash256;

// use crate::crypto::hash_helper::single_hash;

pub trait SecretField {
    fn into_bigint(self) -> BigInt;
}
impl SecretField for BigInt {
    fn into_bigint(self) -> BigInt {
        self
    }
}
impl SecretField for String {
    fn into_bigint(self) -> BigInt {
        let hash = hash256(self.as_bytes());
        BigInt::from_bytes_be(Sign::Plus, &hash)
    }
}
impl SecretField for &str {
    fn into_bigint(self) -> BigInt {
        let hash = hash256(self.as_bytes());
        BigInt::from_bytes_be(Sign::Plus, &hash)
    }
}