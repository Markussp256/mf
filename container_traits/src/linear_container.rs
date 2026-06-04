// 1d container

use typenum::Unsigned;

pub mod display;

pub mod linear_container_construct_error;
use generic_array::{ArrayLength, GenericArray};
pub use linear_container_construct_error::LinearContainerConstructError;

pub mod linear_container_construct;
pub use linear_container_construct::LinearContainerConstruct;

pub mod linear_container_dynamic;
pub use linear_container_dynamic::LinearContainerDynamic;

pub mod linear_container_try_construct;
pub use linear_container_try_construct::*;

pub mod linear_container_view_mut;
pub use linear_container_view_mut::LinearContainerViewMut;

pub mod linear_container_view;
pub use linear_container_view::LinearContainerView;

pub mod linear_container;
pub use linear_container::LinearContainer;

pub trait LinearContainerSized : LinearContainerTryConstruct { const N:usize; }
impl<T,N:ArrayLength+Unsigned> LinearContainerSized for GenericArray<T,N> { const N:usize=N::USIZE; }
impl<T,const N:usize> LinearContainerSized for [T;N] { const N:usize=N; }


pub trait LinearContainerStatic<const N:usize> : LinearContainerTryConstruct {}
impl<T,const N:usize> LinearContainerStatic<N> for [T;N] {}
