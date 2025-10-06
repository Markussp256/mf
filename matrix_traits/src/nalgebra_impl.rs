use algebra_traits::Scalarproduct;
use num_traits::Zero;
use utils::iter::ChainExactSize;
use std::ops::Mul;
use crate::{crop_to_square::{CropToSquareMatrixIfTall, CropToSquareMatrixIfWide}, *};

use container_traits::IntoSum;

use nalgebra::{
    ComplexField, Const, DMatrix, DVector, DVectorViewMut, DVectorView, Dim, Dyn, RawStorage, RowDVector, RowSVector, SMatrix, SVectorViewMut, SVectorView, SVector, Scalar, VecStorage, ViewStorageMut, ViewStorage
};

use container_traits::{CommonLengthError, Get, IndexOutOfBoundsError, LenNotEqualToRequiredLenError, TryCommonLength};

type RowDVectorView   <'a,F>=nalgebra::Matrix<F,Const<1>,Dyn,ViewStorage<'a, F, Const<1>, Dyn, Const<1>, Dyn>>;
type RowDVectorViewMut<'a,F>=nalgebra::Matrix<F,Const<1>,Dyn,ViewStorageMut<'a, F, Const<1>, Dyn, Const<1>, Dyn>>;

type RowSVectorView   <'a,F, const M:usize, const N:usize>=nalgebra::MatrixView   <'a,F,Const<1>,Const<N>,Const<1>,Const<M>>;
type RowSVectorViewMut<'a,F, const M:usize, const N:usize>=nalgebra::MatrixViewMut<'a,F,Const<1>,Const<N>,Const<1>,Const<M>>;

impl<   F:Scalar>                ColVectorView for    DVector       <   F>   {}
impl<   F:Scalar>                RowVectorView for RowDVector       <   F>   {}
impl<   F:Scalar, const N:usize> ColVectorView for    SVector       <   F,N> {} 
impl<   F:Scalar, const N:usize> RowVectorView for RowSVector       <   F,N> {}

impl<'a,F:Scalar>                               ColVectorView for    DVectorView   <'a,F>     {}
impl<'a,F:Scalar>                               RowVectorView for RowDVectorView   <'a,F>     {}
impl<'a,F:Scalar,                const N:usize> ColVectorView for    SVectorView   <'a,F,  N> {}
impl<'a,F:Scalar, const M:usize, const N:usize> RowVectorView for RowSVectorView   <'a,F,M,N> {}

impl<'a,F:Scalar>                               ColVectorView for    DVectorViewMut<'a,F>     {}
impl<'a,F:Scalar>                               RowVectorView for RowDVectorViewMut<'a,F>     {}
impl<'a,F:Scalar,                const N:usize> ColVectorView for    SVectorViewMut<'a,F,  N> {}
impl<'a,F:Scalar, const M:usize, const N:usize> RowVectorView for RowSVectorViewMut<'a,F,M,N> {}


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

