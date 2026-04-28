use std::{
    collections::VecDeque,
    io::{Cursor, Read},
};

use crate::{
    transactions::{helper::read_variant, tx_in::TxIn, tx_out::TxOut},
    utils::errors::BTCErr,
};

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

    pub fn parse(&self, stream: String) -> Result<(), BTCErr> {
        let bytes = hex::decode(stream)?;
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

        println!("\n-------------- Extract Inputs --------------");
        println!("Number of transaction inputs: {}", t_num_inputs);
        while t_num_inputs > 0 {
            let prev_trans_id = &buffer[pos..pos + 32];
            pos = pos + 32;
            let prev_trans_ind = &buffer[pos..pos + 4];
            pos = pos + 4;
            println!("Prev transaction id: {:?}", prev_trans_id);
            println!("Prev transaction ind: {:?}", prev_trans_ind);

            let sc_size = buffer[pos];
            println!("Script size: {}", sc_size);
            pos += sc_size as usize + 1;
            println!("new pos: {}", pos);
            println!("Seq and Lock: {:?}", &buffer[pos..pos + 4]);
            pos += 4;
            t_num_inputs -= 1;
        }

        // Now extract transactin outputs
        println!("\n-------------- Extract outputs --------------");
        let mut t_num_outputs = buffer[pos];
        println!("Number of transaction output: {}", t_num_outputs);
        pos += 1;

        while t_num_outputs > 0 {
            let amt_bytes = &buffer[pos..pos + 8];
            println!("Amt bytes: {:?}", amt_bytes);
            println!(
                "Amt: {}",
                u64::from_le_bytes(amt_bytes.try_into().expect("Incorrect slice length"))
            );
            pos = pos + 8;

            let sc_pk_size = buffer[pos];
            println!("Script pub key size: {}", sc_pk_size);
            println!(
                "Script pub key: {:?}",
                &buffer[pos..pos + sc_pk_size as usize]
            );
            pos = pos + sc_pk_size as usize + 1;
            t_num_outputs -= 1;
        }
        // println!();
        let locktime_bytes = &buffer[pos..];
        let locktime =
            u32::from_le_bytes(locktime_bytes.try_into().expect("Incorrect slice lenght"));

        println!("\nLocktime bytes: {:?}", locktime_bytes);
        println!("Locktime: {}", locktime);
        Ok(())
    }
}
