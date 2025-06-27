pub mod field;
use container_traits::for_static::TryFromIterator;
use container_traits::ContainerConstructError;
pub use field::{Ring, Field};

pub mod impl_real_number;

pub mod one;
pub use one::IsAOne;
// we dont reexport One because of potential name clashes with num_traits::One

pub mod scalar_mul;
pub use scalar_mul::{ScalarMul, ScalarDiv, TryScalarDiv};

pub mod trigonometric_functions;
pub use trigonometric_functions::{TrigonometricFunctions, TryATan2, Sinc};

pub mod zero;
// we dont reexport Zero because of potential name clashes with num_traits::Zero
pub use zero::{IsAZero, NonZero, IntegralDomain};

pub mod consts;
pub use consts::{ConstZero, ConstNonZero, ConstOne, ConstPi, ConstRad2Deg, ConstDeg2Rad};

use crate::{Conjugate, DivError, FiniteDimensionalInnerProductSpace, LogError, Nonnegative, Norm, NormSquared, Origin, Pow2, PowError, SqrtError, Tolerance, TryDiv, TryIntoReal, TryLog, TryNormalize};

// marker trait
use crate::{Exp, InnerProductSpace1d, Max, TryPow, TrySqrt};

use std::ops::{AddAssign, SubAssign, Mul};

use num_traits::{Zero,One};

pub trait CastFromf64 {
    fn from_f64(value:f64) -> Self;
}

// Scalar can be f64 or c64 and implements basically all traits

pub trait Scalar : 'static
                  +Field
                  +Conjugate
                  +Pow2<Output=Self>
                  +AddAssign+SubAssign
                  +Norm<NormT=Self::RealType>
                  +NormSquared<Norm2T=Self::RealType>
                  +Origin
                  +IsAZero
                  +PartialEq
                  +From<Self::RealType>
                  +Exp<Output=Self>
                  +TryLog<Output=Self,         Error=LogError>
                  +TryScalarDiv<Self,          Error=DivError>
                  +TryScalarDiv<Self::RealType,Error=DivError>
                  +TryIntoReal<Output=Self::RealType>
                  +TryDiv<Self::RealType,Output=Self,Error=DivError>
                  +TryNormalize
                  +TryPow<i16, Output=Self, Error=PowError>
                  // +TryPow<Self::RealType, Output=Self>
                  +InnerProductSpace1d<     NormT = Self::RealType,
                                            DistT = Self::RealType,
                                         TryDistT = Self::RealType,
                                          ScProdT = Self,
                                       TryScProdT = Self>
                  +FiniteDimensionalInnerProductSpace<Self,1>
                  +Tolerance<DistT=Self::RealType>
                  +Mul<Self::RealType, Output=Self>
                  +ScalarMul<Self::RealType>
                  +ConstZero
                  +ConstOne
                  +ConstNonZero
                  +TrigonometricFunctions<Output=Self>
                  +Sinc<Output=Self>
                  +TryFromIterator<Self::RealType,ContainerConstructError<usize>> {
    type RealType : RealNumber;
    
    // provided method
    fn basis_over_r() -> Vec<Self>;
}


// i16 because if we convert to f32 and back we get the same value which is not the case for f32
pub trait RealNumber : Scalar<RealType=Self>
                        +PartialOrd
                        +Max
                        +From<i16>
                        +CastFromf64
                        +ConstPi
                        +ConstDeg2Rad
                        +ConstRad2Deg
                        +TryPow<Self, Output=Self, Error=PowError>
                        +TrySqrt<Output=Nonnegative<Self>, Error=SqrtError>
                        +TryATan2<Output=Self>  {
    fn is_positive   (&self) -> bool { self >  &Self::ZERO }
    fn is_negative   (&self) -> bool { self <  &Self::ZERO }
    fn is_nonpositive(&self) -> bool { self <= &Self::ZERO }
    fn is_nonnegative(&self) -> bool { self >= &Self::ZERO }
}

// marker trait

// note: real and into_real is defined in Scalar
pub trait RealAndImag {
    type RealT;
    fn new(real:Self::RealT, imag:Self::RealT) -> Self;
    fn real(&self) -> &Self::RealT;
    fn imag(&self) -> &Self::RealT;
    fn into_real_imag(self) -> [Self::RealT;2];
    
    // provided method
    fn i() -> Self where Self: Sized, Self::RealT:Zero+One {
        Self::new(<Self::RealT as Zero>::zero(), 
                  <Self::RealT as One>::one())}
}

pub trait ComplexNumber : Scalar + RealAndImag<RealT = <Self as Scalar>::RealType> {}

pub trait NthRoots : Sized {
    fn nth_roots(self, n:u8) -> Vec<Self>;
}




// fn compare<F:Scalar>(f:F) -> bool {
//     let r:F::RealType=F::RealType::from(1.0);
//     let nr=<F::RealType as crate::Norm>::norm(r);
//     &nr < &F::from(F::RealType::from(0.2))
// }