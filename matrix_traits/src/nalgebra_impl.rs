use algebra_traits::Scalarproduct;
use num_traits::Zero;
use std::ops::Mul;
use crate::*;

use nalgebra::{
    ComplexField, Const, DMatrix, DVector, Dim, RawStorage, RowDVector, RowSVector, SMatrix, SVector, Scalar
};

use container_traits::{CommonLengthError, Get, LenNotEqualToRequiredLenError, Map, TryCommonLength};

impl<F:Scalar> ColVector for DVector<F>    {}
impl<F:Scalar> RowVector for RowDVector<F> {}
impl<F:Scalar, const N:usize> ColVector for SVector<F,N>    {} 
impl<F:Scalar, const N:usize> RowVector for RowSVector<F,N> {}

impl<T:Scalar> Transpose for DMatrix<T> {
    type Output=Self;
    fn transpose(self) -> Self::Output {
        Self::transpose(&self).into()
    }
}

impl<T:Scalar,const M:usize, const N:usize> Transpose for SMatrix<T,M,N> {
    type Output=SMatrix<T,N,M>;
    fn transpose(self) -> Self::Output {
        Self::transpose(&self).into()
    }
}

impl<T:Scalar> Transpose for DVector<T> {
    type Output = RowDVector<T>;
    fn transpose(self) -> Self::Output {
        Self::transpose(&self)
    }
}

