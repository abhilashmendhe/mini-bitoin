use std::collections::VecDeque;

use num_bigint::BigInt;

use crate::{
    crypto::{
        hash_helper::{hash160, hash256},
        s256_point::{G, S256Point},
        signature::Signature,
    },
    script::helper::{decode_num, encode_num},
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

pub fn op_nop(_stack: &mut VecDeque<Vec<u8>>) -> bool {
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

pub fn op_drop(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    stack.pop_back();
    true
}

pub fn op_2drop(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    stack.pop_back();
    stack.pop_back();
    true
}

pub fn op_nip(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let top1 = stack.pop_back().unwrap();
    let _top2 = stack.pop_back();
    stack.push_back(top1);
    true
}

pub fn op_over(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let s = stack[stack.len() - 2].clone();
    stack.push_back(s);
    true
}

pub fn op_swap(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let top1 = stack.pop_back().unwrap();
    let top2 = stack.pop_back().unwrap();
    stack.push_back(top1);
    stack.push_back(top2);
    true
}

pub fn op_tuck(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let top1 = stack[stack.len() - 1].clone();
    stack.insert(stack.len() - 3, top1);
    true
}

pub fn op_2up(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    stack.push_back(stack[stack.len() - 2].clone());
    stack.push_back(stack[stack.len() - 1].clone());
    true
}

pub fn op_3up(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 3 {
        return false;
    }
    stack.push_back(stack[stack.len() - 3].clone());
    stack.push_back(stack[stack.len() - 2].clone());
    stack.push_back(stack[stack.len() - 1].clone());
    true
}

pub fn op_verify(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let elem = stack.pop_back().unwrap();
    if decode_num(elem) == 0 { false } else { true }
}

pub fn op_equal(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let t1 = stack.pop_back().unwrap();
    let t2 = stack.pop_back().unwrap();
    if t1 == t2 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

pub fn op_equalverify(stack: &mut VecDeque<Vec<u8>>) -> bool {
    op_equal(stack) && op_verify(stack)
}

pub fn op_1add(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let elem = stack.pop_back().unwrap();
    let sum = decode_num(elem) + 1;
    stack.push_back(encode_num(sum));
    true
}

pub fn op_1sub(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let elem = stack.pop_back().unwrap();
    let sum = decode_num(elem) - 1;
    stack.push_back(encode_num(sum));
    true
}

pub fn op_abs(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let elem = decode_num(stack.pop_back().unwrap());
    if elem < 0 {
        stack.push_back(encode_num(-1 * elem));
    } else {
        stack.push_back(encode_num(elem));
    }
    true
}

pub fn op_not(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let elem = decode_num(stack.pop_back().unwrap());
    if elem == 0 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

pub fn op_0notequal(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let elem = decode_num(stack.pop_back().unwrap());
    if elem == 0 {
        stack.push_back(encode_num(0));
    } else {
        stack.push_back(encode_num(1));
    }
    true
}

pub fn op_add(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    stack.push_back(encode_num(elem1 + elem2));
    true
}

pub fn op_sub(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    stack.push_back(encode_num(elem2 - elem1));
    true
}

pub fn op_booland(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    if elem1 > 0 && elem2 > 0 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

pub fn op_boolor(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    if elem1 > 0 || elem2 > 0 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

pub fn op_numequal(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    if elem1 == elem2 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

pub fn op_numequalverify(stack: &mut VecDeque<Vec<u8>>) -> bool {
    op_numequal(stack) && op_verify(stack)
}

pub fn op_numnotequal(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    if elem1 != elem2 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

pub fn op_lessthan(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    if elem2 < elem1 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

pub fn op_greaterthan(stack: &mut VecDeque<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let elem1 = decode_num(stack.pop_back().unwrap());
    let elem2 = decode_num(stack.pop_back().unwrap());
    if elem1 < elem2 {
        stack.push_back(encode_num(1));
    } else {
        stack.push_back(encode_num(0));
    }
    true
}