macro_rules! impl_svec_svec {
    ($lhs:ident <$($a:lifetime,)? $f:ident, $m:ident $(,$n:ident)?>, $rhs:ident <$($b:lifetime,)? $f2:ident, $m2:ident>) => {
        impl<$($a,)?
             $($b,)?
             $f :Scalar+Mul<$f2,Output=F3>,
             $f2:Scalar,
             F3:Scalar+Zero,
             $(const $n:usize,)?
             const $m:usize> VectorVectorProduct<$rhs<$($b,)? $f2, $m2>> for $lhs<$($a,)? $f,$m $(,$n)?> {
                type Output=F3;
            
                fn vector_vector_product(&self, rhs:&$rhs<$($b,)? $f2, $m2>) -> F3 {
                    try_vector_vector_product_impl(self,rhs).unwrap()
                }
        }

        // impl<$($a,)?
        //      $($b,)?
        //      $f :Scalar+Mul<$f2,Output=F3>,
        //      $f2:Scalar,
        //      F3:Scalar+Zero,
        //      $(const $n:usize,)?
        //      const $m:usize> TryVectorVectorProduct<$rhs<$($b,)? $f2, $m2>> for $lhs<$($a,)? $f,$m $(,$n)?> {
        //         type Output=F3;
            
        //         fn try_vector_vector_product(&self, rhs:&$rhs<$($b,)? $f2, $m2>) -> Option<F3> {
        //             try_vector_vector_product_impl(self,rhs)
        //         }
        // }

    };
}
impl_svec_svec!(RowSVector       <   F,M  >, SVector       <   F2,M>);
impl_svec_svec!(RowSVectorView   <'a,F,M,N>, SVector       <   F2,M>);
impl_svec_svec!(RowSVectorViewMut<'a,F,M,N>, SVector       <   F2,M>);
impl_svec_svec!(RowSVector       <   F,M  >, SVectorView   <'b,F2,M>);
impl_svec_svec!(RowSVectorView   <'a,F,M,N>, SVectorView   <'b,F2,M>);
impl_svec_svec!(RowSVectorViewMut<'a,F,M,N>, SVectorView   <'b,F2,M>);
impl_svec_svec!(RowSVector       <   F,M  >, SVectorViewMut<'b,F2,M>);
impl_svec_svec!(RowSVectorView   <'a,F,M,N>, SVectorViewMut<'b,F2,M>);
impl_svec_svec!(RowSVectorViewMut<'a,F,M,N>, SVectorViewMut<'b,F2,M>);


impl<F0  : Scalar+Mul<F1,Output=F2>,
     F1  : Scalar,
     F2  : Zero+Scalar,
     D0  : Dim,
     RS0 : RawStorage<F0,Const<1>,D0>,
     D1  : Dim,
     RS1 : RawStorage<F1,D1,Const<1>>> TryVectorVectorProduct<nalgebra::Matrix<F1,D1,Const<1>,RS1>> for nalgebra::Matrix<F0,Const<1>,D0,RS0> {
    type Output=F2;
    fn try_vector_vector_product(&self, rhs:&nalgebra::Matrix<F1,D1,Const<1>,RS1>) -> Option<F2> {
        (self.ncols() == rhs.nrows()).then(||
            self.iter()
                .zip(rhs.iter())
                .map(|(l,r)|l.clone()*r.clone())
                .into_sum()
        )
    }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const N:usize> IntoVectorVectorProduct<SVector<F2,N>> for RowSVector<F,N> {
        type Output=F3;
     
        fn into_vector_vector_product(self, rhs:SVector<F2,N>) -> F3 {
            try_into_vector_vector_product_impl(self,rhs).unwrap()
        }
}


impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const N:usize> TryIntoVectorVectorProduct<SVector<F2,N>> for RowSVector<F,N> {
        type Output=F3;
     
        fn try_into_vector_vector_product(self, rhs:SVector<F2,N>) -> Option<F3> {
            try_into_vector_vector_product_impl(self,rhs)
        }
}



impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero> TryIntoVectorVectorProduct<DVector<F2>> for RowDVector<F> {
        type Output=F3;
        fn try_into_vector_vector_product(self, rhs:DVector<F2>) -> Option<F3> {
            try_into_vector_vector_product_impl(self,rhs)
        }
}


impl<F:Scalar, 
     R:Dim,
     C:Dim,
     S:RawStorage<F,R,C>> IntoDynMatrix for nalgebra::Matrix<F,R,C,S> where Self : Matrix<T=F> {
        type Output=DMatrix<F>;
}

impl<F:Scalar, 
     R:Dim,
     C:Dim,
     S:RawStorage<F,R,C>> IntoBaseMatrix for nalgebra::Matrix<F,R,C,S> where Self : Matrix<T=F> {
        type Output=Self;
    fn into_base_matrix(self) -> Self::Output {
        self
    }
}

impl<F:Scalar, 
     R:Dim,
     C:Dim,
     S:RawStorage<F,R,C>> AsBaseMatrix for nalgebra::Matrix<F,R,C,S> where Self : Matrix<T=F> {
        type Output=Self;
    fn base_matrix(&self) -> &Self {
        &self
    }
}


impl<F:Scalar+algebra_traits::Scalar> AlgebraMatrix for nalgebra::DMatrix<F> {
    algebra_matrix_impl!();
}

impl<F:Scalar+algebra_traits::Scalar,
     const M:usize,
     const N:usize>                   AlgebraMatrix for nalgebra::SMatrix<F,M,N> {
    algebra_matrix_impl!();
}

impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const M:usize,
     const N:usize> MatrixVectorProduct<SVector<F2,N>> for SMatrix<F,M,N> {
        type Output=SVector<F3,M>;
     
        fn matrix_vector_product(&self, rhs:&SVector<F2,N>) -> Self::Output {
            try_matrix_vector_product_impl(self,rhs).unwrap()
        }
}


impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const M:usize,
     const N:usize> IntoMatrixVectorProduct<SVector<F2,N>> for SMatrix<F,M,N> {
        type Output=SVector<F3,M>;
     
        fn into_matrix_vector_product(self, rhs:SVector<F2,N>) -> Self::Output {
            try_into_matrix_vector_product_impl(self,rhs).unwrap()
        }
}


impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const M:usize,
     const N:usize> TryMatrixVectorProduct<SVector<F2,N>> for SMatrix<F,M,N> {
        type Output=SVector<F3,M>;
     
        fn try_matrix_vector_product(&self, rhs:&SVector<F2,N>) -> Result<Self::Output,VectorConstructError> {
            try_matrix_vector_product_impl(self,rhs)
        }
}


impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const M:usize,
     const N:usize> TryIntoMatrixVectorProduct<SVector<F2,N>> for SMatrix<F,M,N> {
        type Output=SVector<F3,M>;
     
        fn try_into_matrix_vector_product(self, rhs:SVector<F2,N>) -> Result<Self::Output,VectorConstructError> {
            try_into_matrix_vector_product_impl(self,rhs)
        }
}

impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero> TryMatrixVectorProduct<DVector<F2>> for DMatrix<F> {
        type Output=DVector<F3>;
     
        fn try_matrix_vector_product(&self, rhs:&DVector<F2>) -> Result<Self::Output,VectorConstructError> {
            try_matrix_vector_product_impl(self,rhs)
        }
}


impl<F :Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero> TryIntoMatrixVectorProduct<DVector<F2>> for DMatrix<F> {
        type Output=DVector<F3>;
     
        fn try_into_matrix_vector_product(self, rhs:DVector<F2>) -> Result<Self::Output,VectorConstructError> {
            try_into_matrix_vector_product_impl(self,rhs)
        }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const L:usize,
     const M:usize,
     const N:usize> MatrixMatrixProduct<SMatrix<F2,M,N>> for SMatrix<F,L,M> {
        type Output=SMatrix<F3,L,N>;
     
        fn matrix_matrix_product(&self, rhs:&SMatrix<F2,M,N>) -> Self::Output {
            try_matrix_matrix_product_impl(self,rhs).unwrap()
        }
}


impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const L:usize,
     const M:usize,
     const N:usize> IntoMatrixMatrixProduct<SMatrix<F2,M,N>> for SMatrix<F,L,M> {
        type Output=SMatrix<F3,L,N>;
     
        fn into_matrix_matrix_product(self, rhs:SMatrix<F2,M,N>) -> Self::Output {
            try_into_matrix_matrix_product_impl(self,rhs).unwrap()
        }
}

impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const L:usize,
     const M:usize,
     const N:usize> TryMatrixMatrixProduct<SMatrix<F2,M,N>> for SMatrix<F,L,M> {
        type Output=SMatrix<F3,L,N>;
     
        fn try_matrix_matrix_product(&self, rhs:&SMatrix<F2,M,N>) -> Result<Self::Output,MatrixConstructError> {
            try_matrix_matrix_product_impl(self,rhs)
        }
}


impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero,
     const L:usize,
     const M:usize,
     const N:usize> TryIntoMatrixMatrixProduct<SMatrix<F2,M,N>> for SMatrix<F,L,M> {
        type Output=SMatrix<F3,L,N>;
     
        fn try_into_matrix_matrix_product(self, rhs:SMatrix<F2,M,N>) -> Result<Self::Output,MatrixMatrixProduct> {
            try_into_matrix_matrix_product_impl(self,rhs)
        }
}


impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero> TryMatrixMatrixProduct<DMatrix<F2>> for DMatrix<F> {
        type Output=DMatrix<F3>;
     
        fn try_matrix_matrix_product(&self, rhs:&DMatrix<F2>) -> Result<Self::Output,MatrixConstructError> {
            try_matrix_matrix_product_impl(self,rhs)
        }
}


impl<F:Scalar+Mul<F2,Output=F3>,
     F2:Scalar,
     F3:Scalar+Zero> TryIntoMatrixMatrixProduct<DMatrix<F2>> for DMatrix<F> {
        type Output=DMatrix<F3>;
     
        fn try_into_matrix_matrix_product(self, rhs:DMatrix<F2>) -> Result<Self::Output,MatrixConstructError> {
            try_into_matrix_matrix_product_impl(self,rhs)
        }
}


crate::impl_op_diag_dyn! (RowDVector,DMatrix,Scalar);
crate::impl_op_diag_stat!(RowSVector,SMatrix,Scalar);


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

fn get_stride<F:Scalar,R:Dim,C:Dim,RS:RawStorage<F,R,C>>(rs:RS) -> (RS::RStride,RS::CStride){
    rs.strides()
}

fn vec_stor_strides<F:Scalar>(rs:VecStorage<F,Dyn,Dyn>) -> (Const<1>,Dyn) {
    rs.strides()
}

// fn view_dmat<'a,F:Scalar>(m:&'a DMatrix<F>)
//     -> nalgebra::Matrix<F, Const<1>, Dyn,
//         ViewStorage<'a, F, Const<1>, Dyn,
//         <VecStorage<F, Dyn, Dyn> as RawStorage<F, Dyn, Dyn>>::RStride,
//         <VecStorage<F, Dyn, Dyn> as RawStorage<F, Dyn, Dyn>>::CStride>> { //
//     m.row(0)
// }

fn view_dmat<'a,F:Scalar>(m:&'a DMatrix<F>)
    -> nalgebra::Matrix<F, Const<1>, Dyn,
        ViewStorage<'a, F, Const<1>, Dyn,
        Const<1>,
        Dyn>> { //
    m.row(0)
}

impl<F:Scalar> MatrixView for DMatrix<F> {
    type ColView<'a> =    DVectorView<'a,F>;
    type RowView<'a> = RowDVectorView<'a,F>;

    fn nrows(&self) -> usize {
        DMatrix::nrows(self)
    }

    fn ncols(&self) -> usize {
        DMatrix::ncols(self)
    }

    fn try_row_view<'a>(&'a self, i:usize) -> Result<Self::RowView<'a>,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.nrows(),&i)?;
        Ok(self.row(i))
    }

    fn row_views<'a>(&'a self) -> impl ExactSizeIterator<Item=Self::RowView<'a>> {
        self.row_iter()
    }

    fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.ncols(),&j)?;
        Ok(self.column(j))
    }

    fn col_views<'a>(&'a self) -> impl ExactSizeIterator<Item=Self::ColView<'a>> {
        self.column_iter()
    }

}



