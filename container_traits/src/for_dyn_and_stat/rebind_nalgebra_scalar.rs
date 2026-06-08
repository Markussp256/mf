
//  same as rebind but with the nalgebra::Scalar constraint on the type T

use generic_array::{ArrayLength, GenericArray};

use crate::{AnyFromIterator, LinearContainerConstructError, IntoIter};

pub trait NAlgebraScalar
    : 'static
      +Clone
      +std::cmp::PartialEq
      +std::fmt::Debug {}
impl<T:'static+Clone+std::cmp::PartialEq+std::fmt::Debug> NAlgebraScalar for T {}

// suggested by ChatGPT to replace ChangeT and AnyFromIterator

pub trait RebindNAlgebraScalar<E> {
    type WithNAlgebraScalar<T : NAlgebraScalar> : IntoIter<T> + AnyFromIterator<T,E>;
}


impl<T> RebindNAlgebraScalar<LinearContainerConstructError> for Vec<T> {
    type WithNAlgebraScalar<T2:NAlgebraScalar>=Vec<T2>;
}

impl<T,const N:usize> RebindNAlgebraScalar<LinearContainerConstructError> for [T;N] {
    type WithNAlgebraScalar<T2:NAlgebraScalar>=[T2;N];
}

impl<T,N:ArrayLength> RebindNAlgebraScalar<LinearContainerConstructError> for GenericArray<T,N> {
    type WithNAlgebraScalar<T2:NAlgebraScalar>=GenericArray<T2,N>;
}