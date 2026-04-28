use std::{
    collections::VecDeque,
    io::{Cursor, Read, Seek, SeekFrom},
};

use crate::{
    transactions::{tx_in::TxIn, tx_out::TxOut},
    utils::errors::BTCErr,
};

pub struct Tx {
    pub version: usize, // defines what additional features the transaction uses,
    pub tx_ins: VecDeque<TxIn>, // defines what bitcoins are being spent
    pub tx_outs: VecDeque<TxOut>, // defines where the bitcoins are going
    pub locktime: usize, // defines when this transaction starts being valid
    pub testnet: bool,  // On testnet, or mainnet
}

impl Tx {
    pub fn new(
        version: usize,
        tx_ins: VecDeque<TxIn>,
        tx_outs: VecDeque<TxOut>,
        locktime: usize,
        testnet: bool,
    ) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
        }
    }

    pub fn parse(&self, stream: String) -> Result<(), BTCErr> {
        let bytes = hex::decode(stream)?;
        let mut cursor = Cursor::new(bytes);

        let mut buffer = Vec::new();
        let tx_obj_size = cursor.read_to_end(&mut buffer)?;

        let version = u32::from_le_bytes(buffer[..4].try_into().expect("slice with incorrect length"));
        let pos = 4;
        let out = buffer[pos];
        let (num_inputs, pos) = match out {
            255 => {
                (u64::from_le_bytes(buffer[pos+1..pos+1+8].try_into().expect("Incorrect slice length")), pos + 1 + 8 as usize)
            },
            254 => {
                (u32::from_le_bytes(buffer[pos+1..pos+1+4].try_into().expect("Incorrect slice length")) as u64, pos + 1 + 4 as usize)
            },
            253 => {
                (u16::from_le_bytes(buffer[pos+1..pos+1+2].try_into().expect("Incorrect slice length")) as u64, pos + 1 + 2 as usize)
            },
            _ => {
                (u8::from_le(buffer[pos].try_into().expect("Incorrect slice length")) as u64, pos + 1 as usize)
            }
        };
        // println!("Number of inputs: {}", num_inputs);
        // println!("next pos: {}",pos);
        // println!();
        Ok(())
    }
}