impl<F:Scalar> Matrix for DMatrix<F> {
    type Col = DVector<F>;
    type Row = RowDVector<F>;
    
    fn rows(&self) -> impl ExactSizeIterator<Item=Self::Row> {
        self.row_iter()
                .map(|r|r.into_owned())
    }

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        let rows:Vec<Self::Row>=
            <Self as Matrix>::rows(&self)
                .collect();
        rows.into_iter()
    }

    fn cols(&self) -> impl ExactSizeIterator<Item=Self::Col> {
        self.column_iter()
            .map(|c|c.into_owned())
    }
    
    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        let cols:Vec<Self::Col>=
            self.cols()
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

impl<F:Scalar, const M:usize, const N:usize> MatrixFixedNumberOfCols<N> for SMatrix<F,M,N> {}

impl<F:Scalar, const M:usize, const N:usize> MatrixFixedNumberOfRows<M> for SMatrix<F,M,N> {}

impl<F:Scalar, const M:usize, const N:usize> MatrixView for SMatrix<F, M, N> {
    
    type RowView<'a>=RowSVectorView<'a,F,M,N>;
    type ColView<'a>=   SVectorView<'a,F,M>;

    fn nrows(&self) -> usize {
        M
    }

    fn ncols(&self) -> usize {
        N
    }

