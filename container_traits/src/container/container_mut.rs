use super::ContainerView;

use crate::{IndexedIterMut, IterMut, GetMut};

pub trait ContainerMut<Index>
    : ContainerView <Index>
     +IndexedIterMut<Index,Self::T>
     +IterMut       <      Self::T>
     +GetMut        <Index,Self::T> {}

impl<T,Index,
     C : ContainerView <Index,T=T>
        +IndexedIterMut<Index,T>
        +IterMut       <      T>
        +GetMut        <Index,T>> ContainerMut<Index> for C {}