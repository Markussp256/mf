use crate::{AnyFromIterator, ContainerConstructError, RebindNAlgebraScalar, TryFromFn, TryAccept, TryClosedMap};

use super::Container;

pub trait ContainerTryConstruct<Index,E=ContainerConstructError<Index>>
    : Container <Index>
     +AnyFromIterator <Self::T,E>
     +TryAccept <Index,Self::T,E>
     +TryFromFn <Index,Self::T,E>
     +RebindNAlgebraScalar    <E>
     +TryClosedMap    <Self::T,E> {}

impl<Index,E,T,
     C : Container       <Index,T=T>
        +AnyFromIterator       <T,E>
        +TryAccept       <Index,T,E>
        +TryFromFn       <Index,T,E>
        +RebindNAlgebraScalar    <E>
        +TryClosedMap          <T,E>> ContainerTryConstruct<Index,E> for C {}