    fn try_row_view<'a>(&'a self, i:usize) -> Result<Self::RowView<'a>,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.nrows(),&i)?;
        Ok(self.row(i))
    }

    fn row_views<'a>(&'a self) -> impl ExactSizeIterator<Item=Self::RowView<'a>> {
        self.row_iter()
    }

    fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.ncols(),&j)?;
        Ok(self.column(j))
    }

    fn col_views<'a>(&'a self) -> impl ExactSizeIterator<Item=Self::ColView<'a>> {
        self.column_iter()
    }
}

impl<F:Scalar, const M:usize, const N:usize> Matrix for SMatrix<F, M, N> {
    
    type Row=RowSVector<F, N>;

    type Col=SVector<F, M>;
    
    fn rows(&self) -> impl ExactSizeIterator<Item=Self::Row> {
        self.row_iter()
            .map(|r|r.into_owned())
    }

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        let rows:Vec<Self::Row>=
            <Self as Matrix>::rows(&self)
                .collect();
        rows.into_iter()
    }
    
    fn cols(&self) -> impl ExactSizeIterator<Item=Self::Col> {
        self.column_iter()
            .map(|c|c.into_owned())
    }

    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        let cols:Vec<Self::Col>=
            <Self as Matrix>::cols(&self)
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

// cam not implement this for general N:usize therefore macro

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

