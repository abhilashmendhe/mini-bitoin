use std::ops::{Add, Div, Mul, Sub};

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

    pub fn pow_modulo(&self, n: isize) -> Result<FieldElement, BTCErr> {
        
        if n < 0 {
            let f1 = FieldElement::new(1, self.prime)?;
            let f2 = FieldElement::new(self.num, self.prime)?.pow_modulo(-1*n)?;
            return f1 / f2;
        }

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
        Ok(FieldElement { num: result, prime: self.prime })
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

/*
    For division, we know that a/b. It can transform to the inverse multiplication i.e; a*b^-1.
    Since we are taking modulo over the div result, we will use Fermat theorem.
    The theorem says that:
                n^(p-1) % p= 1
    Because div is inverse multiplication, we can reduce to multiplication problem.
                a / b = a * b^-1
    From fermat theorem,
                b ^ (p-1) = 1 
    Multiply above equation by b^-1 gives,
                b^-1 = b^(p-2)
    For e.g. F19 (p=19) is b^-1 = b^17
    Now we compute the modulo exponential value of b^17
    The final answer of above modulo exponetial value is equal to b^-1
*/

impl Div for FieldElement {
    type Output = Result<FieldElement, BTCErr>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Division".to_string()));
        }
        let rhs_mul_inv = rhs.pow_modulo(rhs.prime - 2)?;
        self * rhs_mul_inv
    }
}