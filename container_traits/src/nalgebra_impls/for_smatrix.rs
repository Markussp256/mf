use nalgebra::{Scalar, SMatrix};
use crate::{for_static::*, ContainerConstructError, LenTooSmallError, OCTSize};

type U2=(usize,usize);

impl<T : Scalar,
     const M:usize,
     const N:usize> FromElement<T> for SMatrix<T,M,N> {
    fn from_element(t:T) -> Self {
        Self::from_element(t)   
    }
}

impl<T : Scalar,
     const M:usize,
     const N:usize> OCTSize<U2> for SMatrix<T,M,N> {
    const OCTSIZE:Option<U2>=Some((M,N));
}

impl<T : Scalar,
     const M:usize,
     const N:usize> FromFn<U2,T> for SMatrix<T,M,N> {
    fn from_fn(f:impl Fn(U2) -> T) -> Self {
        SMatrix::from_fn(|i,j|f((i,j)))
    }
}

impl<T:Scalar,
     const M:usize,
     const N:usize> Size<U2> for SMatrix<T,M,N> {
        const SIZE:U2=(M,N);
}

impl<T : Scalar,
     const M:usize,
     const N:usize> NumberOfDegreesOfFreedom<T> for SMatrix<T,M,N> {
    const NDOFS:usize=M*N;
}


impl<T : Scalar,
     const M:usize,
     const N:usize> TryFromIterator<T,ContainerConstructError<U2>> for SMatrix<T,M,N>  {
    fn try_take_away<I:Iterator<Item=T>>(iter:& mut I) -> Result<Self,ContainerConstructError<U2>> {
        utils::iter::next_chunk_dyn(iter,M*N)
            .map(|v|Self::from_iterator(v))
            .map_err(|e|LenTooSmallError::new(M*N, e.len()).into())
    }

    crate::try_from_iter_impl!(T, ContainerConstructError<U2>);
}