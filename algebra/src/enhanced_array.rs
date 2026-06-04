use std::ops::{Add,Sub};
use num_traits::Zero;
use crate::{EnhancedContainer, EnhancedVec};
use container_traits::{IntoInner, LenNotEqualToRequiredLenError};
use generic_array::{ArrayLength,GenericArray, IntoArrayLength};
use typenum::Const;

// use algebra_traits::{FiniteDimensionalVectorspace, TryDiv, Vectorspace1d};
pub type EnhancedArray<T, N:ArrayLength>=EnhancedContainer<GenericArray<T,N>>;

// impl<T,N : ArrayLength> IntoArray<T,N> for EnhancedArray<T,N>{}

impl<T,N:ArrayLength> EnhancedArray<T,N> {
    pub const fn from_array<const U:usize>(value:[T;U]) -> Self
    where Const<U>: IntoArrayLength<ArrayLength = N> {
        Self::new(GenericArray::from_array(value))
    }

    pub fn into_array<const U:usize>(self) -> [T;U]
    where Const<U>: IntoArrayLength<ArrayLength = N> {
        self.into_inner()
            .into_array::<U>()
    }
}


impl<T,N : ArrayLength> Into<GenericArray<T,N>> for EnhancedArray<T,N> {
    fn into(self) -> GenericArray<T,N> {
        self.into_inner()
    }
}

impl<T, N : ArrayLength> TryFrom<Vec<T>> for EnhancedArray<T,N> {
    type Error = LenNotEqualToRequiredLenError;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let len=value.len();
        LenNotEqualToRequiredLenError::try_new(N::to_usize(), len)?;
        Ok(EnhancedArray::new(GenericArray::<T,N>::try_from_iter(value.into_iter().take(N::to_usize())).unwrap()))
    }
}

impl<T, N : ArrayLength> TryFrom<EnhancedVec<T>> for EnhancedArray<T,N> {
    type Error=LenNotEqualToRequiredLenError;
    fn try_from(value: EnhancedVec<T>) -> Result<Self, Self::Error> {
        let v:Vec<T>=value.into();
        Self::try_from(v)
    }
}

// utils::try_from_via!(impl<T, N : ArrayLength> TryFrom<EnhancedVec<T>> for EnhancedArray<T,N>, via Vec<T>);
// utils::    into_via!(impl<T, N : ArrayLength> Into<Vec<T>>            for EnhancedArray<T,N>, via GenericArray<T,N>);

impl<T,N:ArrayLength> Into<Vec<T>> for EnhancedArray<T,N> {
    fn into(self) -> Vec<T> {
        self.into_iter()
            .collect()
    }
}


macro_rules! add_sub {
    ($tr:ident,$fn:ident) => {
        impl<T:$tr<T2,Output=TR>,T2,TR, N : ArrayLength> $tr<EnhancedArray<T2,N>> for EnhancedArray<T,N> {
            type Output = EnhancedArray<TR,N>;
            fn $fn(self, rhs: EnhancedArray<T2,N>) -> Self::Output {
                EnhancedArray::new(<GenericArray<T,N> as algebra_traits::operators::basic::$tr<GenericArray<T2,N>>>::$fn(self.into_inner(),rhs.into_inner()))
            }
        }

        paste::paste!(
            impl<T,T2, N : ArrayLength> std::ops::[<$tr Assign>]<EnhancedArray<T2,N>> for EnhancedArray<T,N>
            where Self : Clone+$tr<EnhancedArray<T2,N>,Output=Self> {
                fn [<$fn _assign>](& mut self, rhs:EnhancedArray<T2,N>) {
                    *self=self.clone()
                              .$fn(rhs);
                }
            }
        );
    };
}
add_sub!(Add,add);
add_sub!(Sub,sub);

impl<T:Zero,N : ArrayLength> Zero for EnhancedArray<T,N> {
    fn zero() -> Self {
        Self::new(GenericArray::try_from_iter(std::iter::repeat_with( ||T::zero())).unwrap())
    }

    fn is_zero(&self) -> bool {
        self.as_ref()
            .iter()
            .all(T::is_zero)
    }
}

// impl<T : Vectorspace1d+TryDiv<Output=F>,
//      F : Clone,
//      N : ArrayLength+Unsigned> FiniteDimensionalVectorspace<F,N::USIZE> for [T,N] {}