impl<F: Scalar> CropToSquareMatrixIfWide for DMatrix<F> {
    type Output = DMatrix<F>;
    fn crop_to_square_matrix_if_wide(self) -> Self::Output {
        let (m,n)=self.matrix_dimensions();
        if m >= n {
            self
        } else {
            DMatrix::try_from_cols(self.into_cols().take(m)).unwrap()
        }
    }
}

impl<F: Scalar> CropToSquareMatrixIfTall for DMatrix<F> {
    type Output = DMatrix<F>;
    fn crop_to_square_matrix_if_tall(self) -> Self::Output {
        let (m,n)=self.matrix_dimensions();
        if n >= m {
            self
        } else {
            DMatrix::try_from_rows(self.into_rows().take(n)).unwrap()
        }
    }
}


impl<F: Scalar> CropToSquareMatrix for DMatrix<F> {
    type Output = DMatrix<F>;
    fn crop_to_square_matrix(self) -> Self::Output {
        let (m,n)=self.matrix_dimensions();
        if m == n {
            self
        } else if m > n {
            DMatrix::try_from_rows(self.into_rows().take(n)).unwrap()
        } else {
            DMatrix::try_from_cols(self.into_cols().take(m)).unwrap()
        }
    }
}

macro_rules! crop_to_squares_one_with_many {
    ($i0:literal | $($i:literal),* $(,)?) => {
        $(
            impl<F: Scalar> CropToSquareMatrix for SMatrix<F, $i0, $i> {
                type Output = SMatrix<F, $i0, $i0>;
                fn crop_to_square_matrix(self) -> Self::Output {
                    Self::Output::try_from_cols(self.into_cols().take($i0)).unwrap()
                }
            }
            impl<F: Scalar> CropToSquareMatrixIfWide for SMatrix<F, $i0, $i> {
                type Output = SMatrix<F, $i0, $i0>;
                fn crop_to_square_matrix_if_wide(self) -> Self::Output {
                    Self::Output::try_from_cols(self.into_cols().take($i0)).unwrap()
                }
            }
            impl<F: Scalar> CropToSquareMatrixIfTall for SMatrix<F, $i0, $i> {
                type Output = Self;
                fn crop_to_square_matrix_if_tall(self) -> Self {
                    self
                }
            }
            impl<F: Scalar> CropToSquareMatrix for SMatrix<F, $i, $i0> {
                type Output = SMatrix<F, $i0, $i0>;
                fn crop_to_square_matrix(self) -> Self::Output {
                    Self::Output::try_from_rows(self.into_rows().take($i0)).unwrap()
                }
            }
            impl<F: Scalar> CropToSquareMatrixIfWide for SMatrix<F, $i, $i0> {
                type Output = Self;
                fn crop_to_square_matrix_if_wide(self) -> Self {
                    self
                }
            }
            impl<F: Scalar> CropToSquareMatrixIfTall for SMatrix<F, $i, $i0> {
                type Output = SMatrix<F, $i0, $i0>;
                fn crop_to_square_matrix_if_tall(self) -> Self::Output {
                    Self::Output::try_from_rows(self.into_rows().take($i0)).unwrap()
               }
            }
        )*
    };
}


