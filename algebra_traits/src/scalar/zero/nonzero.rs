use crate::{Field, TryDiv, TryInv};
use std::ops::Div;
use num_traits::{Zero,Inv};

// unfortunately we can not use std::num::NonZero
// because we can not implement traits generically
// without the use of unstable features

#[derive(Clone,
         Copy,
         Debug,
         algebra_derive::One,
         algebra_derive::ClosedMul,
         container_derive::AsRef)]
pub struct NonZero<T>(T);

// marker trait that sais we can divide by any NonZero.
// this is for example not true for matrices
pub trait IntegralDomain {}

impl<T: Div<NonZero<T2>, Output=TR>, T2:IntegralDomain, TR> Div<NonZero<T2>> for NonZero<T> {
    type Output=NonZero<TR>;
    fn div(self, rhs:NonZero<T2>) -> Self::Output {
        NonZero(self.into_inner().div(rhs))
    }
}

impl<T : TryDiv<T2, Output=TR>, T2, TR> TryDiv<T2> for NonZero<T> {
    type Output=NonZero<TR>;
    type Error=<T as TryDiv<T2>>::Error;

    fn is_divable_by(&self,rhs:&T2) -> Result<(),Self::Error> {
        self.as_ref()
            .is_divable_by(rhs)   
    }

    fn try_div(self,rhs:T2) -> Result<Self::Output,Self::Error> {
        self.into_inner()
            .try_div(rhs)
            .map(|d|NonZero(d))
    }
}

impl<F:TryInv<Output=F>+Field> Inv for NonZero<F> {
    type Output=Self;
    fn inv(self) -> Self {
        NonZero(<F as TryInv>::try_inv(self.into_inner()).ok().unwrap())
    }
}



impl<T> NonZero<T> {
    pub fn try_new(t:T) -> Option<Self> where T:Zero {
        (!t.is_zero()).then(||
            Self(t))
    }

    pub const fn new_unchecked(t:T) -> Self {
        Self(t)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}