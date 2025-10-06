use super::ContainerView;

use crate::{GetMut, IterMut, IterMutIndexed};

pub trait ContainerViewMut<Index>
    : ContainerView <Index>
     +IterMutIndexed<Index,Self::T>
     +IterMut       <      Self::T>
     +GetMut        <Index,Self::T> {}

impl<T,Index,
     C : ContainerView <Index,T=T>
        +IterMutIndexed<Index,T>
        +IterMut       <      T>
        +GetMut        <Index,T>> ContainerViewMut<Index> for C {}