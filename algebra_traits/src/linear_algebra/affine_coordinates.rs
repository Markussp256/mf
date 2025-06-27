use container_traits::{AnyParameters, Container, ContainerConstructError, IndexOutOfBoundsError, LinearContainer};

pub trait AffineCoordinates<Index=usize,E=ContainerConstructError<Index>> : Container<Index>+AnyParameters<Self::T,E> {
    fn any_ei(size:Index, index:usize) -> Result<Self, IndexOutOfBoundsError<Index>>;
}

pub trait LinearAffineCoordinates<E=ContainerConstructError<usize>> : LinearContainer+AnyParameters<Self::T,E> {}

impl<E, LAC : LinearContainer+AnyParameters<LAC::T,E>> LinearAffineCoordinates<E> for LAC {}