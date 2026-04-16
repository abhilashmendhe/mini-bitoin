use std::ops::{Add, Mul};

use num_bigint::BigInt;
use once_cell::sync::Lazy;

use crate::{crypto::{s256_field::S256Field, signature::Signature}, elliptic_curve::{curve_field::CurveField, ecc_point::Point}, finite_fields::modulo_helper::{modulo, pow_modulo}};

pub const N: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap()
});

pub const G: Lazy<S256Point> = Lazy::new(|| {
    let gx = BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
    let gy = BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();
    let gx = S256Field::new(gx, None);
    let gy = S256Field::new(gy, None);
    S256Point::new(gx, gy, None,None)
});

#[derive(Debug, Clone)]
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

    pub fn verify(self, z: BigInt, sig: Signature, generator: S256Point) -> bool {
        let n = N;
        let s_inv = pow_modulo(sig.s.clone(), (*n).clone() - 2, (*n).clone());
        let u = z * modulo(s_inv.clone(), (*n).clone());
        let v = sig.r.clone() * modulo(s_inv.clone(), (*n).clone());
        let sum_two_points = (generator*u) + (self*v);
        if let Point::Finite { x, y:_, a:_, b:_ } = sum_two_points.inner {
            x.inner.num==sig.r
        } else {
            false
        }
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