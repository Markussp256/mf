use crate::{Container,LinearContainerView, IntoVec};

pub trait LinearContainer : Container<usize>
        +IntoVec<Self::T> {}

impl<C : Container<usize>
        +LinearContainerView
        +IntoVec<C::T>> LinearContainer for C {}


// impl<T>               Container<T> for Vec<T> {}
// impl<T,const N:usize> Container<T> for [T;N]  {}
// impl<T>               Container for Vec<T> {
//     type T=T;
// }
// impl<T,const N:usize> Container for [T;N]  {
//     type T=T;
// }