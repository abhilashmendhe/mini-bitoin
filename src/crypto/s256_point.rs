use std::ops::{Add, Mul};

use num_bigint::{BigInt, Sign};
use once_cell::sync::Lazy;

use crate::{
    crypto::{
        crypto_utils::{encode_base58_checksum, to_32bytes_vec_big_endian},
        hash_helper::hash160,
        s256_field::{P, S256Field},
        signature::Signature,
    },
    elliptic_curve::{curve_field::CurveField, ecc_point::Point},
    finite_fields::modulo_helper::{modulo, pow_modulo},
};

pub const N: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
});

pub const G: Lazy<S256Point> = Lazy::new(|| {
    let gx = BigInt::parse_bytes(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let gy = BigInt::parse_bytes(
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();
    let gx = S256Field::new(gx, None);
    let gy = S256Field::new(gy, None);
    S256Point::new(gx, gy, None, None)
});

#[derive(Debug, Clone)]
pub struct S256Point {
    pub inner: Point<S256Field>,
}

impl S256Point {
    pub fn new(x: S256Field, y: S256Field, a: Option<S256Field>, b: Option<S256Field>) -> Self {
        let na = if let None = a {
            x.zero()
        } else {
            a.clone().unwrap()
        };
        let nb = if let None = b {
            x.seven()
        } else {
            b.clone().unwrap()
        };
        let inner = Point::new(x, y, na, nb);
        S256Point { inner }
    }

    pub fn rmul(self, coeff: BigInt) -> Self {
        let coeff = modulo(coeff, (*N).clone());
        S256Point {
            inner: self.inner.rmul(coeff),
        }
    }

    pub fn verify(self, z: BigInt, sig: Signature, generator: S256Point) -> bool {
        let n = N;
        let s_inv = pow_modulo(sig.s.clone(), (*n).clone() - 2, (*n).clone());
        let u = z * modulo(s_inv.clone(), (*n).clone());
        let v = sig.r.clone() * modulo(s_inv.clone(), (*n).clone());
        let sum_two_points = (generator * u) + (self * v);
        if let Point::Finite {
            x,
            y: _,
            a: _,
            b: _,
        } = sum_two_points.inner
        {
            x.inner.num == sig.r
        } else {
            false
        }
    }

    pub fn sec(self, compressed: bool) -> Vec<u8> {
        let mut serialize_data = vec![0u8; 0];
        if let Point::Finite { x, y, a: _, b: _ } = &self.inner {
            let x = &x.inner.num;
            let y = &y.inner.num;

            if compressed {
                if y % BigInt::from(2) == BigInt::from(0) {
                    serialize_data.extend(&[0x02]);
                } else {
                    serialize_data.extend(&[0x03]);
                }
                let x_32_bytes = to_32bytes_vec_big_endian(x);
                serialize_data.extend(x_32_bytes);
            } else {
                serialize_data.extend(&[0x04]);

                let x_32_bytes = to_32bytes_vec_big_endian(x);
                serialize_data.extend(x_32_bytes);

                let y_32_bytes = to_32bytes_vec_big_endian(y);
                serialize_data.extend(y_32_bytes);
            }
        }

        serialize_data
    }

    pub fn parse(sec_bin: Vec<u8>) -> S256Point {
        if sec_bin[0] == 4 {
            let x = BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..33]);
            let y = BigInt::from_bytes_be(Sign::Plus, &sec_bin[33..65]);
            S256Point::new(S256Field::new(x, None), S256Field::new(y, None), None, None)
        } else {
            let p = P;
            let is_even = sec_bin[0] == 2;
            let x = BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..]);

            // right side of the equation y^2 = x^3 + 7

            // let rhs = pow_modulo(num, n, m);
            let rhs = x.pow(3) + BigInt::from(7);
            let s256_rhs = S256Field::new(rhs, None);

            let lhs = s256_rhs.sqrt();
            let (even_b, odd_b) = if lhs.clone().num % BigInt::from(2) == BigInt::from(0) {
                let even_beta = S256Field::new(lhs.clone().num, None);
                let odd_beta = S256Field::new((*p).clone() - lhs.num, None);
                (even_beta, odd_beta)
            } else {
                let even_beta = S256Field::new((*p).clone() - lhs.clone().num, None);
                let odd_beta = S256Field::new(lhs.num, None);
                (even_beta, odd_beta)
            };

            if is_even {
                S256Point::new(S256Field::new(x, None), even_b, None, None)
            } else {
                S256Point::new(S256Field::new(x, None), odd_b, None, None)
            }
        }
    }

    pub fn hash160(self, compressed: bool) -> Vec<u8> {
        hash160(&self.sec(compressed))
    }

    pub fn address(self, compressed: bool, testnet: bool) -> String {
        let h160 = self.hash160(compressed);
        let mut ret_result: Vec<_> = Vec::<u8>::new();
        if testnet {
            ret_result.extend([0x6f]);
            ret_result.extend(&h160);
        } else {
            ret_result.extend([0x00]);
            ret_result.extend(&h160);
        }
        encode_base58_checksum(&ret_result)
    }
}

impl Mul<BigInt> for S256Point {
    type Output = S256Point;

    fn mul(self, rhs: BigInt) -> Self::Output {
        self.rmul(rhs)
    }
}

impl Add for S256Point {
    type Output = S256Point;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self.inner + rhs.inner;
        S256Point { inner }
    }
}
