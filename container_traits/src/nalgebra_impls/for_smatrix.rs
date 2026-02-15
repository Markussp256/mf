use nalgebra::{SMatrix, Scalar};
use crate::{ContainerConstructError, FromElement, IndexOutOfBoundsError, LenTooSmallError, for_static};

type U2=(usize,usize);

use num_traits::{Zero,One};


impl<T : Scalar,
     const M:usize,
     const N:usize> FromElement<usize,T> for SMatrix<T,M,N> {
    fn from_element(_:usize,t:T) -> Self {
        Self::from_element(t)   
    }
}

impl<T : Scalar,
     const M:usize,
     const N:usize> for_static::FromElement<T> for SMatrix<T,M,N> {
    fn from_element(t:T) -> Self {
        Self::from_element(t)   
    }
}


impl<T : Scalar,
     const M:usize,
     const N:usize> for_static::FromFn<U2,T> for SMatrix<T,M,N> {
    fn from_fn(f:impl Fn(U2) -> T) -> Self {
        SMatrix::from_fn(|i,j|f((i,j)))
    }
}

impl<T:Scalar,
     const M:usize,
     const N:usize> for_static::Size<U2> for SMatrix<T,M,N> {
        const SIZE:U2=(M,N);
}

impl<T : Scalar,
     const M:usize,
     const N:usize> for_static::NumberOfDegreesOfFreedom<T> for SMatrix<T,M,N> {
    const NDOFS:usize=M*N;
}


impl<T : Scalar,
     const M:usize,
     const N:usize> for_static::TryFromIterator<T,ContainerConstructError<U2>> for SMatrix<T,M,N>  {
    fn try_take_away<I:Iterator<Item=T>>(iter:& mut I) -> Result<Self,ContainerConstructError<U2>> {
        utils::iter::next_chunk_dyn(iter,M*N)
            .map(|v|Self::from_iterator(v))
            .map_err(|e|LenTooSmallError::new(M*N, e.len()).into())
    }

    crate::try_from_iter_impl!(T, ContainerConstructError<U2>);
}

impl<T : Scalar, const M:usize, const N:usize> for_static::TryPutAt<usize,T> for SMatrix<T,M,N> {
    fn try_put_at(index:usize,t:T) -> Result<Self,IndexOutOfBoundsError<usize>> where T : num_traits::Zero {
        IndexOutOfBoundsError::try_new(&(M*N),&index)?;
        let mut res=SMatrix::<T,M,N>::from_element(T::zero());
        res[(index % M,index/M)]=t;
        Ok(res)
    }
}


impl<T : Scalar+Zero+One, const M:usize, const N:usize> for_static::StandardBasis for SMatrix<T,M,N> {
    fn try_standard_basis_element(index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
        <Self as for_static::TryPutAt<usize,T>>::try_put_at(index,T::one())
    }
    
    fn standard_basis() -> impl ExactSizeIterator<Item=Self> {
        (0..M*N).map(|i|<Self as for_static::StandardBasis>::try_standard_basis_element(i).unwrap())
    }
}