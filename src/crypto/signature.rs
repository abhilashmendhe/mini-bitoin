use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: BigInt,
    pub s: BigInt
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Self {
        Signature { r, s }
    }
}

impl std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature({:x},{:x})", self.r, self.s)
    }
}