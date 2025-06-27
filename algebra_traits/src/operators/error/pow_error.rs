use thiserror::Error;
use num_traits::Zero;
use super::{FloatOpError,CheckFloatInput};

#[derive(Clone,Debug,Error,PartialEq)]
pub enum PowError {
    #[error("power error: {0}")]
    FloatOp(FloatOpError),
    #[error("negative power of zero is not defined")]
    NegativePowerOfZeroNotDefined,
    #[error("non integral power of negative number is not defined")]
    NonIntegralPowerOfNegativeNumberNotDefined
}
impl<E:Into<FloatOpError>> From<E> for PowError {
    fn from(value: E) -> Self {
        PowError::FloatOp(value.into())
    }
}

pub trait CheckPowInput<Rhs:CheckFloatInput=Self> : CheckFloatInput {
    fn check_pow_input(&self,rhs:&Rhs) -> Result<(),PowError>;
}

macro_rules! impl_check_pow_input {
    ($t:ty) => {
        impl CheckPowInput for $t {
            fn check_pow_input(&self,rhs:&Self) -> Result<(),PowError> {
                self.check_float_input()?;
                rhs.check_float_input()?;
                let z=Self::zero();
                     if  self == &z     &&  rhs         < &z { Err(PowError::NegativePowerOfZeroNotDefined) }
                else if  self <  &z     &&  rhs.fract() != z { Err(PowError::NonIntegralPowerOfNegativeNumberNotDefined) }
                else { Ok(()) }
            }
        }
    };
}
impl_check_pow_input!(f32);
impl_check_pow_input!(f64);