use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;
use once_cell::sync::Lazy;

use crate::finite_fields::field_element::FieldElement;

pub const P: Lazy<BigInt> = Lazy::new(|| {
    BigInt::from(2).pow(256) - BigInt::from(2).pow(32) - BigInt::from(977)
});

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct S256Field {
    pub inner: FieldElement
}

impl S256Field {
    pub fn new(num: BigInt, prime: Option<BigInt>) -> Self {
        let new_prime = if let None = prime {
            P.clone()
        } else {
            prime.unwrap()
        };
        let inner = FieldElement::new(num, new_prime);
        S256Field { inner }
    }
}

impl std::fmt::Display for S256Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = 64;
        write!(f, "{:0width$x}", self.inner.num, width = width)
    }
}

impl Add for S256Field {
    type Output = S256Field;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.inner + rhs.inner;
        S256Field::new(r.num, Some(r.prime))
    }
}

impl Sub for S256Field {
    type Output = S256Field;

    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.inner - rhs.inner;
        S256Field::new(r.num, Some(r.prime))
    }
}

impl Mul for S256Field {
    type Output = S256Field;

    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.inner * rhs.inner;
        S256Field::new(r.num, Some(r.prime))
    }
}

impl Div for S256Field {
    type Output = S256Field;

    fn div(self, rhs: Self) -> Self::Output {
        let r = self.inner / rhs.inner;
        S256Field::new(r.num, Some(r.prime))
    }
}