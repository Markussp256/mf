use crate::{Get, IsEmpty, ItemT, Iter, IterIndexed, NumberOfDegreesOfFreedom, OCTSize, Size, SizeFromORef};

pub trait ContainerView<Index>
          : Sized
           +NumberOfDegreesOfFreedom<Self::T>
           +Size           <Index>
           +OCTSize        <Index>
           +SizeFromORef   <Index>
           +Get            <Index,Self::T>
           +IsEmpty
           +IterIndexed    <Index,Self::T>
           +Iter           <      Self::T>
           +ItemT {}

pub trait ContainerViewable<Index> : crate::Container<Index> {
    type Viewer<'a> where Self : 'a;
    fn as_view<'a>(&'a self) -> Self::Viewer<'a>;
}


impl<T,Index,
     C : Sized
        +NumberOfDegreesOfFreedom<T>
        +Size           <Index>
        +OCTSize        <Index>
        +SizeFromORef   <Index>  
        +Get            <Index,T>
        +IsEmpty
        +IterIndexed    <Index,T>
        +Iter           <      T>
        +ItemT              <T=T>> ContainerView<Index> for C {}

// impl<T>               Container<T> for Vec<T> {}
// impl<T,const N:usize> Container<T> for [T;N]  {}
// impl<T>               Container for Vec<T> {
//     type T=T;
// }
// impl<T,const N:usize> Container for [T;N]  {
//     type T=T;
// }