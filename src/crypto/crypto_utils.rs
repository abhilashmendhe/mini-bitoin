use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::crypto::hash_helper::single_hash;

pub fn to_32bytes_vec_big_endian(data: &BigInt) -> Vec<u8> {
    let (_, data_bytes) = data.to_bytes_be();
    let mut data_bytes = data_bytes;
    if data_bytes.len() < 32 {
        let mut padd_ext = vec![0u8; 32-data_bytes.len()];
        padd_ext.extend(&data_bytes);
        data_bytes = padd_ext;
    }
    data_bytes
}

pub fn base58(data: &Vec<u8>) -> String {
    let base58_alpha = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let l_zero_ind = if let Some(lzind) = data.iter().rposition(|&v| v==0) {lzind} else {0};
    let ahead_zero_count = data[..l_zero_ind].len();   
    let prefix = "1".repeat(ahead_zero_count);
    let num_big = BigInt::from_bytes_be(num_bigint::Sign::Plus,&data);
    let mut res = String::new();
    let mut num = num_big;
    while num.clone() > BigInt::from(0) {

        let rem: BigInt = &num % 58;
        let rem_usize = &rem.to_usize().unwrap();
        let quot = &num / 58;
        num = quot;

        res.insert(0, base58_alpha[*rem_usize]);
    }
    format!("{}{}",prefix,res)
}

pub fn encode_base58_checksum(b: &Vec<u8>) -> String {

    let b_hex = hex::encode(b);
    let b_sha256 = single_hash(b_hex.clone()).0.to_str_radix(16);
    let mut combine_a = String::new();
    combine_a.push_str(&b_hex);
    combine_a.push_str(&b_sha256[..4]);
    let data_b = BigInt::parse_bytes(combine_a.as_bytes(), 16).unwrap();
    let data = to_32bytes_vec_big_endian(&data_b);
    base58(&data)
}