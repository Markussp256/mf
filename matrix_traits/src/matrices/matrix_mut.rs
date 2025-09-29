use container_traits::ContainerMut;
use super::MatrixView;

type U2=(usize,usize);

pub trait MatrixMut : ContainerMut<U2> + MatrixView {}

impl<M:MatrixView+ContainerMut<U2>> MatrixMut for M {}