impl<T:Scalar> Transpose for RowDVector<T> {
    type Output = DVector<T>;
    fn transpose(self) -> Self::Output {
        Self::transpose(&self)
    }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const N:usize> VectorVectorProduct<SVector<F2,N>> for RowSVector<F,N> {
        type Output=F3;
     
        fn vector_vector_product(self, rhs:SVector<F2,N>) -> F3 {
            any_vector_vector_product_impl(self,rhs).unwrap()
        }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const N:usize> AnyVectorVectorProduct<SVector<F2,N>> for RowSVector<F,N> {
        type Output=F3;
     
        fn any_vector_vector_product(self, rhs:SVector<F2,N>) -> Option<F3> {
            any_vector_vector_product_impl(self,rhs)
        }
}



impl<F:Scalar, 
     R:Dim,
     C:Dim,
     S:RawStorage<F,R,C>> IntoDynMatrix for nalgebra::Matrix<F,R,C,S> where Self : Matrix<T=F> {
        type Output=DMatrix<F>;
}



impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const M:usize,
     const N:usize> MatrixVectorProduct<SVector<F2,N>> for SMatrix<F,M,N> {
        type Output=SVector<F3,M>;
     
        fn matrix_vector_product(self, rhs:SVector<F2,N>) -> Self::Output {
            any_matrix_vector_product_impl(self,rhs).unwrap()
        }
}

impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const M:usize,
     const N:usize> AnyMatrixVectorProduct<SVector<F2,N>> for SMatrix<F,M,N> {
        type Output=SVector<F3,M>;
     
        fn any_matrix_vector_product(self, rhs:SVector<F2,N>) -> Option<Self::Output> {
            any_matrix_vector_product_impl(self,rhs)
        }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero+AsRef<F3>,
     const L:usize,
     const M:usize,
     const N:usize> MatrixMatrixProduct<SMatrix<F2,M,N>> for SMatrix<F,L,M> {
        type Output=SMatrix<F3,L,N>;
     
        fn matrix_matrix_product(self, rhs:SMatrix<F2,M,N>) -> Self::Output {
            any_matrix_matrix_product_impl(self,rhs).unwrap()
        }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero> TryVectorVectorProduct<DVector<F2>> for RowDVector<F> {
        type Output=F3;
     
        fn try_vector_vector_product(self, rhs:DVector<F2>) -> Option<F3> {
            any_vector_vector_product_impl(self,rhs)
        }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero> AnyVectorVectorProduct<DVector<F2>> for RowDVector<F> {
        type Output=F3;
     
        fn any_vector_vector_product(self, rhs:DVector<F2>) -> Option<F3> {
            any_vector_vector_product_impl(self,rhs)
        }
}

// impl<F:Scalar+Mul<F2,Output=F3>,
//      F2:Scalar,
//      F3:Scalar+Zero> TryMatrixVectorProduct<DVector<F2>> for DMatrix<F> {
//         type Output=DVector<F3>;
//     fn try_matrix_vector_product(self, rhs:DVector<F2>) -> Option<Self::Output> {
//         try_matrix_vector_product_impl(self,rhs)
//     }
// }

// impl<F:Scalar+Mul<F2,Output=F3>,
//      F2:Scalar,
//      F3:Scalar+Zero> TryMatrixMatrixProduct<DMatrix<F2>> for DMatrix<F> {
//         type Output=DMatrix<F3>;
//         fn try_matrix_matrix_product(self, rhs:DMatrix<F2>) -> Option<DMatrix<F3>> {
//             try_matrix_matrix_product_impl(self,rhs)
//         }
// }

impl<F:Scalar> Matrix for DMatrix<F> {
    type Col = DVector<F>;
    type Row = RowDVector<F>;
    
    fn nrows(&self) -> usize {
        DMatrix::nrows(self)
    }
    
    fn ncols(&self) -> usize {
        DMatrix::ncols(self)
    }

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        let rows:Vec<Self::Row>=
        (0..self.nrows())
            .into_iter()
            .map(|r_ind|<Self as Matrix>::row(&self,r_ind).unwrap().into())
            .collect();
        rows.into_iter()
    }
    
    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        let cols:Vec<Self::Col>=
        (0..self.ncols())
            .into_iter()
            .map(|c_ind|<Self as Matrix>::col(&self,c_ind).unwrap().into())
            .collect();
        cols.into_iter()
    }
}

impl<F:Scalar> MatrixTryConstruct for DMatrix<F> {
    fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError> {
        let rows:Vec<Self::Row>=rows.collect();
        let (nrows,ncols)=match Self::Row::try_common_length(rows.iter()) {
            Ok(sz) => sz,
            Err(CommonLengthError::EmptyVec(_)) => (0,0),
            Err(CommonLengthError::NotAllHaveSameLength(vs)) => { return Err(MatrixConstructError::RowsDoNotHaveTheSameLength(vs)); }
        };
        let f=|i,j|<Self::Row as Get<usize,F>>::get(&rows[i],j).unwrap().clone();
        Ok(Self::from_fn(nrows,ncols,f))
    }
}

// note implementing MatrixDynmaic for DMatrix is not possible because
// nalgebra uses arrays inside which can not be resized

impl<F:ComplexField> Det for DMatrix<F> {
    type DetF=F;

    fn det(self) -> Self::DetF {
        self.determinant()
    }
}


impl<F:Scalar> MatrixConstruct        for DMatrix<F> {}
impl<F:Scalar> MatrixDynamicallySized for DMatrix<F> {}

// impl<F:Scalar> BuildMatrix<DVector<F>> for RowDVector<F> {
//     type Matrix=DMatrix<F>;
// }

impl<F:Scalar, const M:usize, const N:usize> FixedNumberOfCols for SMatrix<F,M,N> {
    const NCOLS:usize = N;
}

impl<F:Scalar, const M:usize, const N:usize> FixedNumberOfRows for SMatrix<F,M,N> {
    const NROWS:usize = M;
}

impl<F:Scalar, const M:usize, const N:usize> Matrix for SMatrix<F, M, N> {
    
    type Row=RowSVector<F, N>;

    type Col=SVector<F, M>;

    fn nrows(&self) -> usize {
        M
    }

    fn ncols(&self) -> usize {
        N
    }
    
    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        let rows:Vec<Self::Row>=
        (0..self.nrows())
            .into_iter()
            .map(|r_ind|<Self as Matrix>::row(&self,r_ind).unwrap().into())
            .collect();
        rows.into_iter()
    }
    
    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        let cols:Vec<Self::Col>=
        (0..self.ncols())
            .into_iter()
            .map(|c_ind|<Self as Matrix>::col(&self,c_ind).unwrap().into())
            .collect();
        cols.into_iter()
    }
}

impl<F:Scalar, const M:usize, const N:usize> MatrixTryConstruct for SMatrix<F,M,N> {

