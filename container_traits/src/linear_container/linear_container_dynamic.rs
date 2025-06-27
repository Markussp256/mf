use crate::{TryInsert, Pop, Push, TryRemove};

use super::{LinearContainerConstruct, LinearContainerConstructError};

pub trait LinearContainerDynamic<E=LinearContainerConstructError>
    : LinearContainerConstruct<E>
     +Push     <Self::T>
     +Pop      <Self::T>
     +Extend   <Self::T>
     +TryInsert<Self::T>
     +TryRemove<Self::T> {}

impl<T, E,
     C : LinearContainerConstruct<E,T=T>
        +Push<T>
        +Pop<T>
        +Extend<T>
        +TryInsert<T>
        +TryRemove<T>> LinearContainerDynamic<E> for C {}