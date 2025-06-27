use std::fmt::Display;

use algebra_traits::*;
use num_traits::{Zero,One,Inv};
use algebra_traits::operators::basic::Add;

use container_traits::{for_static::{IntoArray,TryFromIterator, TryFromParameters}, IntoParameters, LinearContainerConstructError};

use crate::EnhancedArray;

#[derive(Clone,
         Debug,
         PartialEq,
         algebra_derive::IsAZero,
         algebra_derive::Norm,
         algebra_derive::NormSquared,
         algebra_derive::AdditiveGroup,
         container_derive::TryFromIterator,
         container_derive::IntoIterator,
         container_derive::Iter,
         container_derive::NumberOfDegreesOfFreedom,
)]
pub struct Quaternion<T:'static>(EnhancedArray<T,4>);

impl<T:Display> Display for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width=f.width().unwrap_or(6);
        let precision=f.precision().unwrap_or(4);
        write!(f,
              "real: {:width$.precision$}, imag: [{:width$.precision$}, {:width$.precision$}, {:width$.precision$}]",
               self.real(),
               self.imag()[0],
               self.imag()[1],
               self.imag()[2],
               width=width,
               precision=precision)
    }
}


impl<T> Quaternion<T> {
    pub fn new(real:T,imag:[T;3]) -> Self {
        let [i0, i1, i2]=imag;
        Self(EnhancedArray::new([real, i0, i1, i2]))
    }

    pub fn real(&self) -> &T {
        &self.0[0]
    }

    pub fn into_real(self) -> T {
        let [real, _, _, _]=self.0.into_array();
        real
    }

    pub fn imag(&self) -> [&T;3] {
        [1,2,3].map(|i|&self.0[i])
    }

    pub fn into_imag(self) -> [T;3] {
        self.into_real_imag().1
    }

    pub fn into_real_imag(self) -> (T, [T;3]) {
        let [real,i0,i1,i2]=self.0.into_array();
        (real, [i0,i1,i2])
    }

    pub fn conjugate(self) -> Self where T:std::ops::Neg<Output=T> {
        let (real, imag)=self.into_real_imag();
        Self::new(real, imag.map(<T as std::ops::Neg>::neg))
    }
}

impl<T:ScalarMul<T>> std::ops::Mul<T> for Quaternion<T> {
    type Output=Self;
    fn mul(self, t:T) -> Self {
        Self(self.0.scalar_mul(&t))
    }
}

impl<T:Clone+Scalar> TryDiv for Quaternion<T> {
    type Output=Self;
    type Error=DivError;

    fn is_divable_by(&self,rhs:&Self) -> Result<(),DivError> {
        DivisionByZeroError::try_new(rhs)?;
        Ok(())
    }

    fn try_div(self, rhs:Self) -> Result<Self,DivError> {
        Ok(self*rhs.try_inv()?)
    }
}

impl<T:Clone+Scalar> TryInv for Quaternion<T> {
    type Output=Self;
    type Error=InvError;

    fn is_invertible(&self) -> Result<(),InvError> {
        DivisionByZeroError::try_new(self)?;
        Ok(())
    }

    fn try_inv(self) -> Result<Self,InvError> {
        self.is_invertible()?;
        Ok(NonZero::try_new(
           self.clone()
               .norm_squared()
               .into_signed()
               .into())
            .map(|norm_squared: NonZero<T>|
            Self(self.conjugate().0 / norm_squared)).unwrap())
    }
}

impl<T:Zero> From<T> for Quaternion<T> {
    fn from(value: T) -> Self {
        Self::new(value, std::array::from_fn(|_|T::zero()))
    }
}

#[cfg(feature = "nalgebra_support")]
impl From<nalgebra::Quaternion<f64>> for Quaternion<f64> {
    fn from(value: nalgebra::Quaternion<f64>) -> Self {
        Self::new(value.w,
                  [value.i, value.j, value.k])
    }
}

#[cfg(feature = "nalgebra_support")]
impl Into<nalgebra::Quaternion<f64>> for Quaternion<f64> {
    fn into(self) -> nalgebra::Quaternion<f64> {
        let (real, imag)=self.into_real_imag();
        nalgebra::Quaternion::<f64>::new(real,
                                         imag[0], 
                                         imag[1],
                                         imag[2])
    }
}

impl<T:Clone+Field+Scalarproduct<ScProdT=T>+ScalarMul<T>> std::ops::Mul for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs:Self) -> Self {
        let (real, imag)=self.into_real_imag();
        let (rhs_real, rhs_imag)=rhs.into_real_imag();
        Self::new(real.clone() * rhs_real.clone()-Scalarproduct::scalar_product(imag.clone(), rhs_imag.clone()),
                      imag.clone().scalar_mul(&rhs_real)
                  .add(rhs_imag.clone().scalar_mul(&real))
                  .add(imag.cross_product(rhs_imag)))
    }
}

impl<T:Clone+Field+Scalarproduct<ScProdT=T>+ScalarMul<T>> One for Quaternion<T> {
    fn one() -> Self {
        T::one().into()
    }
}

// because Quaternion does not implement std::ops::Div and num_traits::Inv we can not
// derive MultiplicativeGroup

