use nalgebra::{Scalar, SMatrix};
use crate::{for_static::*, OCTSize, ContainerConstructError, IndexOutOfBoundsError, LenTooSmallError};

use num_traits::{Zero,One};

impl<T : Scalar,
     const M:usize,
     const N:usize> FromFn<usize,T> for SMatrix<T,M,N> {
    fn from_fn(f:impl Fn(usize) -> T) -> Self {
        assert!(M == 1 || N == 1);
        if M == 1 {
            SMatrix::from_fn(|_,j|f(j))
        } else {
            SMatrix::from_fn(|i,_|f(i))
        }
    }
}

impl<T : Scalar,
     const M:usize,
     const N:usize> OCTSize<usize> for SMatrix<T,M,N> {
    const OCTSIZE:Option<usize>=if M == 1 { Some(N) } else if N == 1 { Some(M) } else { None };
}

impl<T:Scalar,
     const M:usize,
     const N:usize> Size<usize> for SMatrix<T,M,N> {
        const SIZE:usize=M*N;
}


impl<T : Scalar+Zero,
     const M:usize,
     const N:usize> TryPutAt<usize,T> for SMatrix<T,M,N> {
    fn try_put_at(index:usize, t:T) -> Result<Self,IndexOutOfBoundsError<usize>> {
        let mut res=Self::zeros();
        if M == 1 {
            IndexOutOfBoundsError::try_new(&N, &index)?;
            res[(1,index)]=t;
        } else if N == 1 {
            IndexOutOfBoundsError::try_new(&M, &index)?;
            res[(index,1)]=t;
        } else {
            panic!("either M or N must be 1");
        }
        Ok(res)
    }
}


impl<T : Scalar,
     const M:usize,
     const N:usize> TryFromIterator<T,ContainerConstructError<usize>> for SMatrix<T,M,N>  {

    fn try_take_away<I:Iterator<Item=T>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        assert!(M == 1 || N == 1);
        utils::iter::next_chunk_dyn(iter,M*N)
            .map(|v|Self::from_iterator(v))
            .map_err(|e|LenTooSmallError::new(M*N, e.len()).into())
    }

    crate::try_from_iter_impl!(T);
}

macro_rules! impl_xyz {
    ($uc:ident, $lc:ident, $k:literal, $n:literal) => {
        paste::paste!(
        impl<F : Scalar> $uc<F> for SMatrix<F,1,$n> {
            fn $lc(&self) -> &F { &self[(0,$k)] }
            fn [<e $lc>]() -> Self where F:Zero+One { Self::from_fn(|i,j| if (i,j) == (0,$k) { F::one() } else { F::zero() })}
        }
        
        impl<F : Scalar> $uc<F> for SMatrix<F,$n,1> {
            fn $lc(&self) -> &F { &self[($k,0)] }
            fn [<e $lc>]() -> Self where F:Zero+One { Self::from_fn(|i,j| if (i,j) == ($k,0) { F::one() } else { F::zero() })}
        }
        );
    };

    ($uc:ident, $lc:ident, $k:literal, $n0:literal, $($n:literal),*) => {
        impl_xyz!($uc, $lc, $k, $n0);
        impl_xyz!($uc, $lc, $k, $($n),*);
    }
}

impl_xyz!(X,x,0, 2,3,4);
impl_xyz!(Y,y,1, 2,3,4);
impl_xyz!(Z,z,2,   3,4);