    fn try_from_rows(mut rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError> {
        let rows:[Self::Row;M]=utils::iter::next_chunk(& mut rows)
                .map_err(|e|LenNotEqualToRequiredLenError::new(M,e.len()))?;
        let f=
        |i,j|
        <Self::Row as Get<usize,F>>::get(&rows[i], j)
         .unwrap()
         .clone();
        Ok(Self::from_fn(f))
    }
}

// cam not implement this for generela N:usize therefore macro

macro_rules! impl_det {
    ($n:literal) => {
        impl<F:ComplexField> Det for SMatrix<F,$n,$n>{
            type DetF=F;
            fn det(self) -> Self::DetF {
                self.determinant()
            }
        }
    };
}
impl_det!(1);
impl_det!(2);
impl_det!(3);
impl_det!(4);
impl_det!(5);
impl_det!(6);
impl_det!(7);
impl_det!(8);
impl_det!(9);

crate::impl_tall_square_and_wide_matrix_marker!(SMatrix, Scalar);

impl<F:Scalar, const N:usize> MatrixSquareTryConstruct for SMatrix<F,N,N> {}

impl<F:Scalar+Scalarproduct<ScProdT=ScProdT>,ScProdT:Scalar> HermitianOuterProduct for DVector<F> {
    type Output=DMatrix<ScProdT>;
}

impl<F:Scalar+Scalarproduct<ScProdT=ScProdT>,ScProdT:Scalar, const N:usize> HermitianOuterProduct for SVector<F,N> {
    type Output=SMatrix<ScProdT,N,N>;
}


impl<F:Scalar+algebra_traits::Scalar> GramMatrix for DMatrix<F> {
    type Output=DMatrix<F>;
}

impl<F:Scalar+algebra_traits::Scalar, const M:usize, const N:usize> GramMatrix for SMatrix<F,M,N> {
    type Output=SMatrix<F,N,N>;
}

impl<F:Scalar, const M0:usize, const N0:usize> ChangeDim for SMatrix<F,M0,N0> {
    type Output<const M:usize,const N:usize> = SMatrix<F,M,N>;
}

impl<F:Scalar, const M:usize> SquareStaticMatrix for SMatrix<F,M,M> {
    const M:usize = M;
}

impl<F:Scalar, const M:usize, const N:usize> StaticMatrix for SMatrix<F,M,N> {
    const M:usize = M;
    const N:usize = N;
}

#[test]
fn test_iter() {
    let m0=nalgebra::matrix![1.0,2.0;3.0,4.0;5.0,6.0];
    let m1=m0.clone();
    assert!(m1.is_close_to(m0));
}

// pub trait GramMatrix : Matrix where
//     Self::F : Clone+Zero+Scalarproduct<ScProdT = Self::F>,
//     Self::Row : Transpose {
//     type Output:MatrixSquareTryConstruct<F=Self::F,Row=Self::Row,Col=<Self::Row as Transpose>::Output>;
//     fn gram_matrix(&self) -> Self::Output {
//         Self::Output::try_from_dim_and_fn(
//             self.ncols(),
//             self.ncols(),
//             |i,j|self.try_col_sc_prod(i, j)).unwrap()
//     }
// }

// impl<F:Scalar,const M:usize,const N:usize> BuildMatrix<SVector<F,M>> for RowSVector<F,N> {
//     type Matrix=SMatrix<F,M,N>;
// }


