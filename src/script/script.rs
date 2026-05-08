use num_traits::ToPrimitive;

use crate::{
    crypto::crypto_utils::little_endian_to_int, transactions::helper::read_variant,
    utils::errors::BTCErr,
};

#[derive(Debug)]
pub struct Script {
    pub cmds: Vec<u8>,
}

impl Script {
    pub fn new(cmds: Option<Vec<u8>>) -> Self {
        let n_cmds = if let Some(c) = cmds { c } else { vec![] };
        Self { cmds: n_cmds }
    }

    pub fn parse(script_pub_key: Vec<u8>) -> Result<Vec<u8>, BTCErr> {
        let mut cmds = vec![];
        let pos = 0;
        let f_pos = script_pub_key[pos];
        let (until_next, mut pos) = read_variant(pos, &script_pub_key, f_pos).unwrap();
        // pos += 1;
        // println!("next pos: {}, len: {}", next_pos, pos);
        // println!("{:?}",script_pub_key[pos..next_pos]);
        while pos < until_next as usize + 1 {
            // println!("{}",script_pub_key[pos]);
            let current_byte = script_pub_key[pos];
            if current_byte >= 1 && current_byte <= 75 {
                cmds.extend_from_slice(&script_pub_key[pos + 1..pos + current_byte as usize + 1]);
                pos += current_byte as usize + 1;
            } else if current_byte == 76 {
                pos += 1;
                let d_len = little_endian_to_int(&[script_pub_key[pos]])
                    .to_usize()
                    .unwrap();
                cmds.extend_from_slice(&script_pub_key[pos + 1..pos + d_len + 1]);
                pos += d_len + 1;
            } else if current_byte == 77 {
                pos += 1;
                let read_d = &script_pub_key[pos..pos + 1];
                let d_len = little_endian_to_int(read_d).to_usize().unwrap();
                cmds.extend_from_slice(&script_pub_key[pos + 2..pos + d_len + 1]);
                pos += d_len + 1;
            } else {
                cmds.push(current_byte);
                pos += 1;
            }
            // println!("cmds ->> {:?}, pos: {}, [pos]: {}",cmds, pos, script_pub_key[pos]);
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
}
