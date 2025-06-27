use std::convert::Infallible;
use std::ops::{Neg,Mul,Div};
use std::fmt::Display;
use algebra_traits::{div_by_small_natural::Div2,*};
use num_traits::{Inv, Zero, One};

use crate::EnhancedArray;

// we dont use container_derive for add and sub
// because that would implement it too general
// generating conflicts with other definitions of Complex
#[derive(Clone,
        Debug,
        PartialEq,
        algebra_derive::AdditiveGroup,
        algebra_derive::Distance,
        algebra_derive::Norm,
        algebra_derive::NormSquared,
        algebra_derive::AddAssignFromAdd,
        algebra_derive::SubAssignFromSub,
        container_derive::FromFn,
        container_derive::TryFromIterator,
        container_derive::IntoIterator,
        container_derive::Iter,
        container_derive::NumberOfDegreesOfFreedom
        )]
pub struct Complex<T:'static>(EnhancedArray<T,2>);

impl<T:Display+Zero+PartialOrd> Display for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width=f.width().unwrap_or(6);
        let precision=f.precision().unwrap_or(4);
        write!(f, "{:+width$.precision$}",  self.real(), width=width, precision=precision)?;
        write!(f, "{:+width$.precision$}i", self.imag(), width=width, precision=precision)
    }
}

impl<T> RealAndImag for Complex<T> {
    type RealT = T;
    fn new(real:Self::RealT, imag:Self::RealT) -> Self {
        Self(EnhancedArray::new([real, imag]))
    }
    
    fn real(&self) -> &Self::RealT {
        &self.0[0]
    }
    
    fn imag(&self) -> &Self::RealT {
        &self.0[1]
    }
    
    fn into_real_imag(self) -> [Self::RealT;2] {
        self.0
            .into()
    }
}


impl<T> Complex<T> {
    pub fn into_real(self) -> T {
        let [real,_]=self.into_real_imag();
        real
    }

    pub fn into_imag(self) -> T {
        let [_,imag]=self.into_real_imag();
        imag
    }
}

impl<R:RealNumber> NthRoots for Complex<R> {
    fn nth_roots(self, n:u8) -> Vec<Self> {
        // returns all n-th roots
        match n {
            0 => Vec::new(),
            _ => {
                let one_root=self.try_a_pow(R::one().try_div(R::from(n as i16)).unwrap()).unwrap();
                Self::unit_roots(n).into_iter()
                                   .map(|ur|ur*one_root.clone())
                                   .collect()
            }
        }
    }
}

impl<T:Neg<Output=T>> Conjugate for Complex<T> {
    fn conjugate(self) -> Self {
        let [real, imag]=self.into_real_imag();
        Self::new(real, -imag)
    }
}

impl<T:Zero> From<T> for Complex<T> {
    fn from(t:T) -> Self {
        Self::new(t,T::zero())
    }
}

impl<T:Zero> TryIntoReal for Complex<T> {
    type Output=T;
    fn try_into_real(self) -> Option<T> {
        (!self.is_zero()).then(||self.into_real())
    }
}

// trigo stuff
impl<F:Clone+RealNumber> Complex<F> {
    pub fn from_norm_arg<A:Clone+TrigonometricFunctions<Output=R>, R>(norm:Nonnegative<F>, arg:A) -> Self where F:Mul<R, Output=F> {
        let norm_s=norm.into_signed();
        Self::new(norm_s.clone()*arg.clone().cos(),
                        norm_s*arg.sin())
    }
}

impl<T:Clone+TryATan2> Complex<T> {
    pub fn arg(self) -> Option<<T as TryATan2>::Output> {
         let [real, imag]=self.into_real_imag();
         T::try_atan2(imag, real)
    }
}

impl<T:Clone+RealNumber> TryLog for Complex<T> {
    type Output=Self;
    type Error=LogError;
    fn is_logable(&self) -> Result<(),LogError> {
        if self.imag().is_zero() && self.real().is_nonpositive() {
            Err(LogError::LogOfComplexNumberOnNonPositiveRealAxisNotPossible)
        } else {
            Ok(())
        }
    }

    fn try_log(self) -> Result<Self,LogError> {
        self.is_logable()?;
        let real=self.clone()
                        .norm()
                        .try_log()
                        .ok()
                        .unwrap();
        let imag=self.arg()
                        .unwrap();
        Ok(Self::new(real, imag))
    }
}


impl Mul<Complex<f64>> for f64 {
    type Output=Complex<f64>;
    fn mul(self, rhs:Complex<f64>) -> Self::Output {
        let [rhs_real, rhs_imag]=rhs.into_real_imag();
        Self::Output::new(self*rhs_real,
                          self*rhs_imag)
    }
}

impl<T:Clone+RealNumber> Exp for Complex<T> {
    type Output=Self;

    fn exp(self) -> Self {
        let [real, imag]=self.into_real_imag();
        Self::from_norm_arg::<T,T>(Nonnegative::try_new(real
                                             .exp()).unwrap(), 
                                   imag)
    }
}

