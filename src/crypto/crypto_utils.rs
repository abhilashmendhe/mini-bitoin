use crate::{crypto::hash_helper::hash256, utils::errors::BTCErr};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;

// use crate::crypto::hash_helper::single_hash;
pub const BASE58_ALPHA: Lazy<Vec<char>> = Lazy::new(|| {
    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        .chars()
        .collect::<Vec<_>>()
});

pub fn to_32bytes_vec_big_endian(data: &BigInt) -> Vec<u8> {
    let (_, data_bytes) = data.to_bytes_be();
    let mut data_bytes = data_bytes;
    if data_bytes.len() > 32 {
        // truncate (keep least significant bytes)
        data_bytes = data_bytes[data_bytes.len() - 32..].to_vec();
    } else if data_bytes.len() < 32 {
        let mut padd_ext = vec![0u8; 32 - data_bytes.len()];
        padd_ext.extend(&data_bytes);
        data_bytes = padd_ext;
    }
    data_bytes
}

pub fn int_to_little_endian(n: &BigInt, length: usize) -> Vec<u8> {
    let (_, mut bytes) = n.to_bytes_le();
    bytes.resize(length, 0u8);
    bytes
}

pub fn little_endian_to_int(bytes: &[u8]) -> BigInt {
    BigInt::from_bytes_le(num_bigint::Sign::Plus, bytes)
}

pub fn encode_base58(data: &[u8]) -> String {
    let base58_alpha = BASE58_ALPHA;
    let leading_zeros = data.iter().take_while(|&&b| b == 0).count();
    let prefix = "1".repeat(leading_zeros);
    // let prefix = "".to_string();

    let mut num = BigInt::from_bytes_be(num_bigint::Sign::Plus, &data);

    let mut res = String::new();
    let base = BigInt::from(58);
    while num > BigInt::from(0) {
        let rem = (&num % &base).to_usize().unwrap();
        num = &num / &base;

        res.insert(0, base58_alpha[rem] as char);
    }
    // println!("{} - {}",prefix, res);
    format!("{}{}", prefix, res)
}

pub fn encode_base58_checksum(b: &[u8]) -> String {
    let hash = hash256(b);
    let checksum = &hash[..4];

    let mut combined = Vec::from(b);
    combined.extend_from_slice(checksum);

    encode_base58(&combined)
}

pub fn decode_base58(s: &str) -> Result<Vec<u8>, BTCErr> {
    let base58_alpha = BASE58_ALPHA;
    let mut base58_vec = vec![];
    let leading_ones = s.chars().into_iter().take_while(|b| *b == '1').count();
    let mut total_sum = 0;
    for c in s.chars() {
        total_sum *= 58;
        let ind_value = base58_alpha.iter().position(|ch| *ch == c);
        // println!("{v:?}");
        match ind_value {
            Some(iv) => {
                total_sum += iv;
            }
            None => {
                return Err(BTCErr::Baes58DecodeFailed(format!(
                    "Invalid base58 : {}",
                    s
                )));
            }
        }
    }
    // println!("{}",total_sum);
    while total_sum > 0 {
        let rem = total_sum % 256;
        total_sum /= 256;
        // print!("{} ", rem);
        base58_vec.push(rem as u8);
    }
    let leading_zeroes = vec![0; leading_ones];
    base58_vec.extend(&leading_zeroes);
    base58_vec.reverse();
    Ok(base58_vec)
}
