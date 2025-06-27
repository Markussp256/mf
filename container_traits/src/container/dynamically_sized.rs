use crate::{for_dynamic::{Empty, OneElement}, Zeros};
use super::{ContainerConstruct, ContainerConstructError};

// these containers can have any size and no constraints on the elements

pub trait ContainerDynamicallySized<Index,E=ContainerConstructError<Index>>
    : ContainerConstruct<Index,E>
     +Zeros<Index,Self::T>
     +OneElement<Self::T>
     +Empty {}

impl<Index,E,T,
     C : ContainerConstruct<Index,E,T=T>
        +Zeros<Index,Self::T>
        +OneElement<T>
        +Empty> ContainerDynamicallySized<Index,E> for C {}