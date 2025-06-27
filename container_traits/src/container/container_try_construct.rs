use crate::{AnyFromIterator, ContainerConstructError, TryFromFn, TryAccept, TryClosedMap};

use super::Container;

pub trait ContainerTryConstruct<Index,E=ContainerConstructError<Index>>
    : Container <Index>
     +TryAccept <Index,Self::T,E>
     +TryFromFn <Index,Self::T,E>
     +AnyFromIterator <Self::T,E>
     +TryClosedMap    <Self::T,E> {}

impl<Index,E,T,
     C : Container <Index,T=T>
        +TryAccept <Index,T,E>
        +TryFromFn <Index,T,E>
        +AnyFromIterator <T,E>
        +TryClosedMap    <T,E>> ContainerTryConstruct<Index,E> for C {}