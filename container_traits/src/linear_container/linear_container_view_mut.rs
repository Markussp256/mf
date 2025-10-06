use super::LinearContainerView;

use crate::ContainerViewMut;

pub trait LinearContainerViewMut
    : LinearContainerView
     +ContainerViewMut<usize> {}

impl<C : LinearContainerView
        +ContainerViewMut<usize>> LinearContainerViewMut for C {}