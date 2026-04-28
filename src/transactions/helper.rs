use crate::utils::errors::BTCErr;

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
            Ok((buffer[pos] as u64, pos + 1))
        }
    }
}
