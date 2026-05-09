use std::collections::VecDeque;

use crate::{script::script::Script, utils::errors::BTCErr};

#[derive(Debug)]
pub struct Satoshis(u64);

impl Satoshis {
    pub fn new(btc: f64) -> Self {
        let amt = 100_000_000.0 * btc;
        Self(amt as u64)
    }
    pub fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}
impl From<u64> for Satoshis {
    fn from(value: u64) -> Self {
        Satoshis(value)
    }
}

#[derive(Debug)]
pub struct TxOut {
    pub satoshis: Satoshis,
    pub script_pub_key: Script,
}

impl TxOut {
    pub fn new(satoshis: u64, script_pub_key: Script) -> Self {
        Self {
            satoshis: satoshis.into(),
            script_pub_key,
        }
    }
    pub fn parse(
        mut t_num_outputs: u64,
        pos: &mut usize,
        buffer: &[u8],
    ) -> Result<VecDeque<TxOut>, BTCErr> {
        let mut tx_outs = VecDeque::new();

        while t_num_outputs > 0 {
            let amt_bytes = &buffer[*pos..*pos + 8];
            // println!("Amt bytes: {:?}", amt_bytes);
            let amt = u64::from_le_bytes(amt_bytes.try_into().expect("Incorrect slice length"));
            // println!("Amt: {}", amt);
            *pos = *pos + 8;

            let sc_pk_size = buffer[*pos];
            // println!("Script pub key size: {}", sc_pk_size);
            // println!(
            //     "Script pub key: {:?}",
            //     &buffer[*pos..*pos + sc_pk_size as usize + 1]
            // );
            let script_pub_key = Script::parse(buffer[*pos..*pos + sc_pk_size as usize + 1].to_vec())?;
            // println!("{:?}", script_pub_key);
            *pos = *pos + sc_pk_size as usize + 1;
            t_num_outputs -= 1;
            tx_outs.push_back(TxOut::new(
                amt,
                Script {
                    cmds: script_pub_key,
                },
            ));
        }
        Ok(tx_outs)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, BTCErr> {
        let mut result = vec![];
        let amount_bytes = self.satoshis.0.to_le_bytes();
        result.extend(amount_bytes);
        result.extend(self.script_pub_key.serailize()?);
        Ok(result)
    }
}
