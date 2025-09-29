use crate::{ContainerView, IntoIndexedIter, IntoIter, TryIntoElement};


pub trait Container<Index>
          : ContainerView  <Index>
           +IntoIndexedIter<Index,Self::T>
           +IntoIter       <      Self::T>
           +TryIntoElement <Index,Self::T> {}


impl<T,Index,
     C : ContainerView  <Index,T=T>
        +IntoIndexedIter<Index,T>
        +IntoIter       <      T>
        +TryIntoElement <Index,T>> Container<Index> for C {}