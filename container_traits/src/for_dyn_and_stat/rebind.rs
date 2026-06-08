
use generic_array::{ArrayLength, GenericArray};

use crate::{AnyFromIterator, IntoIter, LinearContainerConstructError, RebindNAlgebraScalar};

// suggested by ChatGPT to replace ChangeT and AnyFromIterator

pub trait Rebind<E> : RebindNAlgebraScalar<E> {
    type With<T> : IntoIter<T> + AnyFromIterator<T,E>;
}


impl<T> Rebind<LinearContainerConstructError> for Vec<T> {
    type With<T2>=Vec<T2>;
}

impl<T,const N:usize> Rebind<LinearContainerConstructError> for [T;N] {
    type With<T2>=[T2;N];
}

impl<T,N:ArrayLength> Rebind<LinearContainerConstructError> for GenericArray<T,N> {
    type With<T2>=GenericArray<T2,N>;
}