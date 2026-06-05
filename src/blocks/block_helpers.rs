use num_bigint::BigInt;

use crate::crypto::crypto_utils::little_endian_to_int;

pub fn bits_to_target(bits: &[u8]) -> BigInt {
    let exp = bits[bits.len()-1];
    let coeff = little_endian_to_int(&bits[0..(bits.len()-1)]);
    coeff * BigInt::from(256).pow((exp - 3) as u32)
}