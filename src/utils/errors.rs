use thiserror::Error;

#[derive(Debug, Error)]
pub enum BTCErr {
    #[error("Number {} is not in the field range 0 to {}", .num, .prime)]
    FiniteFieldRangeErr {
        num:   isize,
        prime: isize
    },

    #[error("Finite fields have different primes. Cant perform {}", .0)]
    TwoDiffFiniteFields(String),

    #[error("{}", .0)]
    PointNotOnECC(String),

    #[error("{}", .0)]
    PointNotOnSameECC(String),
}