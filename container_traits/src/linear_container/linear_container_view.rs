use crate::{ContainerView,First,Last,Len,LenFromORef};

pub trait LinearContainerView : ContainerView<usize>
        +Len
        +LenFromORef
        +First  <Self::T>
        +Last   <Self::T> {}

impl<C : ContainerView<usize>
        +Len
        +LenFromORef
        +First<C::T>
        +Last<C::T>> LinearContainerView for C {}


// impl<T>               Container<T> for Vec<T> {}
// impl<T,const N:usize> Container<T> for [T;N]  {}
// impl<T>               Container for Vec<T> {
//     type T=T;
// }
// impl<T,const N:usize> Container for [T;N]  {
//     type T=T;
// }