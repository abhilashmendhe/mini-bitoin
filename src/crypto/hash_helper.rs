use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub fn sha256(data: &[u8]) -> Vec<u8> {
    Sha256::digest(data).to_vec()
}

pub fn hash256(data: &[u8]) -> Vec<u8> {
    Sha256::digest(&Sha256::digest(data)).to_vec()
}

pub fn hash160(data: &[u8]) -> Vec<u8> {
    let sha256_hash = Sha256::digest(data);
    let ripemd160_hash = Ripemd160::digest(sha256_hash);
    ripemd160_hash.to_vec()
}
