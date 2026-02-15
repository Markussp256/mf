use algebra_traits::Scalarproduct;
use container_traits::{
    nalgebra_impls::DimExtension,
    OCTSize,
    CommonLengthError,
    Get,
    IndexOutOfBoundsError,
    IntoSum,
    IntoIter,
    TryCommonLength};

use num_traits::Zero;
use utils::iter::ChainExactSize;
use std::ops::Mul;
use crate::{crop_to_square::{CropToSquareMatrixIfTall, CropToSquareMatrixIfWide}, *};

use nalgebra::{
    allocator::Allocator,
    ComplexField, Const,
    DMatrix, DVector, DefaultAllocator, Dim, DimMin,
    OVector, OMatrix,
    RawStorage, RowDVector, RowSVector,
    SMatrix, SVector, Scalar,
};

type U2=(usize,usize);

type ORowVector<T,C>=nalgebra::OMatrix<T,nalgebra::U1,C>;

impl<F:Scalar,R:DimExtension,S:RawStorage<F,R,Const<1>>> ColVectorView for nalgebra::Matrix<F,R,Const<1>,S> {}
impl<F:Scalar,C:DimExtension,S:RawStorage<F,Const<1>,C>> RowVectorView for nalgebra::Matrix<F,Const<1>,C,S> {}


impl<T:Scalar,
     R:DimExtension,
     C:DimExtension,
     S:RawStorage<T,R,C>> Transpose for nalgebra::Matrix<T,R,C,S>
     where DefaultAllocator : Allocator<C,R> {
    type Output = nalgebra::OMatrix<T,C,R>;
    fn transpose(&self) -> Self::Output {
         nalgebra::Matrix::<T,R,C,S>::transpose(&self)
    }

    fn into_transpose(self) -> Self::Output {
         self.transpose()
    }
}

impl<F0  : Scalar+Mul<F1,Output=F2>,
     F1  : Scalar,
     F2  : Zero+Scalar,
     A   : DimExtension,
     B   : DimExtension,
     RS0 : RawStorage<F0,Const<1>,A>,
     RS1 : RawStorage<F1,B,Const<1>>> TryVectorVectorProduct<nalgebra::Matrix<F1,B,Const<1>,RS1>> for nalgebra::Matrix<F0,Const<1>,A,RS0> {
    type Output=F2;
    fn try_into_vector_vector_product(self, rhs:nalgebra::Matrix<F1,B,Const<1>,RS1>) -> Option<F2> {
        (self.len() == rhs.len()).then(||
            self.into_iterator()
                .zip(rhs.into_iterator())
                .map(|(l,r)|l*r)
                .into_sum())
    }
}


