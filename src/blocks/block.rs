use crate::{crypto::hash_helper::hash256, utils::errors::BTCErr};

#[derive(Debug)]
pub struct Block {
    pub version: u32,
    pub previous_block: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub timestamp: u32, 
    pub bits: Vec<u8>, 
    pub nonce: Vec<u8>
}

impl Block {
    pub fn new(version: u32, previous_block: Vec<u8>, merkle_root: Vec<u8>, timestamp: u32, bits: Vec<u8>, nonce: Vec<u8>) -> Self {
        Self { version, previous_block, merkle_root, timestamp, bits, nonce }
    }

    pub fn parse(block_bytes: Vec<u8>) -> Result<Self, BTCErr> {

        // 1. read version
        let version_bytes = &block_bytes[0..4];
        let version = u32::from_le_bytes(version_bytes.try_into()?);

        // 2. read previous block bytes
        let previous_block = block_bytes[4..36].to_vec();

        // 3. read merkle root bytes
        let merkle_root = block_bytes[36..68].to_vec();

        // 4. read timestamp
        let timestamp_bytes = &block_bytes[68..72];
        let timestamp = u32::from_le_bytes(timestamp_bytes.try_into()?);

        // 5. read bits
        let bits_bytes = block_bytes[72..76].to_vec();
        // let bits = u32::from_be_bytes(bits_bytes.try_into()?);

        // 6. read nonce
        let nonce_bytes = block_bytes[76..80].to_vec();
        // let nonce = u32::from_be_bytes(nonce_bytes.try_into()?);

        Ok(Self { version, previous_block, merkle_root, timestamp, bits: bits_bytes, nonce: nonce_bytes })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];

        // 1. version
        bytes.extend(self.version.to_le_bytes());

        // 2. prev block
        bytes.extend(&self.previous_block);

        // 3. merkle root 
        bytes.extend(&self.merkle_root);

        // 4. timestamp
        bytes.extend(self.timestamp.to_le_bytes());

        // 5. bits
        bytes.extend(&self.bits);

        // 6. nonce
        bytes.extend(&self.nonce);

        bytes
    }

    pub fn hash(&self) -> String {
        let mut hash_block_bytes = hash256(&self.serialize());
        hash_block_bytes.reverse();
        hex::encode(hash_block_bytes)
    } 

    pub fn bip9(&self) -> bool {
        self.version >> 29 == 0b001
    }

    pub fn bip91(&self) -> bool {
        self.version >> 4 & 1 == 1
    }

    pub fn bip141(&self) -> bool {
        self.version >> 1 & 1 == 1
    }
}