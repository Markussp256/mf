// only use this if its guaranteed that data lives in some multidimensional box

use generic_array::{ArrayLength,GenericArray};
use typenum::Unsigned;


pub trait Size<Index> {
    const SIZE:Index;
}

impl<T,const N:usize> Size<usize> for [T;N] {
    const SIZE:usize=N;
}

impl<T,N:ArrayLength+Unsigned> Size<usize> for GenericArray<T,N> {
    const SIZE:usize = N::USIZE;
}

pub trait Len : Size<usize> {
    const LEN:usize;
}

impl<S:Size<usize>> Len for S {
    const LEN:usize=<S as Size<usize>>::SIZE;
}