use std::ops::{Add, Mul, Sub};

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

    pub fn pow_modulo(&self, n: isize) -> FieldElement {
        
        let mut result = 1;
        let mut x = self.num;
        let mut n = n;
        let m = self.prime;
        while n > 0 {
            if n % 2 == 1 {
                result = ((result % m) * (x % m)) % m;
            }
            x = ((x % m) * (x % m)) % m;
            n /= 2;
        }
        FieldElement { num: result, prime: self.prime }
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

impl Mul for FieldElement {
    type Output = Result<FieldElement, BTCErr>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Multiplication".to_string()));
        }
        
        let num = modulo(modulo(self.num, self.prime) * modulo(rhs.num, rhs.prime), self.prime); 

        Ok(FieldElement { num, prime: self.prime })
    }
}