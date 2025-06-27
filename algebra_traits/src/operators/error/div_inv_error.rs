
use thiserror::Error;
use super::{InvalidFloatInputError, FloatOpError, OverflowError};

#[derive(Clone,Debug,Error,PartialEq,Eq)]
#[error("division by zero")]
pub struct DivisionByZeroError;

impl DivisionByZeroError {
    pub fn try_new<F:crate::IsAZero>(divisor:&F) -> Result<(),Self> {
        if divisor.is_a_zero() {
            Err(Self)
        } else {
            Ok(())
        }
    }
}


#[derive(Clone,Debug,Error,PartialEq,Eq)]
pub enum InvalidDivisor {
    #[error(transparent)]
    DivisionByZero(#[from] DivisionByZeroError),
    #[error("divisor error")]
    Other
}

impl From<NotInvertibleError> for InvalidDivisor {
    fn from(value: NotInvertibleError) -> Self {
        match value {
            NotInvertibleError::ZeroNotInvertible => InvalidDivisor::DivisionByZero(DivisionByZeroError),
            NotInvertibleError::Other => InvalidDivisor::Other,
        }
    }
}


#[derive(Clone,Debug,Error,PartialEq)]
pub enum DivError {
    #[error("division error: {0}")]
    FloatOp(#[from] FloatOpError),
    #[error("invalid divisor for division : {0}")]
    InvalidDivisor(#[from] InvalidDivisor)
}
utils::from_via!(impl From<OverflowError>          for DivError, via FloatOpError);
utils::from_via!(impl From<InvalidFloatInputError> for DivError, via FloatOpError);
utils::from_via!(impl From<DivisionByZeroError>    for DivError, via InvalidDivisor);

impl From<InvError> for DivError {
    fn from(value: InvError) -> Self {
        match value {
            InvError::FloatOp(math_op_error) => DivError::FloatOp(math_op_error),
            InvError::NotInvertible(not_inv) => DivError::InvalidDivisor(not_inv.into())
        }
    }
}


#[derive(Clone,Debug,Error,PartialEq,Eq)]
pub enum NotInvertibleError {
    #[error("Zero is not invertible")]
    ZeroNotInvertible,
    #[error("Can not compute the inverse")]
    Other
}

impl From<InvalidDivisor> for NotInvertibleError {
    fn from(value: InvalidDivisor) -> Self {
        match value {
            InvalidDivisor::DivisionByZero(_) => NotInvertibleError::ZeroNotInvertible,
            InvalidDivisor::Other => NotInvertibleError::Other,
        }
    }
}

impl From<DivisionByZeroError> for NotInvertibleError {
    fn from(_: DivisionByZeroError) -> Self {
        Self::ZeroNotInvertible
    }
}

#[derive(Clone,Debug,Error,PartialEq)]
pub enum InvError {
    #[error("math error during computation of inverse")]
    FloatOp(#[from] FloatOpError),
    #[error(transparent)]
    NotInvertible(#[from] NotInvertibleError)
}
utils::from_via!(impl From<OverflowError>          for InvError, via FloatOpError);
utils::from_via!(impl From<InvalidFloatInputError> for InvError, via FloatOpError);
utils::from_via!(impl From<DivisionByZeroError>    for InvError, via NotInvertibleError);

impl From<DivError> for InvError {
    fn from(value: DivError) -> Self {
        match value {
            DivError::FloatOp(math_op_error) => InvError::FloatOp(math_op_error),
            DivError::InvalidDivisor(invalid_divisor) => InvError::NotInvertible(invalid_divisor.into())
        }
    }
}