use std::fmt::Display;

use crate::{crypto::hash_helper::hash256, utils::errors::BTCErr};

#[derive(Debug)]
pub struct NetworkEnvelope {
    pub network_magic: Vec<u8>,
    pub command: Vec<u8>,
    pub payload_length: u32, // little-endian
    pub payload_checksum: Vec<u8>,
    pub payload: Vec<u8>,
}

impl NetworkEnvelope {
    pub fn new(
        command: Vec<u8>,
        payload_length: u32,
        payload_checksum: Vec<u8>,
        payload: Vec<u8>,
        testnet: bool,
    ) -> NetworkEnvelope {
        let network_magic = if testnet {
            b"\xf9\xbe\xb4\xd9".to_vec()
        } else {
            b"\x0b\x11\x09\x07".to_vec()
        };
        NetworkEnvelope {
            network_magic,
            command,
            payload_length,
            payload_checksum,
            payload,
        }
    }

    pub fn parse(b: Vec<u8>, testnet: bool) -> Result<NetworkEnvelope, BTCErr> {
        let network_magic = b[0..4].to_vec();
        if network_magic.len() == 0 {
            return Err(BTCErr::ConnReset("Connection reset".to_string()));
        }
        let exp_network_magic = if testnet {
            b"\xf9\xbe\xb4\xd9".to_vec()
        } else {
            b"\x0b\x11\x09\x07".to_vec()
        };
        if exp_network_magic != network_magic {
            return Err(BTCErr::IncorrectMagicNumber(format!(
                "Incorrect magic number: {} vs {}",
                hex::encode(&network_magic),
                hex::encode(&exp_network_magic)
            )));
        }
        let command = b[4..16].to_vec();
        // println!("payload len in byte: {:?}", &b[16..20]);
        let payload_length = u32::from_le_bytes(b[16..20].try_into().unwrap());

        // println!("Payload len: {}", payload_length);
        let payload_checksum = b[20..24].to_vec();
        let payload = b[24..].to_vec();
        let calc_checksum = &hash256(&payload)[..4];
        if calc_checksum != payload_checksum {
            return Err(BTCErr::NetworkChecksumFailed("Checksum does not match".to_string()));
        }
        Ok(NetworkEnvelope {
            network_magic,
            command,
            payload_length,
            payload_checksum,
            payload,
        })
    }
}

impl Display for NetworkEnvelope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            String::from_utf8_lossy(&self.command).to_string(),
            hex::encode(&self.payload)
        )
    }
}
