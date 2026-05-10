use std::collections::VecDeque;

use crate::{
    script::script::Script,
    transactions::{tx::Tx, tx_fetcher::TxFetcher},
    utils::errors::BTCErr,
};

#[derive(Debug, Clone)]
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
            let mut prev_trans_id_bytes = buffer[*pos..*pos + 32].to_vec();
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
            prev_trans_id_bytes.reverse();
            tx_ins.push_back(TxIn::new(
                prev_trans_id_bytes.to_vec(),
                prev_trans_index as usize,
                Some(Script { cmds: script_sig }),
                sequence as u64,
            ));
        }
        Ok(tx_ins)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, BTCErr> {
        let mut result = vec![];
        let mut prev_tx = self.prev_tx.clone();
        prev_tx.reverse();
        result.extend(prev_tx);
        result.extend((self.prev_ind as u32).to_le_bytes());
        result.extend(self.script_sig.serailize()?);
        result.extend((self.sequence as u32).to_le_bytes());
        Ok(result)
    }

    pub fn fetch_tx(&self, testnet: bool) -> Result<Tx, BTCErr> {
        let prev_tx = hex::encode(&self.prev_tx);
        // println!("{}", prev_tx);
        let mut tx_fetcher = TxFetcher::new();
        let tx = tx_fetcher.fetch(prev_tx, testnet, true)?;
        Ok(tx)
    }
    pub fn script_pubkey(&self, testnet: bool) -> Result<Script, BTCErr> {
        let tx = &self.fetch_tx(testnet)?;
        let tx_out = &tx.tx_outs[self.prev_ind];
        Ok(tx_out.script_pub_key.clone())
    }
    pub fn value(&self, testnet: bool) -> Result<u64, BTCErr> {
        let tx = &self.fetch_tx(testnet)?;
        Ok(tx.tx_outs[self.prev_ind].satoshis.amount())
    }
}
