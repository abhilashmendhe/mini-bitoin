/*
    List of APIs to fetch transaction details.
    API docs link https://github.com/Blockstream/esplora/blob/master/API.md

    mainnet
        1. Live explorer - https://blockstream.info/
        2. API           - https://blockstream.info/api/tx/<TXID>
         e.g. curl https://blockstream.info/api/tx/3e3ff224004623a84234b51ef20cda5661bb50f1ca891cf3803999431a2a0cf7
    testnet -
        1. Live explorer - https://blockstream.info/testnet/
        2. API           - https://blockstream.info/testnet/api/tx/<TXID>
         e.g. curl https://blockstream.info/testnet/api/tx/34591e0ab8cf5a0d4bd4f00cf827daf3875179daf97aba21d5e5a04947716265
*/

use std::collections::HashMap;

use crate::{transactions::tx::Tx, utils::errors::BTCErr};

const BLOCKSTREAM_MAINNET: &'static str = "https://blockstream.info/api";
const BLOCKSTREAM_TESTNET: &'static str = "https://blockstream.info/testnet/api";

#[derive(Debug)]
pub struct TxFetcher {
    cache: HashMap<String, (String, Tx)>,
}

impl TxFetcher {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    pub fn fetch(&self, tx_id: String, testnet: bool, fresh: bool) -> Result<(), BTCErr> {
        if fresh || !self.cache.contains_key(&tx_id) {
            let url = format!("{}/tx/{}/hex", get_url(testnet), tx_id);
            let response = reqwest::blocking::get(url).unwrap().text().unwrap();
            
            let tx = Tx::parse(response)?;
            println!("{:?}",tx);
            // println!("{:?}", response);
        }
        Ok(())
    }
}

pub fn get_url(testnet: bool) -> String {
    if testnet {
        BLOCKSTREAM_TESTNET.to_string()
    } else {
        BLOCKSTREAM_MAINNET.to_string()
    }
}
