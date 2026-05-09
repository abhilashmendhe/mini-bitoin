use std::collections::VecDeque;

use crate::{script::script::Script, utils::errors::BTCErr};

#[derive(Debug)]
pub struct TxIn {
    pub prev_tx: Vec<u8>,
    pub prev_ind: usize,
    pub script_sig: Script,
    pub sequence: u64,
}

impl TxIn {
    pub fn new(
        prev_tx: Vec<u8>,
        prev_ind: usize,
        script_sig: Option<Script>,
        sequence: u64,
    ) -> Self {
        let script_sig = if let Some(sc) = script_sig {
            sc
        } else {
            Script::new(None)
        };
        Self {
            prev_tx,
            prev_ind,
            script_sig,
            sequence,
        }
    }

    pub fn parse(
        mut t_num_inputs: u64,
        pos: &mut usize,
        buffer: &[u8],
    ) -> Result<VecDeque<TxIn>, BTCErr> {
        let mut tx_ins = VecDeque::new();
        while t_num_inputs > 0 {
            let prev_trans_id_bytes = &buffer[*pos..*pos + 32];
            *pos = *pos + 32;
            let prev_trans_ind_bytes = &buffer[*pos..*pos + 4];
            let prev_trans_index = u32::from_le_bytes(
                prev_trans_ind_bytes
                    .try_into()
                    .expect("Incorrect bytes size for u32 trans index in TxIn parse"),
            );
            *pos = *pos + 4;
            // println!("Prev transaction id: {:?}", prev_trans_id_bytes);
            // println!("Prev transaction ind: {:?}", prev_trans_ind_bytes);

            let sc_size = buffer[*pos];
            // println!("current pos: {}, Script size: {}", pos, sc_size);
            let script_sig_bytes = &buffer[*pos..*pos + sc_size as usize + 1];
            // println!("le: {}, script_sig_bytes: {:?}", script_sig_bytes.len(), script_sig_bytes);
            let script_sig = Script::parse(script_sig_bytes.to_vec())?;
            // println!("script_sig: {:?}", script_sig);

            *pos += sc_size as usize + 1;
            // println!("new pos: {}", pos);
            let seq_bytes = &buffer[*pos..*pos + 4];
            let sequence = u32::from_le_bytes(
                seq_bytes
                    .try_into()
                    .expect("Incorrect bytes size for u32 sequeunce in TxIn parse"),
            );
            // println!("Seq and Lock: {:?}", );/
            *pos += 4;
            t_num_inputs -= 1;
            tx_ins.push_back(TxIn::new(
                prev_trans_id_bytes.to_vec(),
                prev_trans_index as usize,
                Some(Script { cmds: script_sig }),
                sequence as u64,
            ));
        }
        Ok(tx_ins)
    }
}
