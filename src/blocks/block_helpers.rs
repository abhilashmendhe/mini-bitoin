use num_bigint::BigInt;

use crate::{crypto::crypto_utils::little_endian_to_int, utils::errors::BTCErr};

pub fn bits_to_target(bits: &[u8]) -> BigInt {
    let exp = bits[bits.len() - 1];
    let coeff = little_endian_to_int(&bits[0..(bits.len() - 1)]);
    coeff * BigInt::from(256).pow((exp - 3) as u32)
}

pub fn target_to_bits(target: BigInt) -> Result<Vec<u8>, BTCErr> {
    let (_, raw_bytes) = target.to_bytes_be();

    let mut new_raw_bytes = vec![];
    let stip_raw_bytes = raw_bytes.strip_prefix(b"0");
    if stip_raw_bytes == None {
        return Err(BTCErr::FailedToStipPrefix("Failed to stip prefix in target_to_bits func.".to_string()));
    }

    let stip_raw_bytes = stip_raw_bytes.unwrap();
    let (exp, coeff) = if stip_raw_bytes[0] > 0x7f {
        let exp = raw_bytes.len() + 1;
        let mut coeff = vec![];
        coeff.push(0x00);
        coeff.extend(&stip_raw_bytes[..2]);
        (exp as u8, coeff)
    } else {
        let exp = raw_bytes.len();
        let mut coeff = vec![];
        coeff.extend(&stip_raw_bytes[..3]);
        (exp as u8, coeff)
    };
    new_raw_bytes.extend(coeff.iter().rev());
    new_raw_bytes.push(exp);
    Ok(new_raw_bytes)
}