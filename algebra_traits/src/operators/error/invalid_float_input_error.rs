use container_traits::LenNotEqualToRequiredLenError;
use thiserror::Error;

#[derive(Clone,Debug,Error,PartialEq,Eq)]
#[error("an input was NaN (not a number)")]
pub struct NaNError;

#[derive(Clone,Debug,Error,PartialEq,Eq)]
#[error("an input was infinite")]
pub struct InFiniteError;

#[derive(Clone,Debug,Error,PartialEq,Eq)]
#[error("the operation is not available for the provided inputs")]
pub struct OperationNotAvailableError;

#[derive(Clone,Debug,Error,PartialEq,Eq)]
#[error("invalid dimensions for matrix multiplication. number of columns left hand side={ncols_lhs} whereas number of rows right hand side={nrows_rhs}")]
pub struct DimensionsNotValidForMatrixMultiplicationError {
    ncols_lhs:usize,
    nrows_rhs:usize
}

#[derive(Clone,Debug,Error,PartialEq)]
pub enum InvalidFloatInputError {
    #[error(transparent)]
    NaN(#[from] NaNError),
    #[error(transparent)]
    InFinite(#[from] InFiniteError),
    #[error(transparent)]
    LenNotEqualToRequiredLen(#[from] LenNotEqualToRequiredLenError),
    #[error(transparent)]
    DimensionsNotValidForMatrixMultiplication(#[from] DimensionsNotValidForMatrixMultiplicationError),
    #[error(transparent)]
    OperationNotAvailable(#[from] OperationNotAvailableError)
}

pub trait CheckFloatInput {
    fn check_float_input(&self) -> Result<(),InvalidFloatInputError>;
}

macro_rules! impl_check_float_input {
    ($t:ty) => {
        impl CheckFloatInput for $t {
            fn check_float_input(&self) -> Result<(),InvalidFloatInputError> {
                if      self.is_nan() {
                    Err(NaNError.into())
                } else if !self.is_finite() {
                    Err(InFiniteError.into())
                } else {
                    Ok(())
                }
            }
        }
    };
}
impl_check_float_input!(f32);
impl_check_float_input!(f64);

