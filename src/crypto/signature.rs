use num_bigint::BigInt;

use crate::crypto::crypto_utils::to_32bytes_vec_big_endian;

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Self {
        Signature { r, s }
    }

    pub fn der(&self) -> Vec<u8> {
        let mut result = vec![];
        let mut sub_result = vec![];

        // for r
        let mut rb_res = vec![];

        let r = &self.r;
        let rby = to_32bytes_vec_big_endian(r);
        let rby_z_ind = if let Some(rby_z_ind) = rby.iter().rposition(|&x| x == 0) {
            rby_z_ind
        } else {
            0
        };
        let rby = rby[rby_z_ind..].to_vec();
        if rby[0] >= 128 {
            let mut rb_sub_res = vec![];
            rb_sub_res.extend([0x00]);
            rb_sub_res.extend(&rby);
            rb_res.extend([2u8, rby.len() as u8]);
            rb_res.extend(rb_sub_res);
        } else {
            rb_res.extend([2u8, rby.len() as u8]);
            rb_res.extend(&rby);
        }

        // for s
        let mut sb_res = vec![];
        let mut sb_sub_res = vec![];
        let s = &self.s;
        let sby = to_32bytes_vec_big_endian(s);
        let sby_z_ind = if let Some(sby_z_ind) = sby.iter().rposition(|&x| x == 0) {
            sby_z_ind
        } else {
            0
        };
        let sby = sby[sby_z_ind..].to_vec();
        if sby[0] >= 128 {
            sb_sub_res.extend([0x00]);
            sb_sub_res.extend(&sby);
            sb_res.extend([2u8, sby.len() as u8]);
            sb_res.extend(sb_sub_res);
        } else {
            sb_res.extend([2u8, sby.len() as u8]);
            sb_res.extend(&sby);
        }

        sub_result.extend([0x30, (rb_res.len() + sb_res.len()) as u8]);
        result.extend(sub_result);
        result.extend(rb_res);
        result.extend(sb_res);
        result
    }

    pub fn un_der(bytes: Vec<u8>) -> (BigInt, BigInt) {
        let _marker = bytes[0];
        let _tootal_sig_len = bytes[1];
        let _marker_r_val = bytes[2];

        let r_val_len = bytes[3];
        let r_val_start_ind = 4 as usize;
        let r_val_end_ind = r_val_start_ind + r_val_len as usize + 1;
        let r_val_bytes = &bytes[r_val_start_ind..r_val_end_ind];

        let _marker_s_val = r_val_end_ind;
        let s_val_len = bytes[_marker_s_val + 1];
        let s_val_start_ind = _marker_s_val + 2;
        let s_val_end_ind = s_val_start_ind + s_val_len as usize;
        let s_val_bytes = &bytes[s_val_start_ind..s_val_end_ind];

        (
            BigInt::from_bytes_be(num_bigint::Sign::Plus, r_val_bytes),
            BigInt::from_bytes_be(num_bigint::Sign::Plus, s_val_bytes),
        )
    }
}

impl std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature({:x},{:x})", self.r, self.s)
    }
}
