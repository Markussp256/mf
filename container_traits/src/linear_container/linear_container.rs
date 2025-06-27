use crate::{Container,First,IntoVec,Last,Len,LenFromORef};

pub trait LinearContainer : Container<usize>
        +Len
        +LenFromORef
        +IntoVec<Self::T>
        +First  <Self::T>
        +Last   <Self::T> {}

impl<C : Container<usize>
        +Len
        +LenFromORef
        +IntoVec<C::T>
        +First<C::T>
        +Last<C::T>> LinearContainer for C {}


// impl<T>               Container<T> for Vec<T> {}
// impl<T,const N:usize> Container<T> for [T;N]  {}
// impl<T>               Container for Vec<T> {
//     type T=T;
// }
// impl<T,const N:usize> Container for [T;N]  {
//     type T=T;
// }