impl<F0  : Scalar+Mul<F1,Output=F2>,
     F1  : Scalar,
     F2  : Zero+Scalar,
     const N:usize,
     RS0 : RawStorage<F0,Const<1>,Const<N>>,
     RS1 : RawStorage<F1,Const<N>,Const<1>>> VectorVectorProduct<nalgebra::Matrix<F1,Const<N>,Const<1>,RS1>> for nalgebra::Matrix<F0,Const<1>,Const<N>,RS0> {
    type Output=F2;
    fn into_vector_vector_product(self, rhs:nalgebra::Matrix<F1,Const<N>,Const<1>,RS1>) -> F2 {
        self.into_iterator()
            .zip(rhs.into_iterator())
            .map(|(l,r)|l*r)
            .into_sum()
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


impl<F  : Scalar+Mul<F1,Output=F2>,
     F1 : Scalar+Clone,
     F2 : Scalar+Zero,
     D  : DimExtension,
     const N:usize,
     S  : RawStorage<F, D, Const<N>>+Clone,
     S1 : RawStorage<F1,Const<N>, Const<1>>+Clone> MatrixVectorProduct<nalgebra::Matrix<F1,Const<N>,Const<1>,S1>> for nalgebra::Matrix<F,D,Const<N>,S>
    where DefaultAllocator : Allocator<D,Const<N>>+Allocator<D>+Allocator<Const<1>,D> {
        type Output=nalgebra::OMatrix<F2,D,Const<1>>;

        fn matrix_vector_product<'a>(&'a self, rhs:&'a nalgebra::Matrix<F1,Const<N>,Const<1>,S1>) -> Self::Output {
            try_matrix_vector_product_impl(self,rhs).unwrap()
        }// ::<'a,Self, Self::RowView<'a>,nalgebra::Matrix<F1,Const<N>,Const<1>,S1>,nalgebra::OMatrix<F2,D,Const<1>>>

        fn into_matrix_vector_product(self, rhs:&nalgebra::Matrix<F1,Const<N>,Const<1>,S1>) -> Self::Output where Self : Clone {
            try_matrix_vector_product_impl(&self,rhs).unwrap()
        }
}


impl<F :Scalar+Mul<F1,Output=F2>,
     F1:Scalar+Clone,
     F2:Scalar+Zero,
     A  : DimExtension,
     B  : DimExtension,
     C  : DimExtension,
     S : RawStorage<F, A,B>+Clone,
     S1: RawStorage<F1,C,Const<1>>+Clone> TryMatrixVectorProduct<nalgebra::Matrix<F1,C,Const<1>,S1>> for nalgebra::Matrix<F,A,B,S>
    where DefaultAllocator : Allocator<A>+Allocator<Const<1>,A> {
        type Output=nalgebra::OMatrix<F2,A,Const<1>>;
     
        fn try_matrix_vector_product(&self, rhs:&nalgebra::Matrix<F1,C,Const<1>,S1>) -> Result<Self::Output,VectorConstructError> {
            try_matrix_vector_product_impl(self,rhs)
        }

        fn try_into_matrix_vector_product(self, rhs:&nalgebra::Matrix<F1,C,Const<1>,S1>) -> Result<Self::Output,VectorConstructError> {
            try_matrix_vector_product_impl(&self,rhs)
        }
}


impl<F  : Scalar+Mul<F1,Output=F2>,
     F1 : Scalar,
     F2 : Scalar+Zero,
     A  : DimExtension,
     B  : DimExtension,
     C  : DimExtension,
     S  : RawStorage<F, Const<1>,A>+Clone,
     S1 : RawStorage<F1,B,C>> TryVectorMatrixProduct<nalgebra::Matrix<F1,B,C,S1>> for nalgebra::Matrix<F,Const<1>,A,S>
     where DefaultAllocator                    : Allocator<Const<1>,A>
                                                +Allocator<Const<1>,C>
                                                +Allocator<B>
                                                +Allocator<C> {
        type Output = nalgebra::OMatrix<F2,Const<1>,C>;
        fn try_vector_matrix_product(&self, rhs:&nalgebra::Matrix<F1,B,C,S1>) -> Result<nalgebra::OMatrix<F2,Const<1>,C>,VectorConstructError> {
            try_vector_matrix_product_impl(self,rhs)   
        }
}

impl<F:Scalar+Mul<F1,Output=F2>,
     F1:Scalar,
     F2:Scalar+Zero,
     const L:usize,
     const M:usize,
     const N:usize,
     S  : RawStorage<F, Const<L>,Const<M>>,
     S1 : RawStorage<F1,Const<M>,Const<N>>> MatrixMatrixProduct<nalgebra::Matrix<F1,Const<M>,Const<N>,S1>> for nalgebra::Matrix<F,Const<L>,Const<M>,S> {
        type Output=SMatrix<F2,L,N>;
        fn matrix_matrix_product(&self, rhs:&nalgebra::Matrix<F1,Const<M>,Const<N>,S1>) -> Self::Output {
            try_matrix_matrix_product_impl(self,rhs).unwrap()
        }
}


impl<F  : Scalar+Mul<F1,Output=F2>,
     F1 : Scalar,
     F2 : Scalar+Zero,
     L  : DimExtension,
     M1 : DimExtension,
     M2 : DimExtension,
     N  : DimExtension,
     S  : RawStorage<F, L,M1>,
     S1 : RawStorage<F1,M2,N>> TryMatrixMatrixProduct<nalgebra::Matrix<F1,M2,N,S1>> for nalgebra::Matrix<F,L,M1,S>
     where DefaultAllocator                    : AllAllocator<L,M1>
                                                +AllAllocator<M2,N>
                                                +AllAllocator<L,N> {
        type Output = nalgebra::OMatrix<F2,L,N>;
        fn try_matrix_matrix_product(&self, rhs:&nalgebra::Matrix<F1,M2,N,S1>) -> Result<nalgebra::OMatrix<F2,L,N>,MatrixConstructError> {
            try_matrix_matrix_product_impl(self,rhs)   
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

// fn get_stride<F:Scalar,R:Dim,C:Dim,RS:RawStorage<F,R,C>>(rs:RS) -> (RS::RStride,RS::CStride){
//     rs.strides()
// }

// fn vec_stor_strides<F:Scalar>(rs:VecStorage<F,Dyn,Dyn>) -> (Const<1>,Dyn) {
//     rs.strides()
// }

// fn view_dmat<'a,F:Scalar>(m:&'a DMatrix<F>)
//     -> nalgebra::Matrix<F, Const<1>, Dyn,
//         ViewStorage<'a, F, Const<1>, Dyn,
//         <VecStorage<F, Dyn, Dyn> as RawStorage<F, Dyn, Dyn>>::RStride,
//         <VecStorage<F, Dyn, Dyn> as RawStorage<F, Dyn, Dyn>>::CStride>> { //
//     m.row(0)
// }

// fn view_dmat<'a,F:Scalar>(m:&'a DMatrix<F>)
//     -> nalgebra::Matrix<F, Const<1>, Dyn,
//         ViewStorage<'a, F, Const<1>, Dyn,
//         Const<1>,
//         Dyn>> { //
//     m.row(0)
// }

impl<F : Scalar,
     R : DimExtension,
     C : DimExtension,
     S : RawStorage<F, R, C>> MatrixView for nalgebra::Matrix<F, R, C, S> where Self : OCTSize<U2> {
    type RowView<'a> = nalgebra::MatrixView<'a, F, Const<1>, C, S::RStride, S::CStride> where S : 'a;
    type ColView<'a> = nalgebra::MatrixView<'a, F, R, Const<1>, S::RStride, S::CStride> where S : 'a;

    fn nrows(&self) -> usize {
        self.nrows()
    }

    fn ncols(&self) -> usize {
        self.ncols()
    }

    fn try_row_view<'a>(&'a self, i: usize) -> Result<Self::RowView<'a>, IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.nrows(), &i)?;
        Ok(self.row(i))
    }

    fn row_views<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::RowView<'a>> {
        self.row_iter()
    }

    fn try_col_view<'a>(&'a self, j: usize) -> Result<Self::ColView<'a>, IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.ncols(), &j)?;
        Ok(self.column(j))
    }

    fn col_views<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::ColView<'a>> {
        self.column_iter()
    }
}

