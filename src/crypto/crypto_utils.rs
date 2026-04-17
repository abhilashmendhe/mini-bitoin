use num_bigint::BigInt;
use num_traits::ToPrimitive;
use crate::crypto::hash_helper::hash256;

// use crate::crypto::hash_helper::single_hash;

pub fn to_32bytes_vec_big_endian(data: &BigInt) -> Vec<u8> {
    let (_, data_bytes) = data.to_bytes_be();
    let mut data_bytes = data_bytes;
    if data_bytes.len() > 32 {
        // truncate (keep least significant bytes)
        data_bytes = data_bytes[data_bytes.len() - 32..].to_vec();
    } else if data_bytes.len() < 32 {
        let mut padd_ext = vec![0u8; 32-data_bytes.len()];
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

pub fn base58(data: &[u8]) -> String {
    let base58_alpha = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    
    let leading_zeros = data.iter().take_while(|&&b| b == 0).count();
    let prefix = "1".repeat(leading_zeros);
    // let prefix = "".to_string();
    
    let mut num = BigInt::from_bytes_be(num_bigint::Sign::Plus,&data);
    
    let mut res = String::new();
    let base = BigInt::from(58);
    while num > BigInt::from(0) {

        let rem = (&num % &base).to_usize().unwrap();
        num = &num / &base;

        res.insert(0, base58_alpha[rem] as char);
    }
    // println!("{} - {}",prefix, res);
    format!("{}{}",prefix,res)
}

pub fn encode_base58_checksum(b: &[u8]) -> String {

    let hash = hash256(b);
    let checksum = &hash[..4];
   
    let mut combined = Vec::from(b);
    combined.extend_from_slice(checksum);

    base58(&combined)
}