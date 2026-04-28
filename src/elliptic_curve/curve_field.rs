use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;

use crate::{
    crypto::s256_field::S256Field, finite_fields::field_element::FieldElement,
    utils::errors::BTCErr,
};

pub trait CurveField:
    Clone
    + PartialEq
    + Eq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + std::fmt::Display
{
    fn checked_add(self, rhs: Self) -> Result<Self, BTCErr>;
    fn checked_sub(self, rhs: Self) -> Result<Self, BTCErr>;
    fn checked_mul(self, rhs: Self) -> Result<Self, BTCErr>;
    fn checked_div(self, rhs: Self) -> Result<Self, BTCErr>;
    fn zero(&self) -> Self;
    fn seven(&self) -> Self;
}

impl CurveField for FieldElement {
    fn checked_mul(self, rhs: Self) -> Result<Self, BTCErr> {
        self.checked_mul(rhs)
    }
    fn checked_add(self, rhs: Self) -> Result<Self, BTCErr> {
        self.checked_add(rhs)
    }
    fn checked_sub(self, rhs: Self) -> Result<Self, BTCErr> {
        self.checked_sub(rhs)
    }
    fn checked_div(self, rhs: Self) -> Result<Self, BTCErr> {
        self.checked_div(rhs)
    }
    fn zero(&self) -> Self {
        FieldElement {
            num: BigInt::parse_bytes(b"0", 16).unwrap(),
            prime: self.prime.clone(),
        }
    }
    fn seven(&self) -> Self {
        FieldElement {
            num: BigInt::from(7),
            prime: self.prime.clone(),
        }
    }
}

impl CurveField for BigInt {
    fn checked_mul(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(self * rhs)
    }
    fn checked_add(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(self + rhs)
    }
    fn checked_sub(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(self - rhs)
    }
    fn checked_div(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(self / rhs)
    }
    fn zero(&self) -> Self {
        BigInt::parse_bytes(b"0", 16).unwrap()
    }
    fn seven(&self) -> Self {
        BigInt::from(7)
    }
}

impl CurveField for S256Field {
    fn checked_add(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(S256Field {
            inner: self.inner.checked_add(rhs.inner)?,
        })
    }

    fn checked_sub(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(S256Field {
            inner: self.inner.checked_sub(rhs.inner)?,
        })
    }

    fn checked_mul(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(S256Field {
            inner: self.inner.checked_mul(rhs.inner)?,
        })
    }

    fn checked_div(self, rhs: Self) -> Result<Self, BTCErr> {
        Ok(S256Field {
            inner: self.inner.checked_div(rhs.inner)?,
        })
    }

    fn zero(&self) -> Self {
        S256Field {
            inner: self.inner.zero(),
        }
    }

    fn seven(&self) -> Self {
        S256Field {
            inner: self.inner.seven(),
        }
    }
}
