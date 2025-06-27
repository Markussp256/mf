use crate::{Get, IndexedIter, IntoIndexedIter, IntoIter, ItemT, Iter, NumberOfDegreesOfFreedom, OCTSize, Size, SizeFromORef, TryIntoElement};

pub trait Container<Index>
          : Sized
           +NumberOfDegreesOfFreedom<Self::T>
           +Size           <Index>
           +OCTSize        <Index>
           +SizeFromORef   <Index>
           +Get            <Index,Self::T>
           +IndexedIter    <Index,Self::T>
           +IntoIndexedIter<Index,Self::T>
           +IntoIter       <      Self::T>
           +Iter           <      Self::T>
           +ItemT
           +TryIntoElement <Index,Self::T> {}


impl<T,Index,
     C : Sized
        +NumberOfDegreesOfFreedom<T>
        +Size           <Index>
        +OCTSize        <Index>
        +SizeFromORef   <Index>  
        +Get            <Index,T>
        +IndexedIter    <Index,T>
        +IntoIndexedIter<Index,T>
        +IntoIter       <      T>
        +Iter           <      T>
        +ItemT              <T=T>
        +TryIntoElement <Index,T>> Container<Index> for C {}

// impl<T>               Container<T> for Vec<T> {}
// impl<T,const N:usize> Container<T> for [T;N]  {}
// impl<T>               Container for Vec<T> {
//     type T=T;
// }
// impl<T,const N:usize> Container for [T;N]  {
//     type T=T;
// }