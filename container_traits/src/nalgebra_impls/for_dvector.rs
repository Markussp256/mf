use nalgebra::{Const, DVector, Dyn, OMatrix, RowDVector, Scalar};
use crate::{for_dynamic::*, for_dyn_and_stat::*, IntoVec, ChangeLen, IndexOutOfBoundsError};
use num_traits::Zero;

use crate::ContainerConstructError;

type U2=(usize,usize);
type CCE=ContainerConstructError<U2>;


macro_rules! impl_vector_owned {
    ($t:ty) => {

        impl<T:Scalar> Concat for $t {
            fn concat(self, rhs:Self) -> Self {
                Self::from_vec(
                    self.into_vec().concat(
                    rhs .into_vec()))
            }
        }

        impl<T:Scalar> Empty for $t {
            fn empty() -> Self {
                <$t>::from_vec(Vec::new())
            }
        }

        impl<T:Scalar> OneElement<T> for $t {
            fn one_element(t:T) -> Self {
                <$t>::from_vec(vec![t])
            }
        }


        impl<T:Scalar+Zero> TryPutAt<usize,T> for $t {
            fn try_put_at(len:usize, index:usize, t:T) -> Result<Self,IndexOutOfBoundsError<usize>> {
                Vec::<T>::try_put_at(len,index,t)
                    .map(|v|Self::from_vec(v))
            }
        }

        impl<T:Scalar> FromElement<usize,T> for $t {
            fn from_element(len:usize,t:T) -> Self {
                <$t>::from_element(len, t)
            }
        }

        impl<T:Scalar> FromFn<usize,T> for $t {
            fn from_fn(len:usize,f: impl Fn(usize)-> T) -> Self {
                let iter=(0..len).map(f);
                <Self as crate::AnyFromIterator<T,CCE>>::any_from_iter(None, iter).unwrap()
            }
        }

        impl<T:Scalar> Zeros<usize,T> for $t {
            fn zeros(len:usize) -> Self where T:Zero {
                <$t>::zeros(len)
            }
        }

        impl<T:Scalar> FromVec<T> for $t {
            fn from_vec(v:Vec<T>) -> Self {
                <$t>::from_vec(v)
            }
        }


    };
}
impl_vector_owned!(OMatrix<T,Dyn,Const<1>>);
impl_vector_owned!(OMatrix<T,Const<1>,Dyn>);



// can not implement this more general because of conflicting implement for 1x1 smatrix
impl<T : Scalar> ChangeLen for DVector<T> {
    type Output<const L:usize> = nalgebra::SVector<T,L>;
}

impl<T : Scalar> ChangeLen for RowDVector<T> {
    type Output<const L:usize> = nalgebra::RowSVector<T,L>;
}