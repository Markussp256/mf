pub mod div_inv_error;
pub use div_inv_error::{DivError,InvError,DivisionByZeroError,InvalidDivisor,NotInvertibleError};

pub mod invalid_float_input_error;
pub use invalid_float_input_error::*;

pub mod float_op_error;
pub use float_op_error::{FloatOpError,OverflowError, CheckOutput};

pub mod pow_error;
pub use pow_error::{PowError,CheckPowInput};

use thiserror::Error;

macro_rules! impl_from {
    ($name:ident) => {
        impl<E:Into<FloatOpError>> From<E> for $name {
            fn from(value: E) -> Self {
                $name::FloatOp(value.into())
            }
        }
    };
}

#[derive(Clone,Debug,Error,PartialEq)]
pub enum DetError {
    #[error("matherror during computation of determinant")]
    FloatOp(FloatOpError),
    #[error("determinant not defined for nonsquare matrix")]
    DetOnlyDefinedForSquareMatrices
}
impl_from!(DetError);

#[derive(Clone,Debug,Error,PartialEq)]
pub enum Pow2Error {
    #[error("matherror during computation of square power")]
    FloatOp(FloatOpError)
}
impl_from!(Pow2Error);

#[derive(Clone,Debug,Error,PartialEq)]
pub enum ExpError {
    #[error("matherror during computation of exponential")]
    FloatOp(FloatOpError),
}
impl_from!(ExpError);


#[derive(Clone,Debug,Error,PartialEq)]
pub enum LogError {
    #[error("matherror during computation of logarithm")]
    FloatOp(FloatOpError),
    #[error("log of nonpositive realnumber not possible")]
    LogOfNonPositiveRealNumberNotPossible,
    #[error("log of complex number on nonpositive real axis not possible")]
    LogOfComplexNumberOnNonPositiveRealAxisNotPossible
}
impl_from!(LogError);

#[derive(Clone,Debug,Error,PartialEq)]
pub enum SqrtError {
    #[error("math error during computation of squareroot")]
    FloatOp(FloatOpError),
    #[error("squareroot of negative real number not possible")]
    SqrtOfNegativeNumberNotPossible,
    #[error("a quanity with an odd dimension does not have a square root")]
    AllDimensionsOfQuantityMustBeEven
}
impl_from!(SqrtError);


#[derive(Clone,Debug,Error,PartialEq)]
pub enum AddError {
    #[error("adding error: {0}")]
    FloatOp(FloatOpError)
}
impl_from!(AddError);


#[derive(Clone,Debug,Error,PartialEq)]
pub enum SubError {
    #[error("subtraction error: {0}")]
    FloatOp(FloatOpError)
}
impl_from!(SubError);


#[derive(Clone,Debug,Error,PartialEq)]
pub enum MulError {
    #[error("multiplication error: {0}")]
    FloatOp(FloatOpError)
}
impl_from!(MulError);




