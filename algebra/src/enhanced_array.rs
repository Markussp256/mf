use std::ops::{Add,Sub};
use num_traits::Zero;
use crate::{EnhancedContainer, EnhancedVec};
use container_traits::for_static::IntoArray;

use algebra_traits::{FiniteDimensionalVectorspace, TryDiv, Vectorspace1d};
pub type EnhancedArray<T, const N:usize>=EnhancedContainer<[T;N]>;

impl<T,const N:usize> IntoArray<T,N> for EnhancedArray<T,N>{}

impl<T,const N:usize> Into<[T;N]> for EnhancedArray<T,N> {
    fn into(self) -> [T;N] {
        self.into_array()
    }
}

utils::try_from_via!(impl<T, const N:usize> TryFrom<Vec<T>>         for EnhancedArray<T,N>, via [T;N]);
utils::try_from_via!(impl<T, const N:usize> TryFrom<EnhancedVec<T>> for EnhancedArray<T,N>, via [T;N]);
utils::    into_via!(impl<T, const N:usize> Into<Vec<T>>            for EnhancedArray<T,N>, via [T;N]);

macro_rules! add_sub {
    ($tr:ident,$fn:ident) => {
        impl<T:$tr<T2,Output=TR>,T2,TR, const N:usize> $tr<EnhancedArray<T2,N>> for EnhancedArray<T,N> {
            type Output = EnhancedArray<TR,N>;
            fn $fn(self, rhs: EnhancedArray<T2,N>) -> Self::Output {
                EnhancedArray::new(<[T;N] as algebra_traits::operators::basic::$tr<[T2;N]>>::$fn(self.into_array(),rhs.into_array()))
            }
        }

        paste::paste!(
            impl<T,T2, const N:usize> std::ops::[<$tr Assign>]<EnhancedArray<T2,N>> for EnhancedArray<T,N>
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

impl<T:Zero,const N:usize> Zero for EnhancedArray<T,N> {
    fn zero() -> Self {
        Self::new(std::array::from_fn(|_|T::zero()))
    }

    fn is_zero(&self) -> bool {
        self.as_ref()
            .iter()
            .all(T::is_zero)
    }
}

impl<T : Vectorspace1d+TryDiv<Output=F>,
     F : Clone, const N:usize> FiniteDimensionalVectorspace<F,N> for EnhancedArray<T,N> {}