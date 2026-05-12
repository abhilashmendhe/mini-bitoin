use std::{collections::VecDeque, ops::Add, vec};

use num_bigint::BigInt;

use crate::{
    script::op_codes::OP_CODES, transactions::helper::read_variant, utils::errors::BTCErr,
};

#[derive(Debug, Clone)]
pub enum ScriptCmd {
    Op(u8),
    Data(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Script {
    pub cmds: Vec<ScriptCmd>,
}

impl Script {
    pub fn new(cmds: Option<Vec<ScriptCmd>>) -> Self {
        let n_cmds = if let Some(c) = cmds { c } else { vec![] };
        Self { cmds: n_cmds }
    }

    pub fn parse(script_pub_key: Vec<u8>) -> Result<Vec<ScriptCmd>, BTCErr> {
        // let mut cmds: Vec<dyn ScriptCmds>;
        let mut cmds = vec![];
        let pos = 0;
        let f_pos = script_pub_key[pos];
        let (until_next, mut pos) = read_variant(pos, &script_pub_key, f_pos).unwrap();
        pos += 1;
        // println!("next pos: {}, len: {}", until_next, pos);
        // println!("{:?}",script_pub_key[pos..next_pos]);
        while pos <= until_next as usize {
            // println!("pos: {}, until_next: {}", pos, until_next);
            if pos >= script_pub_key.len() {
                pos += 1;
                break;
            }
            // println!("{}",script_pub_key[pos]);
            let current_byte = script_pub_key[pos];
            pos += 1;
            // println!("pos: {}, pos+current_byte: {}", pos, pos+current_byte as usize);
            if current_byte >= 1 && current_byte <= 75 {
                cmds.push(ScriptCmd::Data(
                    script_pub_key[pos..pos + current_byte as usize].to_vec(),
                ));
                pos += current_byte as usize;
            } else if current_byte == 76 {
                pos += 1;
                let d_len = script_pub_key[pos] as usize;
                cmds.push(ScriptCmd::Data(script_pub_key[pos..pos + d_len].to_vec()));
                pos += d_len;
            } else if current_byte == 77 {
                pos += 1;
                let d_len =
                    u16::from_le_bytes([script_pub_key[pos], script_pub_key[pos + 1]]) as usize;
                cmds.push(ScriptCmd::Data(
                    script_pub_key[pos + 1..pos + d_len].to_vec(),
                ));
                pos += d_len;
            } else if current_byte == 78 {
                pos += 1;
                let d_len = u32::from_le_bytes([
                    script_pub_key[pos],
                    script_pub_key[pos + 1],
                    script_pub_key[pos + 2],
                    script_pub_key[pos + 3],
                ]) as usize;
                cmds.push(ScriptCmd::Data(
                    script_pub_key[pos + 3..pos + d_len].to_vec(),
                ));
                pos += d_len;
            } else {
                // cmds.push(current_byte);
                cmds.push(ScriptCmd::Op(current_byte));
                // pos += 1;
            }
            // println!("cmds ->> {:?}, pos: {}", cmds, pos);
            // println!("cmds ->> {:?}, pos: {}, [pos]: {}",cmds, pos, script_pub_key[pos]);
            // pos += 1;
        }
        // println!("cmds: {:?}", cmds);
        // println!("pos:{},until:{}",pos,until_next);
        if pos - 1 != until_next as usize {
            return Err(BTCErr::ScriptParseFailed(
                "Parsing script failed".to_string(),
            ));
        }
        Ok(cmds)
    }

    pub fn serailize(&self) -> Result<Vec<u8>, BTCErr> {
        let mut sub_res = vec![];
        for cmd in &self.cmds {
            match cmd {
                ScriptCmd::Op(data) => {
                    sub_res.push(*data);
                }
                ScriptCmd::Data(items) => {
                    let length = items.len();
                    if length < 76 {
                        sub_res.push(length as u8);
                    } else if length < 0x100 {
                        sub_res.push(76);
                        sub_res.push(length as u8);
                    } else if length <= 520 {
                        sub_res.push(77);
                        sub_res.extend_from_slice(&(length as u16).to_le_bytes());
                    } else {
                        return Err(BTCErr::ScriptSerializeFailed("cmd too long".to_string()));
                    }
                    sub_res.extend_from_slice(items);
                }
            }
        }
        let total = sub_res.len();
        let mut result = vec![];
        result.extend(script_encode_variant(total as u64));
        result.extend(&sub_res);

        Ok(result)
    }

    pub fn evaluate(&self, z: Option<BigInt>) -> bool {
        let mut cmds = self.cmds.clone();
        let mut stack = VecDeque::new();
        let mut altstack = VecDeque::new();

        while cmds.len() > 0 {
            let cmd = &mut cmds.remove(0);
            match cmd {
                ScriptCmd::Op(code) => {
                    let f = OP_CODES::stack_operations(
                        (*code as u16).into(),
                        &mut stack,
                        &mut altstack,
                        z.clone(),
                    );
                    if !f {
                        eprintln!("Bad op: {}", *code);
                        return false;
                    }
                }
                ScriptCmd::Data(items) => stack.push_back(items.clone()),
            }
        }
        if stack.len() == 0 {
            return false;
        }
        if let Some(items) = stack.pop_front() {
            if items == b"" {
                return false;
            }
        }
        println!("final stack: {:?}", stack);
        true
    }
}

pub fn script_encode_variant(n: u64) -> Vec<u8> {
    if n < 0xfd {
        vec![n as u8]
    } else if n <= 0xffff {
        let mut out = vec![0xfd];
        out.extend_from_slice(&(n as u16).to_le_bytes());
        out
    } else if n <= 0xffff_ffff {
        let mut out = vec![0xfe];
        out.extend_from_slice(&(n as u32).to_le_bytes());
        out
    } else {
        let mut out = vec![0xff];
        out.extend_from_slice(&n.to_le_bytes());
        out
    }
}

impl Add for Script {
    type Output = Script;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.cmds.append(&mut rhs.cmds);
        self
    }
}
