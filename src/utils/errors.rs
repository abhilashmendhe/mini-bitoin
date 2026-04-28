use std::array::TryFromSliceError;

use hex::FromHexError;
use num_bigint::BigInt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BTCErr {
    #[error("Number {} is not in the field range 0 to {}", .num, .prime)]
    FiniteFieldRangeErr { num: BigInt, prime: BigInt },

    #[error("Finite fields have different primes. Cant perform {}", .0)]
    TwoDiffFiniteFields(String),

    #[error("{}", .0)]
    PointNotOnECC(String),

    #[error("{}", .0)]
    PointNotOnSameECC(String),

    #[error("{}",.0)]
    FromHexError(#[from] FromHexError),

    #[error("{}",.0)]
    IoErr(#[from] std::io::Error),

    #[error("{}",.0)]
    SliceErrorFromBytes(#[from] TryFromSliceError),
}
