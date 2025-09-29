use super::LinearContainerView;

use crate::ContainerMut;

pub trait LinearContainerMut
    : LinearContainerView
     +ContainerMut<usize> {}

impl<C : LinearContainerView
        +ContainerMut<usize>> LinearContainerMut for C {}