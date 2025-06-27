use super::MatrixNotRegularError;

use container_traits::LensNotEqualError;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum MatrixSolveError {
    #[error("matrix is not square, but this type of system can only be solved for square matrices")]
    MatrixNotSquare,

    #[error("matrix is wide, system therefore has more unknowns than equations")]
    MatrixIsWide,

    #[error(transparent)]
    MatrixNotRegular(#[from] MatrixNotRegularError),

    #[error("number of rows of matrix is not equal to length of columnvector, {0}")]
    LenNotEqual(#[from] LensNotEqualError)
}