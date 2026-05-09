use std::{
    collections::VecDeque,
    io::{Cursor, Read},
};

use crate::{
    transactions::{helper::read_variant, tx_in::TxIn, tx_out::TxOut},
    utils::errors::BTCErr,
};

#[derive(Debug)]
pub struct Tx {
    pub version: u32,           // defines what additional features the transaction uses,
    pub tx_ins: VecDeque<TxIn>, // defines what bitcoins are being spent
    pub tx_outs: VecDeque<TxOut>, // defines where the bitcoins are going
    pub locktime: u32,          // defines when this transaction starts being valid
    pub testnet: bool,          // On testnet, or mainnet
}

impl Tx {
    pub fn new(
        version: u32,
        tx_ins: VecDeque<TxIn>,
        tx_outs: VecDeque<TxOut>,
        locktime: u32,
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

    pub fn parse(&self, stream: String) -> Result<Tx, BTCErr> {
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
        let (t_num_inputs, mut pos) = read_variant(pos, &buffer, out)?;

        // println!("\n-------------- Extract Inputs --------------");
        // println!("Number of transaction inputs: {}", t_num_inputs);
        // println!("current pos: {}", pos);

        let _tx_ins = TxIn::parse(t_num_inputs, &mut pos, &buffer)?;
        // println!("current pos: {}", pos);

        // Now extract transactin outputs
        // println!("\n-------------- Extract outputs --------------");
        let t_num_outputs = buffer[pos];
        // println!("Number of transaction output: {}", t_num_outputs);
        pos += 1;

        let _tx_outs = TxOut::parse(t_num_outputs as u64, &mut pos, &buffer)?;

        // println!();
        let locktime_bytes = &buffer[pos..];
        let locktime =
            u32::from_le_bytes(locktime_bytes.try_into().expect("Incorrect slice lenght"));

        // println!("\nLocktime bytes: {:?}", locktime_bytes);
        // println!("Locktime: {}", locktime);
        Ok(Tx::new(_version, _tx_ins, _tx_outs, locktime, true))
    }
}
