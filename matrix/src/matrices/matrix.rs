use algebra_traits::Pow2;
use container_traits::InstanceStructureDescriptor;
use matrix_traits::{try_matrix_matrix_product_impl, try_matrix_vector_product_impl, AsBaseSquareMatrix, FixedNumberOfCols, IntoBaseSquareMatrix, MatrixMatrixProduct, MatrixSquareTryConstruct, MatrixTryConstruct, MatrixVectorProduct, SquareStaticMatrix, StaticMatrix};
use num_traits::Zero;
use std::ops::Mul;
use super::MatrixGeneric;
use super::super::{MatrixCol, MatrixRow};
use algebra::Vector;

use container_traits::{TryFromFn, for_static::FromFn};

pub type Matrix<F, const M:usize, const N:usize>=MatrixGeneric<MatrixRow<F,N>,MatrixCol<F,M>>;

type U2=(usize,usize);

impl<F:'static, const M:uisze, const N:usize> FixedNumberOfCols for Matrix<F,M,N> {
    const NCOLS:usize = N;
}

impl<F:'static, const M:uisze, const N:usize> FixedNumberOfRows for Matrix<F,M,N> {
    const NROWS:usize = M;
}

impl<F:'static, const M:usize> SquareStaticMatrix for Matrix<F,M,M> {
    const M:usize = M;
}

impl<F:'static, const M:usize, const N:usize> StaticMatrix for Matrix<F,M,N> {
    const M:usize = M;
    const N:usize = N;
}


impl<F:'static, const M:usize,const N:usize> FromFn<U2,F> for Matrix<F,M,N> {
    fn from_fn(f:impl Fn(U2) -> F) -> Self {
        Self::try_from_fn(InstanceStructureDescriptor::Size((M,N)), f).unwrap()
    }
}

impl<F:'static,const M:usize, const N:usize> Matrix<F,M,N> {

    pub fn from_rows(rows:[MatrixRow<F,N>;M]) -> Self {
        Self::try_from_rows(rows.into_iter()).unwrap()
    }

    pub fn from_cols(cols:[MatrixCol<F,M>;N]) -> Self {
        Self::try_from_cols(cols.into_iter()).unwrap()
    }
}

macro_rules! matrix_i {
    ($i:literal) => {
        paste::paste!(
            pub type [<Matrix $i>]<F>=Matrix<F,$i,$i>;
        );
    };
}
matrix_i!(1);
matrix_i!(2);
matrix_i!(3);
matrix_i!(4);

impl<F> Matrix1<F> {
    pub fn new(f:F) -> Self {
        crate::matrix![f]
    }
}




macro_rules! impl_matrix_vector_product {
    ($tr0:ident, $rhs:ident $(, $fn:ident)?) => {
        impl<F:'static+Mul<F2,Output=F3>,
            F2: Clone,
            F3:'static+Zero,
            const M:usize,
            const N:usize>  $tr0<$rhs<F2,N>> for Matrix<F,M,N> {
            $(type Output = $rhs<F3,M>;
            fn $fn(self, rhs:$rhs<F2,N> ) -> $rhs<F3,M> {
                any_matrix_vector_product_impl(self, rhs).unwrap()
            })?
        }
    }
}

macro_rules! impl_product {
    ($tr0:ident, $tr1:ident $(, $fn0:ident, $fn1:ident)?) => {

        impl_matrix_vector_product!($tr0, MatrixCol $(, $fn0)?);
        impl_matrix_vector_product!($tr0, Vector $(, $fn0)?);

        impl<F:'static+Clone+Mul<F2,Output=F3>,
            F2:'static+Clone,
            F3:'static+Zero,
            const L:usize,
            const M:usize,
            const N:usize> $tr1<Matrix<F2,M,N>> for Matrix<F,L,M> {
            $(type Output = Matrix<F3,L,N>;
            fn $fn1(self, rhs:Matrix<F2,M,N>) -> Matrix<F3,L,N> {
                any_matrix_matrix_product_impl(self, rhs).unwrap()
            })?
        }
    };
}
impl_product!(MatrixVectorProduct, MatrixMatrixProduct, matrix_vector_product, matrix_matrix_product);

impl<F,
     F2,
     F3,
     const M:usize,
     const N:usize> Mul<MatrixCol<F2,N>> for Matrix<F,M,N>
     where Self : MatrixVectorProduct<MatrixCol<F2,N>,Output=MatrixCol<F3,M>> {
        type Output=MatrixCol<F3,M>;
        fn mul(self, rhs:MatrixCol<F2,N>) -> Self::Output {
            self.matrix_vector_product(rhs)
        }
}

impl<F,
     F2,
     F3,
     const L:usize,
     const M:usize,
     const N:usize> Mul<Matrix<F2,M,N>> for Matrix<F,L,M>
     where Self : MatrixMatrixProduct<Matrix<F2,M,N>,Output=Matrix<F3,L,N>> {
        type Output=Matrix<F3,L,N>;
        fn mul(self, rhs:Matrix<F2,M,N>) -> Self::Output {
            self.matrix_matrix_product(rhs)
        }
}

impl<F:Clone, const N:usize> Pow2 for Matrix<F,N,N> where Self : MatrixMatrixProduct {
    type Output=<Self as MatrixMatrixProduct>::Output;
    fn pow2(self) -> <Self as Pow2>::Output {
        self.clone()
            .matrix_matrix_product(self)
    }
}

crate::impl_mul_diag!(Matrix<F,M,N>);

matrix_traits::impl_tall_square_and_wide_matrix_marker!(Matrix);

impl<F:'static,const M:usize> MatrixSquareTryConstruct for Matrix<F,M,M> {}

impl<F:'static, const M:usize> IntoBaseSquareMatrix for Matrix<F,M,M> {
    type Output=Self;
    fn into_base_square_matrix(self) -> Self {
        self
    }
}

impl<F:'static, const M:usize> AsBaseSquareMatrix for Matrix<F,M,M> {
    type Output=Self;
    fn base_square_matrix(&self) -> &Self::Output {
        &self
    }
}

matrix_decompositions::impls_qr_tall_static!(1,2,3,4, Matrix);
matrix_decompositions::impls_qr_non_tall_static!(1,2,3,4, Matrix);


#[test]
fn test_add() {
    let m1:Matrix<i32,2,2>=crate::matrix![1,2;3,4];
    let m2:Matrix<i32,2,2>=crate::matrix![3,4;5,6];
    let res=crate::matrix![4,6;8,10];
    assert_eq!(m1+m2,res);
}

#[test]
fn test_iter() {
    let m0=crate::matrix![1.0,2.0;3.0,4.0;5.0,6.0];
    let mut iter=<_ as container_traits::Iter<f64>>::iter(&m0).cloned();
    assert_eq!(iter.next(),Some(1.0));
    assert_eq!(iter.next(),Some(2.0));
    assert_eq!(iter.next(),Some(3.0));
    assert_eq!(iter.next(),Some(4.0));
    assert_eq!(iter.next(),Some(5.0));
    assert_eq!(iter.next(),Some(6.0));
    assert_eq!(iter.next(),None);
}

#[test]
fn test_is_close_to() {
    let m0=crate::matrix![1.0,2.0;3.0,4.0;5.0,6.0];
    let m1=m0.clone();
    let iter0=<_ as container_traits::Iter<f64>>::iter(&m0).cloned();
    let iter1=<_ as container_traits::Iter<f64>>::iter(&m1).cloned();
    assert!(iter0.zip(iter1)
                 .all(|(l,r)|algebra_traits::Tolerance::is_close_to(l, r)));
}