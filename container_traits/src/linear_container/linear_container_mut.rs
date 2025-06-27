use super::LinearContainer;

use crate::ContainerMut;

pub trait LinearContainerMut
    : LinearContainer
     +ContainerMut<usize> {}

impl<C : LinearContainer
        +ContainerMut<usize>> LinearContainerMut for C {}