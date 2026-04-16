use num_bigint::BigInt;

use crate::crypto::hash_helper::single_hash;

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
        single_hash(self)
    }
}
impl SecretField for &str {
    fn into_bigint(self) -> BigInt {
        single_hash(self.to_string())
    }
}