macro_rules! crop_to_squares_all_pairs {
    () => {};
    ($i0:literal $(,$i:literal)*) => {
        crop_to_squares_one_with_many!($i0| $($i),*);
        crop_to_squares_all_pairs!($($i),*);
    };
}

crop_to_squares_all_pairs!(0,1,2,3,4,5,6,7,8,9);

impl<F:Clone+Scalar> TryPopCol for DMatrix<F> {
    type Output = DMatrix<F>;
    fn try_pop_col(self) -> Option<(DMatrix<F>, <Self as Matrix>::Col)> {
        let nc=self.ncols();
        if nc == 0 {
            return None;
        }
        let c=<Self as Matrix>::try_col(&self,nc-1).unwrap();
        Some((Self::try_from_cols(self.into_cols().take(nc-1)).unwrap(),c))
    }
}
impl<F:Clone+Scalar> TryPopRow for DMatrix<F> {
    type Output = DMatrix<F>;
    fn try_pop_row(self) -> Option<(DMatrix<F>, <Self as Matrix>::Row)> {
        let nr=self.nrows();
        if nr == 0 {
            return None;
        }
        let r=<Self as Matrix>::try_row(&self,nr-1).unwrap();
        Some((Self::try_from_rows(self.into_rows().take(nr-1)).unwrap(),r))
    }
}

impl<F:Clone+Scalar> TryPushCol for DMatrix<F> {
    type Output = DMatrix<F>;
    fn try_push_col(self,col:DVector<F>) -> Result<DMatrix<F>, DVector<F>> {
        Self::try_from_cols(
            self.into_cols()
                .chain_exact_size(std::iter::once(col.clone())))
            .map_err(|_|col)
    }
}
impl<F:Clone+Scalar> TryPushRow for DMatrix<F> {
    type Output = DMatrix<F>;
    fn try_push_row(self,row:RowDVector<F>) -> Result<DMatrix<F>, RowDVector<F>> {
        Self::try_from_rows(
            self.into_rows()
                .chain_exact_size(std::iter::once(row.clone())))
            .map_err(|_|row)
    }
}

