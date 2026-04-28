use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;

use crate::{finite_fields::modulo_helper::modulo, utils::errors::BTCErr};

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> Self {
        assert!(prime > BigInt::parse_bytes(b"1", 16).unwrap());
        assert!(num < prime || num >= BigInt::parse_bytes(b"0", 16).unwrap());
        Self { num, prime }
    }

    pub fn try_new(num: BigInt, prime: BigInt) -> Result<FieldElement, BTCErr> {
        if num >= prime || num < BigInt::parse_bytes(b"0", 16).unwrap() {
            return Err(BTCErr::FiniteFieldRangeErr { num, prime });
        }
        Ok(Self { num, prime })
    }

    pub fn checked_add(self, rhs: Self) -> Result<FieldElement, BTCErr> {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Addition".to_string()));
        }
        Ok(self + rhs)
    }

    pub fn checked_sub(self, rhs: Self) -> Result<FieldElement, BTCErr> {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Subtraction".to_string()));
        }
        Ok(self - rhs)
    }

    pub fn checked_mul(self, rhs: Self) -> Result<FieldElement, BTCErr> {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Multiplication".to_string()));
        }
        Ok(self * rhs)
    }

    pub fn checked_div(self, rhs: Self) -> Result<FieldElement, BTCErr> {
        if self.prime != rhs.prime {
            return Err(BTCErr::TwoDiffFiniteFields("Division".to_string()));
        }
        Ok(self / rhs)
    }

    pub fn pow_modulo(&self, n: BigInt) -> FieldElement {
        if n < BigInt::parse_bytes(b"0", 16).unwrap() {
            let f1 = FieldElement::new(BigInt::parse_bytes(b"1", 16).unwrap(), self.prime.clone());
            let f2 = FieldElement::new(self.num.clone(), self.prime.clone())
                .pow_modulo(BigInt::parse_bytes(b"-1", 16).unwrap() * n);
            return f1 / f2;
        }

        let mut result = BigInt::parse_bytes(b"1", 16).unwrap();
        let mut x = self.num.clone();
        let mut n = n;
        let m = self.prime.clone();
        while n.clone() > BigInt::parse_bytes(b"0", 16).unwrap() {
            if n.clone() % BigInt::parse_bytes(b"2", 16).unwrap()
                == BigInt::parse_bytes(b"1", 16).unwrap()
            {
                result = ((result % m.clone()) * (x.clone() % m.clone())) % m.clone();
            }
            x = ((x.clone() % m.clone()) * (x.clone() % m.clone())) % m.clone();
            n /= 2;
        }
        FieldElement {
            num: result,
            prime: self.prime.clone(),
        }
    }

    // pub fn checked_pow_modulo(self)
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        let num = modulo(
            modulo(self.num.clone(), self.prime.clone())
                + modulo(rhs.num.clone(), rhs.prime.clone()),
            self.prime.clone(),
        );
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        let num = modulo(
            modulo(self.num, self.prime.clone()) - modulo(rhs.num, rhs.prime) + self.prime.clone(),
            self.prime.clone(),
        );
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = modulo(
            modulo(self.num, self.prime.clone()) * modulo(rhs.num, rhs.prime),
            self.prime.clone(),
        );
        FieldElement {
            num,
            prime: self.prime,
        }
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
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        let rhs_mul_inv = rhs.pow_modulo(rhs.prime.clone() - 2);
        self * rhs_mul_inv
    }
}