impl<R:Clone+RealNumber> Complex<R> {
    fn expi(self) -> Self {
        Self::exp(Self::i() * self)
    }
}

impl<R:Clone+RealNumber> Complex<R> {
    // A power, but not the only one
    pub fn try_a_pow(self, rhs: R) -> Result<Self,PowError> {
            self.clone()
                .norm()
                .into_signed()
                .try_pow(rhs.clone())
                .map(|r|Nonnegative::try_new(r).unwrap())// result is nonnegative because norm is nonnegative
                .map(|ur|Self::from_norm_arg(ur, rhs*self.arg().unwrap()))
    }

    pub fn unit_roots(n:u8) -> Vec<Self> {
        match n {
            0 => Vec::new(),
            _ => {
                let dangle=<R as MulI>::muli(&R::PI,2).try_div(R::from(n as i16)).unwrap();
                let mut angle=R::zero();
                let mut res=Vec::new();
                for _ in 0..n {
                    res.push(Self::from_norm_arg(Nonnegative::<R>::one(), angle.clone()));
                    angle=angle+dangle.clone();
                }
                res
            }
        }
    }
}


impl<R:Clone+RealNumber> TryPow<i16> for Complex<R> {
    type Output=Self;
    type Error=PowError;

    fn is_powable_by(&self,rhs:&i16) -> Result<(),PowError> {
        self.clone()
            .try_pow(rhs.clone())
            .map(|_|())
    }

    fn try_pow(self, rhs: i16) -> Result<Self,PowError> {
        self.try_a_pow(rhs.into())
    }
}

impl<T:Clone+Field, S:Into<Complex<T>>> Mul<S> for Complex<T> {
    type Output=Self;

    fn mul(self, rhs: S) -> Self {
        let [real, imag]=self.into_real_imag();
        let rhs:Self=rhs.into();
        let [rhs_real, rhs_imag]=rhs.into_real_imag();
        Self::new(real.clone()*rhs_real.clone()-imag.clone()*rhs_imag.clone(),
                  real.clone()*rhs_imag.clone()+imag.clone()*rhs_real.clone())
    }
}

impl<T:Clone+Field, S:Into<Complex<T>>> TryMul<S> for Complex<T> {
    type Output = Self;
    type Error = Infallible;
    fn is_mulable_by(&self,_:&S) -> Result<(),Infallible> {
        Ok(())
    }

    fn try_mul(self,rhs:S) -> Result<Self::Output,Infallible> {
        Ok(self * rhs)
    }
}

impl<T:Field+Clone> One for Complex<T> {
    fn one() -> Self {
        Self::new(<T as num_traits::One>::one(),T::zero())
    }
}

impl<T:Clone+RealNumber> TryInv for Complex<T> {
    type Output=Self;
    type Error=InvError;
    fn is_invertible(&self) -> Result<(),          InvError> {
        DivisionByZeroError::try_new(self)?;
        Ok(())
    }

    fn try_inv(self) -> Result<Self,InvError> {
        self.is_invertible()?;
        let n2=self.clone()
                      .norm_squared()
                      .into_signed();
        Ok(Self(
            self.conjugate()
                .0
                .try_div(n2).unwrap()))
    }
}

impl<T:Clone+RealNumber> TryDiv for Complex<T> {
    type Output = Self;
    type Error=DivError;
    fn is_divable_by(&self,rhs:&Self) -> Result<(),DivError> {
        DivisionByZeroError::try_new(rhs)?;
        Ok(())    
    }

    fn try_div(self, rhs:Self) -> Result<Self,DivError> {
        rhs.try_inv()
           .map(|ri|self * ri)
           .map_err(|e|e.into())
    }
}

impl<T:Clone+RealNumber> Div<NonZero<Complex<T>>> for Complex<T> {
    type Output=Complex<T>;
    fn div(self, rhs:NonZero<Complex<T>>) -> Complex<T> {
        self * rhs.inv().into_inner()
    }
}

macro_rules! divi {
    ($i:literal) => {
        paste::paste!(
        mod [<div $i>] {
            use algebra_traits::operators::div_by_small_natural::[<Div $i>];
            impl<T:[<Div $i>]> [<Div $i>] for super::Complex<T> {
                fn [<div $i>](self) -> Self {
                    Self(self.0.[<div $i>]())
                }
            }
        });
    }
}
impl_div2to10!(divi);

impl<T:Clone+RealNumber> ScalarMul<T> for Complex<T> {
    fn scalar_mul(self, f:&T) -> Self {
        self* Self::from(f.clone())
    }
}

// impl<T:Clone+RealNumber> TryScalarDiv<T> for Complex<T> {}

// impl<T:Clone+RealNumber> Vectorspace<T> for Complex<T> {}

// impl<T:Clone+RealNumber> FiniteDimensionalVectorspace<T,2> for Complex<T> {
//     fn basis() -> Vec<Self> {
//         vec![Self::one(), Self::i()]
//     }
// }

impl<R:Clone+Field> Pow2 for Complex<R> {
    type Output=Self;
    fn pow2(self) -> Self {
        self.clone() * self
    }
}

