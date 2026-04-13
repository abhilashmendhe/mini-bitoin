use thiserror::Error;

#[derive(Debug, Error)]
pub enum BTCErr {
    #[error("Number {} is not in the field range 0 to {}", .num, .prime)]
    FiniteFieldRangeErr {
        num:   isize,
        prime: isize
    },
}