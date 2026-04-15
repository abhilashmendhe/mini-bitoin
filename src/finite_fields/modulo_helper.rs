use num_bigint::BigInt;

pub fn modulo(num: BigInt, m: BigInt) -> BigInt {
    ((num % m.clone()) + m.clone()) % m.clone()
}

pub fn pow_modulo(num: BigInt, n: BigInt, m: BigInt) -> BigInt {

    let mut result = BigInt::from(1);
    let mut x = num;
    let mut n = n;

    while n.clone() > BigInt::from(0) {
        if n.clone() % BigInt::from(2) == BigInt::from(1) {
            result = ((result % m.clone()) * (x.clone() % m.clone())) % m.clone();
        }
        x = ((x.clone() % m.clone()) * (x.clone() % m.clone())) % m.clone();
        n /= 2;
    }
    result
}