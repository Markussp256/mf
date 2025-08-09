use crate::{TryFromVec, ContainerTryConstruct};
use super::{LinearContainer, LinearContainerConstructError};

pub trait LinearContainerTryConstruct<E=LinearContainerConstructError>
    : LinearContainer
      +ContainerTryConstruct<usize,E>
      +TryFromVec<Self::T,E> {}


impl<E,
     C : LinearContainer
        +ContainerTryConstruct<usize,E>
        +TryFromVec<C::T,E>> LinearContainerTryConstruct<E> for C {}