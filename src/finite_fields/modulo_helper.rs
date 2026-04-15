use num_bigint::BigInt;

pub fn modulo(num: BigInt, m: BigInt) -> BigInt {
    ((num % m.clone()) + m.clone()) % m.clone()
}