impl<T:RealNumber> TryScalarDiv<T> for Complex<T> {
    type Error=DivError;
    fn try_scalar_div(self, f:&T) -> Result<Self,DivError> {
        self.0
            .try_scalar_div(f)
            .map(|s|Self(s))
    }
}

impl<T:RealNumber> IsAZero for Complex<T> {
    fn is_a_zero(&self) -> bool {
        self.is_zero()
    }
}

impl<T:RealNumber> ComplexNumber for Complex<T> {}

impl<T> IntegralDomain for Complex<T> {}

impl<R:RealNumber> ScalarMul<Self> for Complex<R> {
    fn scalar_mul(self, f:&Self) -> Self {
        self* f.clone()
    }
}

impl<R:Clone+RealNumber> TryScalarDiv<Self> for Complex<R> {
    type Error = DivError;
    fn try_scalar_div(self, f:&Self) -> Result<Self,DivError> {
        <Self as TryDiv>::try_div(self, f.clone())
    }
}

impl<R:RealNumber> Basis<Self> for Complex<R> {
    fn basis() -> impl ExactSizeIterator<Item=Self> {
        std::iter::once(Complex::one())
    }
}

impl<R:RealNumber> FiniteDimensionalVectorspace<Self,1> for Complex<R> {
}

impl<R:RealNumber> Scalarproduct for Complex<R> {
    type ScProdT = Self;
    fn scalar_product(self, rhs:Self) -> Self::ScProdT {
        self.conjugate() * rhs
    }
}

impl<R:RealNumber> TryScalarproduct for Complex<R> {
    type TryScProdT = Self;
    fn try_scalar_product(self, rhs:Self) -> Option<Self> {
        Some(self.conjugate() * rhs)
    }
}

impl<R:RealNumber> Tolerance for Complex<R> {
    const THRESHOLD:R=R::THRESHOLD;
}

impl<R:Clone+RealNumber> TrigonometricFunctions for Complex<R> {
    type Output=Self;

    fn sin(self) -> Self::Output {
        let sini=(Self::expi(self.clone())-Self::expi(-self.clone())).div2();
        <Self as TryDiv>::try_div(sini, Self::i()).unwrap()
    }

    fn cos(self) -> Self::Output {
        (Self::expi(self.clone())+Self::expi(-self.clone())).div2()
    }

    fn tan(self) -> Result<Self::Output,DivError> {
        <Self::Output as TryDiv>::try_div(self.clone().sin(), self.cos())
    }
}

impl<R:Zero> Origin for Complex<R> {
    fn origin() -> Self {
        Self::zero()
    }
}

impl<R:Clone+RealNumber> TryDiv<R> for Complex<R> {
    type Output=Self;
    type Error=DivError;
    fn is_divable_by(&self,rhs:&R) -> Result<(),DivError> {
        DivisionByZeroError::try_new(rhs)?;
        Ok(())
    }

    fn try_div(self,rhs:R) -> Result<Self::Output,DivError> {
        let rhs:Self=rhs.into();
        self.try_div(rhs)
    }
}

macro_rules! impl_const {
    ($name:ident, $const_name:ident) => {
        impl<R:$name+ConstZero> $name for Complex<R> {
            const $const_name:Self=Self(EnhancedArray::new([R::$const_name, R::ZERO]));
        }
    };
}
impl_const!(ConstZero, ZERO);
impl_const!(ConstOne, ONE);
impl_const!(ConstNonZero, NONZERO);


impl<R:Clone+RealNumber> TryNormalize for Complex<R> {}

impl<R:Clone+RealNumber> Sinc for Complex<R> {
    type Output=Self;
    fn denominator(self) -> Self {
        self
    }
}

#[test]
fn test_roots() {
    let a=c64::from(4.0);
    let roots=a.nth_roots(2);
    assert!(Tolerance::is_close_to(roots[0].clone(), 2.0));
    assert!(Tolerance::is_close_to(roots[1].clone(),-2.0));

    let a=c64::from(-4.0);
    let roots=a.nth_roots(2);
    assert!(Tolerance::is_close_to(roots[0].clone(), c64::new(0.0, 2.0)));
    assert!(Tolerance::is_close_to(roots[1].clone(), c64::new(0.0,-2.0)));
}


impl<R:RealNumber> Scalar for Complex<R> {
    type RealType = R;
    fn basis_over_r() -> Vec<Self> {
        vec![Self::one(), Self::i()]
    }
}



#[allow(non_camel_case_types)]
pub type c64=Complex<f64>;

impl TryInto<f64> for c64 {
    type Error=Self;
    fn try_into(self) -> Result<f64, Self> {
        if self.imag().is_zero() {
            Ok(self.into_real())
        } else {
            Err(self)
        }
    }
}

// impl Scalar for Complex<R> {
//   type RealType = f64;
//   const DIM_OVER_R:usize=2;
// }

// impl ComplexNumber for Complex<R> {
//     fn real(&self) -> Self::RealType {
//         self.real.clone()
//     }
//     fn imag(&self) -> Self::RealType {
//         self.imag.clone()
//     }
// }