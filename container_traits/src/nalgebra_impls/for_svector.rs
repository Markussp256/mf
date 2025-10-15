use nalgebra::{Const, Matrix, RawStorage, RowSVector, SMatrix, SVector, Scalar};
use crate::{for_static, ContainerConstructError, LenTooSmallError, LinearContainerSized, LinearContainerStatic};

use num_traits::{Zero,One};


// type RowSVectorView   <'a,F, const M:usize, const N:usize>=nalgebra::MatrixView   <'a,F,Const<1>,Const<N>,Const<1>,Const<M>>;
// type RowSVectorViewMut<'a,F, const M:usize, const N:usize>=nalgebra::MatrixViewMut<'a,F,Const<1>,Const<N>,Const<1>,Const<M>>;

// fn test_col<'a>(m:&'a SMatrix<f64,4,3>) -> SVectorView<'a,f64,4> {
//     m.column(0)
// }

// fn test_row<'a>(m:&'a SMatrix<f64,4,3>) -> RowSVectorView<'a,f64,4,3> {
//     m.row(0)
// }


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


impl<T,
    const M:usize,
    const N:usize,
    RS> for_static::Size<usize> for nalgebra::Matrix<T,Const<M>,Const<N>,RS> {
        const SIZE:usize = M*N;
}


// macro_rules! size {
//     ($name:ident<$($lt :lifetime,)? $t:ident, $m0:ident $(,$m:ident)?>, $sz:expr) => {
//         impl<$($lt,)?
//              $t:Scalar,
//              const $m0:usize
//              $(,const $m:usize)*> for_static::Size<usize> for $name<$($lt,)? $t, $m0 $(,$m)*> {
//             const SIZE:usize=$sz;
//         }
//     };
// }
// size!(SMatrix<T,M,N>,M*N);
// size!(SVectorView<'a,T,M>,M);
// size!(RowSVectorView<'a,T,M,N>,N);
// size!(SVectorViewMut<'a,T,M>,M);
// size!(RowSVectorViewMut<'a,T,M,N>,N);


// macro_rules! oct_size {
//     ($name:ident<$($lt :lifetime,)? $t:ident, $m:ident>) => {
//         impl<$($lt,)?
//             $t : Scalar,
//             const $m0:usize> OCTSize<usize> for $name<$($lt,)? $t, $m> {
//             const OCTSIZE:Option<usize>=Some($m);
//         }
//     };

//     ($name:ident<$($lt :lifetime,)? $t:ident, $m:ident, $n:ident>) => {
//         impl<$($lt,)?
//             $t : Scalar,
//             const $m:usize,
//             const $n:usize> OCTSize<usize> for $name<$($lt,)? $t, $m0 $(,$m)*> {
//             const OCTSIZE:Option<usize>=if $m == 1 { Some($n) } else if $n == 1 { Some($m)} else { None };
//         }
//     }
// }


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
    ($m_name:ident, $uc:ident, $lc:ident, $k:literal, $n:literal) => {
        
        impl<F : Scalar, S : RawStorage<F,Const<1>,Const<$n>>> for_static::$uc<F> for $m_name<F,Const<1>,Const<$n>,S> {
            fn $lc(&self) -> &F { &self[(0,$k)] }
        }
        
        impl<F : Scalar, S : RawStorage<F,Const<$n>,Const<1>>> for_static::$uc<F> for $m_name<F,Const<$n>,Const<1>, S> {
            fn $lc(&self) -> &F { &self[($k,0)] }
        }
    };

    ($m_name:ident, $uc:ident, $lc:ident, $k:literal, $n0:literal, $($n:literal),*) => {
        impl_xyz!($m_name, $uc, $lc, $k, $n0     );
        impl_xyz!($m_name, $uc, $lc, $k, $($n),*);
    }
}
impl_xyz!(Matrix, X,x,0, 2,3,4);
impl_xyz!(Matrix, Y,y,1, 2,3,4);
impl_xyz!(Matrix, Z,z,2,   3,4);


// impl_xyz!(SMatrixView, X,x,0, 2,3,4|'a);
// impl_xyz!(SMatrixView, Y,y,1, 2,3,4|'a);
// impl_xyz!(SMatrixView, Z,z,2,   3,4|'a);


// impl_xyz!(SMatrixViewMut, X,x,0, 2,3,4|'a);
// impl_xyz!(SMatrixViewMut, Y,y,1, 2,3,4|'a);
// impl_xyz!(SMatrixViewMut, Z,z,2,   3,4|'a);



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