macro_rules! pop_push {
    ($K:literal,$K1:literal) => {
        impl<F:Clone+Scalar, const M:usize> PopCol for SMatrix<F,M,$K1> {
            type Output = SMatrix<F,M,$K>;
            fn pop_col(self) -> (Self::Output, <Self as Matrix>::Col) {
                let c=<Self as Matrix>::try_col(&self,$K).unwrap();
                let m=SMatrix::<F,M,$K>::try_from_cols(self.into_cols().take($K)).unwrap();
                (m,c)
            }
        }

        impl<F:Clone+Scalar, const M:usize> TryPopCol for SMatrix<F,M,$K1> {
            type Output = SMatrix<F,M,$K>;
            fn try_pop_col(self) -> Option<(Self::Output, <Self as Matrix>::Col)> {
                Some(self.pop_col())
            }
        }

        impl<F:Clone+Scalar, const M:usize> PushCol for SMatrix<F,M,$K> {
            type Output = SMatrix<F,M,$K1>;
            fn push_col(self,col:Self::Col) -> Self::Output {
                Self::Output::try_from_cols(
                    self.into_cols()
                        .chain_exact_size(std::iter::once(col.clone())))
                    .ok().unwrap()
            }
        }

        impl<F:Clone+Scalar, const M:usize> TryPushCol for SMatrix<F,M,$K> {
            type Output = SMatrix<F,M,$K1>;
            fn try_push_col(self,col:Self::Col) -> Result<Self::Output, Self::Col> {
                Ok(self.push_col(col))
            }
        }
    
        impl<F:Clone+Scalar, const N:usize> PopRow for SMatrix<F,$K1,N> {
            type Output = SMatrix<F,$K,N>;
            fn pop_row(self) -> (Self::Output, <Self as Matrix>::Row) {
                let r=<Self as Matrix>::try_row(&self,$K).unwrap();
                let m=SMatrix::<F,$K,N>::try_from_rows(self.into_rows().take($K)).unwrap();
                (m,r)
            }
        }

        impl<F:Clone+Scalar, const N:usize> TryPopRow for SMatrix<F,$K1,N> {
            type Output = SMatrix<F,$K,N>;
            fn try_pop_row(self) -> Option<(Self::Output, <Self as Matrix>::Row)> {
                Some(self.pop_row())
            }
        }


        impl<F:Clone+Scalar, const N:usize> PushRow for SMatrix<F,$K,N> {
            type Output = SMatrix<F,$K1,N>;
            fn push_row(self,row:Self::Row) -> Self::Output {
                Self::Output::try_from_rows(
                    self.into_rows()
                        .chain_exact_size(std::iter::once(row.clone())))
                    .ok().unwrap()
            }
        }

        impl<F:Clone+Scalar, const N:usize> TryPushRow for SMatrix<F,$K,N> {
            type Output = SMatrix<F,$K1,N>;
            fn try_push_row(self,row:Self::Row) -> Result<Self::Output, Self::Row> {
                Ok(self.push_row(row))
            }
        }
    };
}
pop_push!(0,1);
pop_push!(1,2);
pop_push!(2,3);
pop_push!(3,4);
pop_push!(4,5);
pop_push!(5,6);
pop_push!(6,7);
pop_push!(7,8);

// special case (0,0) 
impl<F:Clone+Scalar, const M:usize> TryPopCol for SMatrix<F,M,0> {
    type Output = SMatrix<F,0,0>;
    fn try_pop_col(self) -> Option<(Self::Output, <Self as Matrix>::Col)> {
        None
    }
}
impl<F:Clone+Scalar, const N:usize> TryPopRow for SMatrix<F,0,N> {
    type Output = SMatrix<F,0,0>;
    fn try_pop_row(self) -> Option<(Self::Output, <Self as Matrix>::Row)> {
        None
    }
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
    assert!(m1.is_close_to(&m0));
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


