use std::ops::{Add, Sub};

use crate::{finite_fields::modulo_helper::modulo, utils::errors::BTCErr};

#[derive(PartialEq, PartialOrd, Eq, Clone)]
pub struct FieldElement {
    pub num: isize,
    pub prime: isize
}

impl FieldElement {
    pub fn new(num: isize, prime: isize) -> Result<FieldElement, BTCErr> {

        if num >= prime || num < 0{
            return Err(BTCErr::FiniteFieldRangeErr { num, prime })
        }
        Ok(Self {
            num,
            prime, 
        })
    }
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl Add for FieldElement {
    type Output = Result<FieldElement, BTCErr>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Addition".to_string()));
        }
        let num = modulo(modulo(self.num, self.prime) + modulo(rhs.num, rhs.prime), self.prime);
        Ok(
            FieldElement { 
                num,
                prime: self.prime
            }
        )
    }
}

impl Sub for FieldElement {
    type Output = Result<FieldElement, BTCErr>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Subtraction".to_string()));
        }
        let num = modulo(
            modulo(self.num, self.prime) - modulo(rhs.num, rhs.prime) + self.prime, 
            self.prime
        );
        Ok(
            FieldElement { 
                num,
                prime: self.prime
            }
        )
    }
}