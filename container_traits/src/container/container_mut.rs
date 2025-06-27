use super::Container;

use crate::{IndexedIterMut, IterMut, GetMut};

pub trait ContainerMut<Index>
    : Container     <Index>
     +IndexedIterMut<Index,Self::T>
     +IterMut       <      Self::T>
     +GetMut        <Index,Self::T> {}

impl<T,Index,
     C : Container     <Index,T=T>
        +IndexedIterMut<Index,T>
        +IterMut       <      T>
        +GetMut        <Index,T>> ContainerMut<Index> for C {}