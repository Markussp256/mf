use crate::{ContainerView, IntoIterIndexed, IntoIter, TryIntoElement};


pub trait Container<Index>
          : ContainerView  <Index>
           +IntoIterIndexed<Index,Self::T>
           +IntoIter       <      Self::T>
           +TryIntoElement <Index,Self::T> {
}


impl<T,Index,
     C : ContainerView  <Index,T=T>
        +IntoIterIndexed<Index,T>
        +IntoIter       <      T>
        +TryIntoElement <Index,T>> Container<Index> for C {}