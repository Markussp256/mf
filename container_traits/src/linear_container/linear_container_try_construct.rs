use crate::{AnyFromVec, ContainerTryConstruct};
use super::{LinearContainer, LinearContainerConstructError};

pub trait LinearContainerTryConstruct<E=LinearContainerConstructError>
    : LinearContainer
      +ContainerTryConstruct<usize,E>
      +AnyFromVec<Self::T,E> {}


impl<E,
     C : LinearContainer
        +ContainerTryConstruct<usize,E>
        +AnyFromVec<C::T,E>> LinearContainerTryConstruct<E> for C {}