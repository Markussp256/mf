pub mod display;

pub mod linear_container_dynamic;
pub use linear_container_dynamic::LinearContainerDynamic;

pub mod linear_container_mut;
pub use linear_container_mut::LinearContainerMut;

pub mod linear_container_try_construct;
pub use linear_container_try_construct::*;

pub mod linear_container_construct_error;
pub use linear_container_construct_error::LinearContainerConstructError;

pub mod linear_container_construct;
pub use linear_container_construct::LinearContainerConstruct;

pub mod linear_container_view;
pub use linear_container_view::LinearContainerView;

pub mod linear_container;
pub use linear_container::LinearContainer;


pub trait LinearContainerSized : LinearContainerTryConstruct { const N:usize; }
impl<T,const N:usize> LinearContainerSized for [T;N] { const N:usize=N; }

pub trait LinearContainerStatic<const N:usize> : LinearContainerTryConstruct {}
impl<T, const N:usize> LinearContainerStatic<N> for [T;N] { }