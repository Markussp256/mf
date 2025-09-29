use nalgebra::{RowSVector, SMatrix, SMatrixView, SMatrixViewMut, SVector, Scalar};
use crate::{for_static, ContainerConstructError, IndexOutOfBoundsError, LenTooSmallError, LinearContainerSized, LinearContainerStatic, OCTSize, StandardBasis, TryPutAt};

use num_traits::{Zero,One};

impl<T : Scalar,
     const M:usize,
     const N:usize> for_static::FromFn<usize,T> for SMatrix<T,M,N> {
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


impl<'a,
    T : Scalar,
     const M:usize,
     const N:usize> OCTSize<usize> for SMatrixView<'a, T,M,N> {
    const OCTSIZE:Option<usize>=if M == 1 { Some(N) } else if N == 1 { Some(M) } else { None };
}


impl<'a,
    T : Scalar,
     const M:usize,
     const N:usize> OCTSize<usize> for SMatrixViewMut<'a, T,M,N> {
    const OCTSIZE:Option<usize>=if M == 1 { Some(N) } else if N == 1 { Some(M) } else { None };
}


impl<T:Scalar,
     const M:usize,
     const N:usize> for_static::Size<usize> for SMatrix<T,M,N> {
        const SIZE:usize=M*N;
}

impl<'a,
     T:Scalar,
     const M:usize,
     const N:usize> for_static::Size<usize> for SMatrixView<'a,T,M,N> {
        const SIZE:usize=M*N;
}



impl<'a,
     T:Scalar,
     const M:usize,
     const N:usize> for_static::Size<usize> for SMatrixViewMut<'a,T,M,N> {
        const SIZE:usize=M*N;
}


impl<T : Scalar+Zero,
     const M:usize,
     const N:usize> for_static::TryPutAt<usize,T> for SMatrix<T,M,N> {
    fn try_put_at(index:usize, t:T) -> Result<Self,IndexOutOfBoundsError<usize>> {
        let mut res=Self::zeros();
        if M == 1 {
            IndexOutOfBoundsError::try_new(&N, &index)?;
            res[(1,index)]=t;
        } else if N == 1 {
            IndexOutOfBoundsError::try_new(&M, &index)?;
            res[(index,1)]=t;
        } else {
            assert!(false);
            // panic!("either M or N must be 1");
        }
        Ok(res)
    }
}

impl<T : Scalar+Zero+One,
     const M:usize,
     const N:usize> StandardBasis for SMatrix<T,M,N> {
    fn try_standard_basis_element(_:usize, index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&(M*N),&index)?;
        <Self as for_static::TryPutAt<usize,T>>::try_put_at(index, <T as num_traits::One>::one())
    }
}

impl<T : Scalar+Zero,
     const M:usize,
     const N:usize> TryPutAt<usize,T> for SMatrix<T,M,N> {
    fn try_put_at(size:usize, index:usize, t:T) -> Result<Self,IndexOutOfBoundsError<usize>> {
        if size != M*N {
            assert!(false); // panic!("size does not coincide");
        }
        <Self as for_static::TryPutAt<usize,T>>::try_put_at(index,t)
    }
}


impl<T : Scalar,
     const M:usize,
     const N:usize> for_static::TryFromIterator<T,ContainerConstructError<usize>> for SMatrix<T,M,N>  {

    fn try_take_away<I:Iterator<Item=T>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        assert!(M == 1 || N == 1);
        utils::iter::next_chunk_dyn(iter,M*N)
            .map(|v|Self::from_iterator(v))
            .map_err(|e|LenTooSmallError::new(M*N, e.len()).into())
    }

    crate::try_from_iter_impl!(T);
}


impl<   F:Scalar, const M:usize, const K:usize> LinearContainerSized for SMatrix       <   F,M,K> { const N:usize=M*K; }

macro_rules! impl_stat {
    ($i:literal) => {
        impl<   F:Scalar> LinearContainerStatic<$i> for RowSVector<F,$i> {}
        impl<   F:Scalar> LinearContainerStatic<$i> for    SVector<F,$i> {}
    };
}

impl<F:Scalar> LinearContainerStatic<1> for SMatrix<F,1,1> {}

impl_stat!(2);
impl_stat!(3);
impl_stat!(4);
impl_stat!(5);
impl_stat!(6);
impl_stat!(7);
impl_stat!(8);
impl_stat!(9);



macro_rules! impl_xyz {
    ($m_name:ident, $uc:ident, $lc:ident, $k:literal, $n:literal $(|$lt:lifetime)?) => {
        
        impl<$($lt,)? F : Scalar> for_static::$uc<F> for $m_name<$($lt,)?F,1,$n> {
            fn $lc(&self) -> &F { &self[(0,$k)] }
        }
        
        impl<$($lt,)? F : Scalar> for_static::$uc<F> for $m_name<$($lt,)? F,$n,1> {
            fn $lc(&self) -> &F { &self[($k,0)] }
        }
    };

    ($m_name:ident, $uc:ident, $lc:ident, $k:literal, $n0:literal, $($n:literal),* $(|$lt:lifetime)? ) => {
        impl_xyz!($m_name, $uc, $lc, $k, $n0     $(|$lt)? );
        impl_xyz!($m_name, $uc, $lc, $k, $($n),* $(|$lt)?);
    }
}
impl_xyz!(SMatrix, X,x,0, 2,3,4);
impl_xyz!(SMatrix, Y,y,1, 2,3,4);
impl_xyz!(SMatrix, Z,z,2,   3,4);


impl_xyz!(SMatrixView, X,x,0, 2,3,4|'a);
impl_xyz!(SMatrixView, Y,y,1, 2,3,4|'a);
impl_xyz!(SMatrixView, Z,z,2,   3,4|'a);


impl_xyz!(SMatrixViewMut, X,x,0, 2,3,4|'a);
impl_xyz!(SMatrixViewMut, Y,y,1, 2,3,4|'a);
impl_xyz!(SMatrixViewMut, Z,z,2,   3,4|'a);



macro_rules! impl_exyz {
    ($uc:ident, $lc:ident, $k:literal, $n:literal) => {
        impl<F : Scalar> for_static::$uc<F> for SMatrix<F,1,$n> {
            fn $lc() -> Self where F:Zero+One { Self::from_fn(|i,j| if (i,j) == (0,$k) { F::one() } else { F::zero() })}
        }
        
        impl<F : Scalar> for_static::$uc<F> for SMatrix<F,$n,1> {
            fn $lc() -> Self where F:Zero+One { Self::from_fn(|i,j| if (i,j) == ($k,0) { F::one() } else { F::zero() })}
        }
    };

    ($uc:ident, $lc:ident, $k:literal, $n0:literal, $($n:literal),*) => {
        impl_exyz!($uc, $lc, $k, $n0);
        impl_exyz!($uc, $lc, $k, $($n),*);
    }
}

impl_exyz!(EX,ex,0, 2,3,4);
impl_exyz!(EY,ey,1, 2,3,4);
impl_exyz!(EZ,ez,2,   3,4);