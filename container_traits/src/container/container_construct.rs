use crate::{ClosedMap, ContainerConstructError, FromFn};
use super::ContainerTryConstruct;

// these containers have no constraints on the elements

pub trait ContainerConstruct<Index,E=ContainerConstructError<Index>>
    : ContainerTryConstruct<Index,E>
     +FromFn   <Index, Self::T>
     +ClosedMap<       Self::T> {}

impl<Index,E,T,
     C : ContainerTryConstruct<Index,E,T=T>
        +FromFn<Index,T>
        +ClosedMap<T>> ContainerConstruct<Index,E> for C {}