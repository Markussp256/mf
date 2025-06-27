use crate::{FromVec,ContainerConstruct};

use super::{LinearContainerTryConstruct, LinearContainerConstructError};


pub trait LinearContainerConstruct<E=LinearContainerConstructError>
    : LinearContainerTryConstruct<E>
     +ContainerConstruct<usize, E>
     +FromVec<Self::T> {}

impl<T, E,
     C : LinearContainerTryConstruct<E,T=T>
        +ContainerConstruct<usize,E,T=T>
        +FromVec<T>> LinearContainerConstruct<E> for C {}