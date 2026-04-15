use std::ops::{Add, Mul};

use num_bigint::BigInt;
use once_cell::sync::Lazy;

use crate::{crypto::s256_field::S256Field, elliptic_curve::{curve_field::CurveField, ecc_point::Point}, finite_fields::modulo_helper::modulo};

pub const N: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap()
});

#[derive(Debug)]
pub struct S256Point {
    pub inner: Point<S256Field>,
}

impl S256Point {
    pub fn new(x: S256Field, y: S256Field, a: Option<S256Field>, b: Option<S256Field>) -> Self {
        let na = if let None = a { x.zero() } else { a.clone().unwrap() };
        let nb = if let None = b { x.seven() } else { b.clone().unwrap() };
        let inner = Point::new(x, y, na, nb);
        S256Point { inner }
    }

    pub fn rmul(self, coeff: BigInt) -> Self {
        let coeff = modulo(coeff, (*N).clone());
        S256Point { inner: self.inner.rmul(coeff) }
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