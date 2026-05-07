use crate::crypto::hash_helper::{hash160, hash256};

pub fn op_dup(stack: &mut Vec<Vec<u8>>) -> bool {
    let s_len = stack.len();
    if s_len < 1 {
        return false;
    }
    stack.push(stack[s_len - 1].clone());
    true
}

pub fn op_hash256(stack: &mut Vec<Vec<u8>>) -> bool {
    let s_len = stack.len();
    if s_len < 1 {
        return false;
    }
    if let Some(s_elem) = stack.pop() {
        stack.push(hash256(&s_elem));
    }
    true
}

pub fn op_hash160(stack: &mut Vec<Vec<u8>>) -> bool {
    let s_len = stack.len();
    if s_len < 1 {
        return false;
    }
    if let Some(s_elem) = stack.pop() {
        stack.push(hash160(&s_elem));
    }
    true
}
