use num_bigint::BigInt;

pub fn to_32bytes_vec_big_endian(data: &BigInt) -> Vec<u8> {
    let (_, data_bytes) = data.to_bytes_be();
    let mut data_bytes = data_bytes;
    if data_bytes.len() < 32 {
        let mut padd_ext = vec![0u8; 32-data_bytes.len()];
        padd_ext.extend(&data_bytes);
        data_bytes = padd_ext;
    }
    data_bytes
}