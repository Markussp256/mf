use std::ops::Mul;

use algebra_traits::{operators::basic::{Add,Sub}, Nonnegative, TrySqrt};


use crate::generic::*;

use thiserror::Error;
use either::Either;

#[derive(Clone,Debug,Error,PartialEq,Eq)]
#[error("from a quantity with an odd dimension we can not take the square root")]
pub struct FromAQuantityWithAnOddDimensionWeCanNotTakeSquareroot;

pub struct GenericPhysQuant<F> {
    si: F,
    angl_dim: i32,
    time_dim: i32,
    leng_dim: i32,
    mass_dim: i32,
    curr_dim: i32,
    temp_dim: i32
}

impl<F> GenericPhysQuant<F> {
    fn new(si: F, dims: [i32; 6]) -> Self {
        Self {
            si,
            angl_dim: dims[0],
            time_dim: dims[1],
            leng_dim: dims[2],
            mass_dim: dims[3],
            curr_dim: dims[4],
            temp_dim: dims[5]
        }
    }

    fn dims(&self) -> [i32; 6] {
        [self.angl_dim,
         self.time_dim,
         self.leng_dim,
         self.mass_dim,
         self.curr_dim,
         self.temp_dim]
    }
}

impl<F:algebra_traits::Pow2<Output = F>> algebra_traits::Pow2 for GenericPhysQuant<F> {
    type Output=Self;
    fn pow2(self) -> Self {
        let dims=self.dims().map(|a|a*2);
        Self::new(<F as algebra_traits::Pow2>::pow2(self.si), dims)
    }
}



impl<F:algebra_traits::TrySqrt<Output=Nonnegative<F>,Error=E>,E> TrySqrt for GenericPhysQuant<F> {
    type Output=Self;
    type Error=Either<FromAQuantityWithAnOddDimensionWeCanNotTakeSquareroot,E>;

    fn is_sqrtable(&self) -> Result<(),Self::Error> {
        if self.dims()
               .into_iter()
               .any(|d|d & 2 == 1) {
                return Err(Either::Left(FromAQuantityWithAnOddDimensionWeCanNotTakeSquareroot));
        }
        self.si
            .is_sqrtable()
            .map_err(Either::Right)
    }

    fn try_sqrt(self) -> Result<Self,Self::Error> {
        self.is_sqrtable()?;
        let dims=self.dims().map(|a|a/2);
        let sr=self.si
            .try_sqrt()
            .map_err(Either::Right)?;
        Ok(Self::new(sr.into_signed(),dims))
    }
}

impl<F:Mul<Output=F>> std::ops::Mul for GenericPhysQuant<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let dims=self.dims().add(rhs.dims());
        Self::new(self.si * rhs.si, dims)
    }
}


impl<F:algebra_traits::TryDiv<Output=F,Error=E>,E>  algebra_traits::TryDiv for GenericPhysQuant<F> {
    type Output = Self;
    type Error  = E;
    fn is_divable_by(&self,rhs:&Self) -> Result<(),E> {
        self.si
            .is_divable_by(&rhs.si)
    }

    fn try_div(self, rhs: Self) -> Result<Self,E> {
        let dims=self.dims().sub(rhs.dims());
        self.si
            .try_div(rhs.si)
            .map(|si|Self::new(si, dims))
    }
}

macro_rules! impl_from_try_from {
    ($id:ident) => {
        impl<F> From<$id<F>> for GenericPhysQuant<F> where $id<F> : $crate::PhysicalQuantity<F> {
            fn from(value:$id<F>) -> Self {
                Self::new(<$id<F> as $crate::PhysicalQuantity<F>>::si(value),
                          <$id<F> as $crate::PhysicalQuantity<F>>::dims())
            }
        }

        impl<F> TryFrom<GenericPhysQuant<F>> for $id<F> where $id<F> : $crate::PhysicalQuantity<F> {
            type Error=String;
            fn try_from(gpq:GenericPhysQuant<F>) -> Result<Self, Self::Error> {
                let target_dims=<Self as $crate::PhysicalQuantity<F>>::dims();
                match gpq.dims() == target_dims {
                    true => Ok(<Self as $crate::PhysicalQuantity<F>>::from_si(gpq.si)),
                    false => Err(format!("Physical dimensions of the target type are {:?} whereas the physical dimensions of the origin are {:?}",target_dims,gpq.dims()))
                }
            }
        }
    }
}


impl_from_try_from!(Length);
impl_from_try_from!(Area);
impl_from_try_from!(Speed);
impl_from_try_from!(Acceleration);
impl_from_try_from!(Jerk);
impl_from_try_from!(Angle);
impl_from_try_from!(AngularSpeed);
impl_from_try_from!(AngularAcceleration);
impl_from_try_from!(AngularJerk);
impl_from_try_from!(Duration);
