use std::ops::{Add, Div, Mul, Sub};

use crate::{finite_fields::field_element::FieldElement, utils::errors::BTCErr};

pub trait CurveField: 
    Copy
    + Clone
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
        FieldElement { num: 0, prime: self.prime }
    }
}
impl CurveField for isize {
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
        0
    }
    
    
}