use std::collections::VecDeque;

use num_bigint::BigInt;

use crate::{
    crypto::{
        hash_helper::{hash160, hash256},
        s256_point::{G, S256Point},
        signature::Signature,
    },
    script::helper::encode_num,
};

pub fn op_dup(stack: &mut VecDeque<Vec<u8>>) -> bool {
    let s_len = stack.len();
    if s_len < 1 {
        return false;
    }
    stack.push_back(stack[s_len - 1].clone());
    true
}

pub fn op_hash256(stack: &mut VecDeque<Vec<u8>>) -> bool {
    let s_len = stack.len();
    if s_len < 1 {
        return false;
    }
    if let Some(s_elem) = stack.pop_back() {
        stack.push_back(hash256(&s_elem));
    }
    true
}

pub fn op_hash160(stack: &mut VecDeque<Vec<u8>>) -> bool {
    let s_len = stack.len();
    if s_len < 1 {
        return false;
    }
    if let Some(s_elem) = stack.pop_back() {
        stack.push_back(hash160(&s_elem));
    }
    true
}

pub fn op_nop(stack: &mut VecDeque<Vec<u8>>) -> bool {
    return true;
}

pub fn op_toaltstack(stack: &mut VecDeque<Vec<u8>>, altstack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    if let Some(item) = stack.pop_back() {
        altstack.push_back(item);
    }
    true
}

pub fn op_fromaltstack(stack: &mut VecDeque<Vec<u8>>, altstack: &mut VecDeque<Vec<u8>>) -> bool {
    if altstack.len() < 1 {
        return false;
    }
    stack.push_back(altstack.pop_back().unwrap());
    true
}

pub fn op_checksig(stack: &mut VecDeque<Vec<u8>>, z: BigInt) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let sec_pubkey = stack.pop_back().unwrap();
    let signature = stack.pop_back().unwrap();
    let der_signature = signature[..(signature.len() - 1)].to_vec();

    let s256_point = S256Point::parse(sec_pubkey);
    // println!("s256_point: {:?}", s256_point);
    let signature = Signature::parse(der_signature);

    let g = &(*G);
    let f = s256_point.verify(z, signature, g.clone());

    if f {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}