trait AllAllocator<R:Dim,C:Dim>
    : Allocator<R>
     +Allocator<C>
     +Allocator<R,C>
     +Allocator<C,R>
     +Allocator<Const<1>,R>
     +Allocator<Const<1>,C>
     +Allocator<R,Const<1>>
     +Allocator<C,Const<1>> {}

impl<R : Dim,
     C : Dim,
     S : Allocator<R>
        +Allocator<C>
        +Allocator<R,C>
        +Allocator<C,R>
        +Allocator<Const<1>,R>
        +Allocator<Const<1>,C>
        +Allocator<R,Const<1>>
        +Allocator<C,Const<1>>> AllAllocator<R,C> for S {}

impl<F:Scalar,
     R:DimExtension,
     C:DimExtension> Matrix for OMatrix<F,R,C>
     where DefaultAllocator : AllAllocator<R,C> {
    type Col = OVector<F,R>;
    type Row = ORowVector<F,C>;

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


impl<F:Scalar,
     R:DimExtension,
     C:DimExtension> MatrixTryConstruct for OMatrix<F,R,C>
    where DefaultAllocator : AllAllocator<R,C>  {
    fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError> {
        let rows:Vec<Self::Row>=rows.collect();
        let (nrows,ncols)=match Self::Row::try_common_length(rows.iter()) {
            Ok(sz) => sz,
            Err(CommonLengthError::EmptyVec(_)) => (0,0),
            Err(CommonLengthError::NotAllHaveSameLength(vs)) => { return Err(MatrixConstructError::RowsDoNotHaveTheSameLength(vs)); }
        };
        let f=|i,j|<Self::Row as Get<usize,F>>::get(&rows[i],j).unwrap().clone();
        let r=R::new(Some(nrows));
        let c=C::new(Some(ncols));
        Ok(Self::from_fn_generic(r,c, f))
    }
}

// note implementing MatrixDynmaic for DMatrix is not possible because
// nalgebra uses arrays inside which can not be resized

impl<F: ComplexField,
     D: DimExtension+DimMin<D,Output=D>,
     S: nalgebra::Storage<F,D,D>> Det for nalgebra::Matrix<F,D,D,S>
     where DefaultAllocator : AllAllocator<D,D> {
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

impl<F:Scalar, const M:usize, const N:usize> MatrixViewFixedNumberOfCols<N> for SMatrix<F,M,N> {}

impl<F:Scalar, const M:usize, const N:usize> MatrixViewFixedNumberOfRows<M> for SMatrix<F,M,N> {}


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


impl<F:Scalar, const M:usize> SquareStaticMatrixView for SMatrix<F,M,M> {
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
