use std::{
    collections::VecDeque,
    io::{Cursor, Read},
};

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{
    crypto::hash_helper::hash256, script::script::script_encode_variant, transactions::{helper::read_variant, tx_fetcher::get_url, tx_in::TxIn, tx_out::TxOut}, utils::errors::BTCErr
};

#[derive(Debug)]
pub struct Tx {
    pub version: u32,           // defines what additional features the transaction uses,
    pub segwit: bool,           // legacy or segwit
    pub tx_ins: VecDeque<TxIn>, // defines what bitcoins are being spent
    pub tx_outs: VecDeque<TxOut>, // defines where the bitcoins are going
    pub locktime: u32,          // defines when this transaction starts being valid
    pub testnet: bool,          // On testnet, or mainnet
}

impl Tx {
    pub fn new(
        version: u32,
        segwit: bool,
        tx_ins: VecDeque<TxIn>,
        tx_outs: VecDeque<TxOut>,
        locktime: u32,
        testnet: bool,
    ) -> Self {
        Self {
            version,
            segwit,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
        }
    }
    
    pub fn hash(&self) -> Result<Vec<u8>, BTCErr> {
        let mut h_ser = hash256(&self.serailize()?);
        h_ser.reverse();
        Ok(h_ser)
    }

    pub fn id(&self) -> Result<String, BTCErr> {
        Ok(hex::encode(&self.hash()?))
    }

    pub fn parse(stream: String) -> Result<Tx, BTCErr> {
        let bytes = hex::decode(stream)?;
        // println!("Whole bytes size: {}", bytes.len());
        // println!("Whole bytes: {:?}\n", bytes);
        // println!("");

        let mut cursor = Cursor::new(bytes);

        let mut buffer = Vec::new();
        let _tx_obj_size = cursor.read_to_end(&mut buffer)?;
        // println!("Total byte size: {}", tx_obj_size);
        // println!("Total obj: {:?}",buffer);
        // println!("\n");
        let _version = u32::from_le_bytes(buffer[..4].try_into()?);
        let pos = 4;
        let out = buffer[pos];
        let (mut t_num_inputs, mut pos) = read_variant(pos, &buffer, out)?;
        // println!("pos after read_variant: {}",pos);

        // ---------------------- Check for sigwit ----------------------
        let segwit = if &buffer[pos..pos + 2] == &[0, 1] {
            pos += 2;
            t_num_inputs = buffer[pos] as u64;
            true
        } else {
            pos += 1;
            false
        };
        // println!("segwit: {}",segwit);
        // println!("pos: {}, buffer[{pos}]: {}, t_num_inputs: {t_num_inputs}",pos,buffer[pos]);

        // println!("\n-------------- Extract Inputs --------------");
        // println!("Number of transaction inputs: {}", t_num_inputs);
        // println!("current pos: {}", pos);

        let _tx_ins = TxIn::parse(t_num_inputs, &mut pos, &buffer)?;
        if segwit {
            pos = pos + 1;
        }
        // println!("current pos: {}", pos);

        // Now extract transactin outputs
        // println!("\n-------------- Extract outputs --------------");
        let t_num_outputs = buffer[pos];
        // println!("Number of transaction output: {}", t_num_outputs);
        pos += 1;

        let _tx_outs = TxOut::parse(t_num_outputs as u64, &mut pos, &buffer)?;
        // println!("\n{:?}\n",_tx_outs);
        // println!("\n\nAfter parsing outputs: {:?}\n\n", &buffer[pos..]);
        // println!();
        // -------------------------- parse witness ---------------------------
        if segwit {
            let mut i = 0;
            while i < t_num_inputs {
                // println!("Num witness inp:{}", &buffer[pos]);
                let sig_len = buffer[pos + 1] as usize;
                let _signature = &buffer[pos + 2..pos + sig_len + 2];
                // println!("{:?}", &buffer[pos + 2..pos + sig_len + 2]);
                pos += sig_len + 2;
                let pubkey_len = buffer[pos] as usize;
                // println!("pubkey len: {}", pubkey_len);
                // println!("{:?}", &buffer[pos + 1..pos + pubkey_len + 1]);
                let _pubkey = &buffer[pos + 1..pos + pubkey_len + 1];
                pos += pubkey_len + 1;
                i += 1;
            }
        }

        // -------------------------- parse locktime ---------------------------

        let locktime_bytes = &buffer[pos..];
        let locktime =
            u32::from_le_bytes(locktime_bytes.try_into().expect("Incorrect slice lenght"));

        // println!("\nLocktime bytes: {:?}", locktime_bytes);
        // println!("Locktime: {}", locktime);
        Ok(Tx::new(_version, segwit, _tx_ins, _tx_outs, locktime, true))
    }

    pub fn serailize(&self) -> Result<Vec<u8>, BTCErr> {
        let mut result = vec![];
        result.extend(self.version.to_le_bytes());
        result.extend(script_encode_variant(self.tx_ins.len() as u64));
        for tx_in in &self.tx_ins {
            result.extend(tx_in.serialize()?);
        }

        result.extend(script_encode_variant(self.tx_outs.len() as u64));
        for tx_out in &self.tx_outs {
            result.extend(tx_out.serialize()?);
        }

        result.extend(self.locktime.to_le_bytes());
        Ok(result)
    }

    pub fn fee(&self, tx_id: String, testnet: bool) -> Result<u64, BTCErr> {
        let url = format!("{}/tx/{}", get_url(testnet), tx_id);

        let client = Client::new();
        let response: Transaction = client.get(url).send()?.json()?;
        // println!("{:?}", response);
        Ok(response.fee)
    }
}

// #[derive(Debug, Deserialize)]
// struct TxStatus {
//     confirmed: bool,
//     block_height: Option<u64>,
// }

#[derive(Debug, Deserialize)]
struct Transaction {
    #[allow(unused)]
    txid: String,
    fee: u64,
    // status: TxStatus,
}
