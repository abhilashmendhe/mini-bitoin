pub fn encode_num(num: i64) -> Vec<u8> {
    if num == 0 {
        return vec![];
    }
    let mut abs_num = num.abs();
    let neg = if num < 0 { true } else { false };
    let mut result = vec![];
    while abs_num > 0 {
        result.push((abs_num & 0xff) as u8);
        abs_num >>= 8;
    }
    let r_len = result.len();
    if result[r_len - 1] & 0x80 != 0 {
        if neg {
            result.push(0x80);
        } else {
            result.push(0);
        }
    } else if neg {
        result[r_len - 1] |= 0x80;
    }
    result
}

pub fn decode_num(mut element: Vec<u8>) -> i64 {
    if element.len() == 0 {
        return 0;
    }
    element.reverse();

    let mut neg = true;
    let mut result: i64 = 0;
    if element[0] & 0x80 == 0 {
        neg = false;
        result = element[0] as i64;
    } else {
        result = element[0] as i64 & 0x7f;
    }
    for c in &mut element[1..] {
        result <<= 8;
        result += *c as i64;
    }
    if neg { -result } else { result }
}
