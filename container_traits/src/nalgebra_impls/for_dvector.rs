use nalgebra::{Scalar, DVector,RowDVector};
use crate::{for_dynamic::*, for_dyn_and_stat::*, IntoVec, IndexOutOfBoundsError};
use num_traits::{Zero,One};

use crate::ContainerConstructError;

type U2=(usize,usize);
type CCE=ContainerConstructError<U2>;

macro_rules! impl_vector {
    ($name:ident) => {

        impl<T:Scalar> Concat for $name<T> {
            fn concat(self, rhs:Self) -> Self {
                Self::from_vec(
                    self.into_vec().concat(
                    rhs .into_vec()))
            }
        }

        impl<T:Scalar> Empty for $name<T> {
            fn empty() -> Self {
                $name::from_vec(Vec::new())
            }

            fn is_empty(&self) -> bool {
                $name::is_empty(&self)
            }
        }

        impl<T : Scalar> OCTSize<usize> for $name<T> {
            const OCTSIZE:Option<usize>=None;
        }

        impl<T:Scalar> OneElement<T> for $name<T> {
            fn one_element(t:T) -> Self {
                $name::from_vec(vec![t])
            }
        }

        impl<T:Scalar> FromElement<usize,T> for $name<T> {
             fn from_element(len:usize,t:T) -> Self {
                $name::from_element(len, t)
             }
        }

        impl<T:Scalar> FromFn<usize,T> for $name<T> {
            fn from_fn(len:usize,f: impl Fn(usize)-> T) -> Self {
                let iter=(0..len).map(f);
                <Self as crate::AnyFromIterator<T,CCE>>::any_from_iter(None, iter).unwrap()
            }
        }

        impl<T:Scalar> FromVec<T> for $name<T> {
            fn from_vec(v:Vec<T>) -> Self {
                $name::from_vec(v)
            }
        }

        impl<T:Scalar+Zero> TryPutAt<usize,T> for $name<T> {
            fn try_put_at(len:usize, index:usize, t:T) -> Result<Self,IndexOutOfBoundsError<usize>> {
                Vec::<T>::try_put_at(len,index,t)
                    .map(|v|Self::from_vec(v))
            }
        }

        impl<T:Scalar+Zero+One> StandardBasis for $name<T> {
            fn try_standard_basis_element(len:usize, index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
                <Self as TryPutAt<usize,T>>::try_put_at(len,index,T::one())
            }
        }

        impl<T:Scalar> Zeros<usize,T> for $name<T> {
            fn zeros(len:usize) -> Self where T:Zero {
                $name::zeros(len)
            }
        }

    };
}
impl_vector!(DVector);
impl_vector!(RowDVector);