#[derive(Clone,
         Debug,
         PartialEq,
         container_derive::IntoIterator,
         derive_more::Into,
         algebra_derive::Conjugate,
         algebra_derive::One,
         algebra_derive::ClosedMul,
         algebra_derive::Neg)]

pub struct UnitQuaternion<T:'static>(Quaternion<T>);


impl<T> UnitQuaternion<T> {
    pub fn quaternion(&self) -> &Quaternion<T> {
        &self.0
    }

    pub fn real(&self) -> &T {
        self.0.real()
    }

    pub fn imag(&self) -> [&T;3] {
        self.0.imag()
    }
}

impl<T:Clone+Scalar> Inv for UnitQuaternion<T> {
    type Output=Self;
    fn inv(self) -> Self {
        Self(self.0
                 .try_inv().unwrap())
    }
}

impl<T:Clone+Scalar> std::ops::Div for UnitQuaternion<T> {
    type Output=Self;
    fn div(self, rhs:Self) -> Self {
        self * rhs.inv()
    }
}

impl<T:Clone+Scalar> TryFrom<Quaternion<T>> for UnitQuaternion<T>  {
    type Error = Nonnegative<T::RealType>;
    fn try_from(value: Quaternion<T>) -> Result<Self, Self::Error> {
        let norm=value.clone().norm();
        if  norm.clone().is_close_to_one() {
            Ok(Self(value))
        } else {
            Err(norm)
        }
    }
}

// quaternion where constant factor are considered equal
#[derive(Clone,
         Debug,
         algebra_derive::MultiplicativeGroup,
         container_derive::IntoIterator,
         derive_more::From)]
pub struct ProjectiveQuaternion<T>(UnitQuaternion<T>)  where T:'static;


impl<T:Clone+RealNumber> IntoParameters<T> for ProjectiveQuaternion<T> {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=T> {
        let (real,imag)=self.0.0.into_real_imag();
        let pos=real > T::zero();
        imag.into_iter()
            .map(move |v| if pos { v } else { -v })
    }
}

impl<T:Clone+RealNumber> TryFromParameters<T,LinearContainerConstructError> for ProjectiveQuaternion<T> {
    fn try_take_away<I: Iterator<Item = T>>(iter: &mut I) -> Result<Self,LinearContainerConstructError> {
        let imag:[T;3]=<[T;3] as TryFromIterator<T,LinearContainerConstructError>>::try_from_iter(iter)?;
        let norm=imag.clone().norm().into_signed();
        if norm > T::one() {
            return Err(LinearContainerConstructError::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer);
        }
        let real=(T::one()-norm.pow2()).try_sqrt().unwrap().into_signed();
        Ok(Self::try_from_real_imag(real, imag)
            .ok().unwrap())
    }

    container_traits::try_from_parameters_impl!(T);
}


impl<T> ProjectiveQuaternion<T> {
    // returns one of the two unit quaternions that represent the Projective space
    pub fn unit_quaternion(&self) -> &UnitQuaternion<T> {
       &self.0
    }

    pub fn quaternion(&self) -> &Quaternion<T> {
        self.0
            .quaternion()
    }
}

// #[cfg(test)]
// fn test_norm<T:Clone+RealNumber>(q:Quaternion<T>) -> T {
//     <Quaternion<T> as Norm>::norm(q).into_signed()
// }

impl<T:RealNumber> TryDiv<T> for Quaternion<T> {
    type Output = Self;
    type Error=DivError;

    fn is_divable_by(&self,rhs:&T) -> Result<(),DivError> {
        DivisionByZeroError::try_new(rhs)?;
        Ok(())
    }

    fn try_div(self,rhs:T) -> Result<Self::Output,DivError> {
        <Self as TryDiv>::try_div(self, rhs.into())
    }
}

impl<T:RealNumber> TryNormalize for Quaternion<T> {}

impl<T:Clone+RealNumber> TryFrom<Quaternion<T>> for ProjectiveQuaternion<T> {
    type Error=Quaternion<T>;
    fn try_from(value: Quaternion<T>) -> Result<Self, Self::Error> {
        match value.clone().try_normalize() {
            Ok(uq) => Ok(Self(uq.1)),
            Err(_) => Err(value)
        }
    }
}

impl<T:Clone+RealNumber> ProjectiveQuaternion<T> {
    pub fn try_from_real_imag(real:T, imag:[T;3]) -> Result<Self,(T,[T;3])> {
        Quaternion::new(real, imag)
            .try_into()
            .map_err(|e: Quaternion<T>|e.into_real_imag())
    }
}

#[test]
fn test_normalize() {
    let q=Quaternion::new(0.9486, [-0.316,0.0,0.0]);
    assert!(q.try_normalize::<Quaternion<f64>>().is_ok())
}

#[test]
fn test_into_proj_quat() {
    let real=0.9486;
    let imag=[-0.316, 0.0, 0.0];
    let q=Quaternion::new(real, imag);
    let qn:Quaternion<f64>=q.clone().try_normalize().unwrap().1;
    let res=UnitQuaternion::try_from(qn);
    assert!(res.is_ok());
    assert!(ProjectiveQuaternion::try_from(q).is_ok());
    assert!(ProjectiveQuaternion::try_from_real_imag(real, imag).is_ok())
}