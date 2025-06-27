
use super::InvalidFloatInputError;
use thiserror::Error;


#[derive(Clone,Debug,Error,PartialEq,Eq)]
#[error("an overflow occured during a math operation")]
pub struct OverflowError;

pub trait CheckOutput {
    fn check_output(&self) -> Result<(),OverflowError>;
}

macro_rules! check_output {
    ($f:ty) => {
        impl CheckOutput for $f {
            fn check_output(&self) -> Result<(),OverflowError> {
                if self.is_finite() {
                    Ok(())
                } else {
                    Err(OverflowError)
                }
            }
        }
    };
}
check_output!(f32);
check_output!(f64);

#[derive(Clone,Debug,Error,PartialEq)]
pub enum FloatOpError {
    #[error("invalid input: {0}")]
    InvalidFloatInput(InvalidFloatInputError),
    #[error("overflow occured")]
    Overflow(#[from] OverflowError)
}

impl<E:Into<InvalidFloatInputError>> From<E> for FloatOpError {
    fn from(value: E) -> Self {
        FloatOpError::InvalidFloatInput(value.into())
    }
}

