use num_bigint::BigInt;

use crate::{crypto::crypto_utils::int_to_little_endian, utils::errors::BTCErr};

pub fn read_variant(pos: usize, buffer: &[u8], out: u8) -> Result<(u64, usize), BTCErr> {
    match out {
        255 => {
            // if buffer.len() < pos + 9 {
            //     return Err(BTCErr::SliceErrorFromBytes(TryFromSliceError));
            // }
            let val = u64::from_le_bytes(buffer[pos + 1..pos + 9].try_into()?);
            Ok((val, pos + 9))
        }
        254 => {
            // if buffer.len() < pos + 5 {
            //     return Err(BTCErr::OutOfBounds);
            // }
            let val = u32::from_le_bytes(buffer[pos + 1..pos + 5].try_into()?) as u64;
            Ok((val, pos + 5))
        }
        253 => {
            // if buffer.len() < pos + 3 {
            //     return Err(BTCErr::OutOfBounds);
            // }
            let val = u16::from_le_bytes(buffer[pos + 1..pos + 3].try_into()?) as u64;
            Ok((val, pos + 3))
        }
        _ => {
            // if buffer.len() <= pos {
            //     return Err(BTCErr::OutOfBounds);
            // }
            Ok((buffer[pos] as u64, pos))
        }
    }
}

pub fn encode_variant(n: BigInt) -> Vec<u8> {
    if n < BigInt::from(0xfd) {
        // n.to_signed_bytes_be()
        n.to_bytes_be().1
    } else if n < BigInt::from(0x10000) {
        let mut b = b"fd".to_vec();
        b.extend_from_slice(&int_to_little_endian(&n, 2));
        b
    } else if n < BigInt::from(0x100000000 as u64) {
        let mut b = b"fe".to_vec();
        b.extend_from_slice(&int_to_little_endian(&n, 4));
        b
    } else {
        let mut b = b"ff".to_vec();
        b.extend_from_slice(&int_to_little_endian(&n, 8));
        b
    }
}
