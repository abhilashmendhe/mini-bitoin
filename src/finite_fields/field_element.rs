use crate::utils::errors::BTCErr;

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