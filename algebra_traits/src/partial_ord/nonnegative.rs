use std::fmt::{Debug,Display};
use crate::{metric::TryDistance, TryAdd, TrySub, Distance, Max, Sqrt, TryLog, TryPow, TrySqrt};

use std::ops::{Add,Div,Mul,Neg,Sub};
use num_traits::{One, Pow, Zero};

// we dont derive ParitialOrd and others because we want to define it generically

#[derive(Clone, Copy,Debug,
        PartialEq, PartialOrd
        )]
pub struct Nonnegative<T>(T);

// can not use derive_macro because traits are in this crate (algebra_traits)
impl<T:Zero> Zero for Nonnegative<T> {
    fn zero() -> Self {
        Nonnegative(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T:TryAdd<Output=T>> TryAdd for Nonnegative<T> {
    type Output=Self;
    type Error=<T as TryAdd>::Error;
    fn is_addable_by(&self,rhs:&Self) -> Result<(),Self::Error> {
        self.0
            .is_addable_by(&rhs.0)
    }

    fn try_add(self,rhs:Self) -> Result<Self,Self::Error> {
        self.0
            .try_add(rhs.0)
            .map(|r|Nonnegative(r))
    }
}

impl<T:Add<Output=T>> Add for Nonnegative<T> {
    type Output=Self;
    fn add(self,rhs:Self) -> Self {
        Nonnegative(self.0 + rhs.0)
    }
}

impl<T> Nonnegative<T> {
    pub fn try_new(t:T) -> Option<Self> where T:Zero+PartialOrd {
        (t >= T::zero()).then(
            ||Self(t))
    }

    pub fn as_signed_ref(&self) -> &T {
        &self.0
    }

    // can not implement Into Trait Generically
    pub fn into_signed(self) -> T {
        self.0
    }
}

impl<T:Neg> Neg for Nonnegative<T> {
    type Output=<T as Neg>::Output;
    fn neg(self) -> Self::Output {
        self.0
            .neg()
    }
}

impl<T:Mul<T2,Output=TR>,T2,TR> Mul<Nonnegative<T2>> for Nonnegative<T> {
    type Output=Nonnegative<TR>;
    fn mul(self, rhs:Nonnegative<T2>) -> Nonnegative<TR> {
        Nonnegative(
        self.into_signed().mul(rhs.into_signed()))
    }
}

impl<T:One> One for Nonnegative<T> {
    fn one() -> Self {
        Self(T::one())
    }
}

impl<T:Div<T2,Output=TR>,T2,TR> Div<Nonnegative<T2>> for Nonnegative<T> {
    type Output=Nonnegative<TR>;
    fn div(self, rhs:Nonnegative<T2>) -> Nonnegative<TR> {
        Nonnegative(
        self.into_signed()
            .div(rhs.into_signed()))
    }
}

impl<T:TryAdd<Output=T>> TryAdd<T> for Nonnegative<T> {
    type Output=T;
    type Error=<T as TryAdd>::Error;
    fn is_addable_by(&self,rhs:&T) -> Result<(),Self::Error> {
        self.as_signed_ref()
            .is_addable_by(rhs)
    }

    fn try_add(self, rhs:T) -> Result<T,Self::Error> {
        self.into_signed()
            .try_add(rhs)
    }  
}

impl<T:Add<Output=T>> Add<T> for Nonnegative<T> {
    type Output=T;
    fn add(self, rhs:T) -> T {
        self.into_signed()
            .add(rhs)
    }
}

impl<T:Sub<Output=T>> Sub<T> for Nonnegative<T> {
    type Output=T;
    fn sub(self, rhs:T) -> T {
        self.into_signed()
            .sub(rhs)
    }
}

impl<T:TrySub<Output=T>> TrySub<T> for Nonnegative<T> {
    type Output=T;
    type Error=<T as TrySub>::Error;
    fn is_subable_by(&self,rhs:&T) -> Result<(),Self::Error> {
        self.as_signed_ref()
            .is_subable_by(rhs)
    }

    fn try_sub(self, rhs:T) -> Result<T,Self::Error> {
        self.into_signed()
            .try_sub(rhs)
    }  
}

impl<T:TryDistance> TryDistance for Nonnegative<T> {
    type TryDistT=T::TryDistT;
    type Error=<T as TryDistance>::Error;
    fn try_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<T::TryDistT>,<T as TryDistance>::Error> {
        let rhs:Self=rhs.into();
        self.0
            .try_distance(rhs.0)
    }
}

impl<T:Distance> Distance for Nonnegative<T> {
    type DistT=T::DistT;
    fn distance(self, rhs:impl Into<Self>) -> Nonnegative<Self::DistT> {
        let rhs:Self=rhs.into();
        self.0.distance(rhs.0)
    }
}


// impl<T> AsRef<Nonnegative<T>> for Nonnegative<T> {
//     fn as_ref(&self) -> &Nonnegative<T> {
//         self
//     }
// }

// need to be able to compare with all types that compare with T
// construction with Into works because its reflexive which asref is not



impl<T> Nonnegative<T> {
    pub fn eq4nonnegative<S>(&self, rhs:&Nonnegative<S>) -> bool where T:PartialEq<S> {
        self.0.eq(&rhs.0)
    }
}

impl<T:PartialEq> PartialEq<T> for Nonnegative<T> {
    fn eq(&self, rhs: &T) -> bool {
        self.0.eq(rhs)
    }
}

impl<T:PartialOrd> PartialOrd<T> for Nonnegative<T> {
    fn partial_cmp(&self, rhs: &T) -> Option<std::cmp::Ordering> {
        self.0
            .partial_cmp(&rhs)
    }
}

impl<T:Max> Max for Nonnegative<T> {
    fn max<'a>(&'a self, rhs:&'a Self) -> &'a Self {
        let ma=self.0.max(&rhs.0);
        if &self.0 == ma {
            self
        } else {
            rhs
        }
    }
}

impl<T> Nonnegative<T> {
    pub fn cmp4nonnegative<S>(&self, rhs:&Nonnegative<S>) -> Option<std::cmp::Ordering> where T:PartialOrd<S> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl<T:Display> Display for Nonnegative<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nonnegative {}", self.0)
    }
}

// assumes the only reason try_sqrt doesnt work is because it can be negative
impl<T:TrySqrt<Error=E>,E:Debug> Sqrt for Nonnegative<T> {
    type Output=<T as TrySqrt>::Output;
    fn sqrt(self) -> Self::Output {
        self.0
            .try_sqrt()
            .unwrap()
    }
}

impl<T:TryPow<T,Error=E>,E:Debug> Pow<Nonnegative<T>> for Nonnegative<T> {
    type Output=Nonnegative<<T as TryPow<T>>::Output>;
    fn pow(self, t:Nonnegative<T>) -> Self::Output {
        Nonnegative(
            self.0
                .try_pow(t.0)
                .unwrap())
    }
}

impl<T:TryLog> TryLog for Nonnegative<T> {
    type Output=<T as TryLog>::Output;
    type Error=<T as TryLog>::Error;
    // yields None for self<0

    fn is_logable(&self) -> Result<(),Self::Error> {
        self.0
            .is_logable()
    }

    fn try_log(self) -> Result<Self::Output,Self::Error> {
        self.0
            .try_log()
    }
}

impl Into<f64> for Nonnegative<f64> {
    fn into(self) -> f64 {
        self.into_signed()
    }
}

impl Div<Nonnegative<f64>> for f64 {
    type Output=f64;
    fn div(self, rhs:Nonnegative<f64>) -> Self::Output {
        self / rhs.into_signed()
    }
}

