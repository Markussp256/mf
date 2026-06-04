// qr_def.rs

use std::ops::Mul;
use num_traits::Zero;
use matrix_traits::*;
use algebra_traits::{Conjugate, TryDiv,TrySolve, TrySolveHomogeneous, Scalar, ComplexNumber, RealNumber};
use container_traits::{ChangeT,IntoInner,ItemT,NewUnchecked};
use matrix_wrappers::{Stiefel, SpecialOrthogonal, SpecialUnitary, RightTriangular};


macro_rules! def_qr {
    ($name_struct:ident, $name_trait:ident, $qn:ident, $qtr:ident, $tr:ident $(, $n_householder:ident)? ) => {

        pub struct $name_struct<MQ : $qtr,MR : Matrix>
            where MQ::T : $tr,
                  MR::T : Zero {
            q:$qn<MQ>,
            r:RightTriangular<MR>,
            $( $n_householder:usize )?
        }

        impl<MQ : $qtr,MR : Matrix> $name_struct<MQ,MR>
            where MQ::T : $tr,
                  MR::T : Zero {
            pub fn new_unchecked(mq:MQ,mr:MR $( ,$n_householder:usize)?) -> Self {
                $name_struct{
                      q:$qn::new_unchecked(mq),
                      r:RightTriangular::new_unchecked(mr),
                      $( $n_householder)?}
            }
            pub fn q(&self) -> & $qn<MQ> { &self.q}
            pub fn r(&self) -> &RightTriangular<MR> { &self.r}
            $(
                pub fn $n_householder(&self) -> usize { self.$n_householder }
            )?
            pub fn into_parts(self) -> ($qn<MQ>, RightTriangular<MR>) { (self.q,self.r) }
        }

        impl<F  : $tr+Mul<V,Output=V>,
             V  : Zero+TryDiv<Output=F>,
             MQ : $qtr<T=F>+TryMatrixMatrixProduct<MR,Output=M>,
             MR : Matrix<T=V>,
             M  : Matrix<T=V>> $name_struct<MQ,MR> {
            pub fn into_matrix(self) -> M {
                self.q
                    .into_inner()
                    .try_matrix_matrix_product(&self.r.into_inner()).unwrap()
            }

            pub fn try_solve_least_squares
            <Rhs : ColVectorTryConstruct+ChangeT<Mid::T,Output=Mid>,
             Mid : ColVectorTryConstruct,
             Out : ColVectorTryConstruct>(self, rhs:&Rhs) -> Option<Out>
            where   MQ : Conjugate<Output=MQ>+Transpose<Output=MQ>+TryMatrixVectorProduct<Rhs,T=F, Output=Mid>,
                    F : Mul<Rhs::T,Output=Mid::T>,
                    RightTriangular<MR> : TrySolve<Mid, MatrixSolveError, Output=Out> {
                let qt=self.q.into_conjugate_transpose();
                let qtr=qt.try_into_matrix_vector_product(rhs).ok()?;
                self.r.try_solve(qtr).ok()
            }
        }

        impl<F   : $tr+Mul<V,Output=V>,
             V   : Zero+TryDiv<Output=F>,
             MQ  : $qtr<T=F>,
             MR  : Matrix<T=V>,
             Out> TrySolveHomogeneous<MatrixSolveError> for $name_struct<MQ,MR>
             where RightTriangular<MR> : TrySolveHomogeneous<MatrixSolveError,Output=Out> {
            type Output=Out;
            fn try_solve_homogeneous(self) -> Result<Out,MatrixSolveError> {
                self.r
                    .try_solve_homogeneous()
            }
        }

        impl<F   : $tr+std::ops::Mul<V,Output=V>,
             V   : Zero+TryDiv<Output=F>,
             MQ  : $qtr<T=F>+Conjugate<Output=MQ>+Transpose<Output=MQ>+TryMatrixVectorProduct<Rhs,Output=Mid>,
             MR  : Matrix<T=V>,
             Mid : ColVector,
             Rhs : ColVector,
             Out : ColVector> TrySolve<Rhs,MatrixSolveError> for $name_struct<MQ,MR>
             where RightTriangular<MR> : TrySolve<Mid,MatrixSolveError,Output=Out> {
            type Output=Out;
            fn try_solve(self,rhs:Rhs) -> Result<Out,MatrixSolveError> {
                let mid=
                    self.q
                        .into_conjugate_transpose()
                        .try_into_matrix_vector_product(&rhs).unwrap();
                self.r
                    .try_solve(mid)
            }
        }

        pub trait $name_trait : Matrix+Into<$name_struct<Self::MQ,Self::MR>>
        where Self::T : Zero+TryDiv<Output=<Self::MQ as ItemT>::T>,
              <Self::MQ as ItemT>::T : $tr {
            type MQ : $qtr;
            type MR : Matrix<T=Self::T>;
            fn into_qr(self) -> $name_struct<Self::MQ,Self::MR> {
                self.into()
            }
        }

        impl<F  : $tr,
             V  : Zero+TryDiv<Output=F>,
             MQ : $qtr<T=F>,
             MR : Matrix<T=V>,
             M  : Matrix<T=V>
                 +CropToSquareMatrixIfWide<Output=MQ>
                 +CropToSquareMatrixIfTall<Output=MR>
                 +Into<$name_struct<MQ,MR>>> $name_trait for M {
            type MQ=MQ;
            type MR=MR;
        }

    };
}

// generic (economical) qr
def_qr!(QRStruct, QRTrait, Stiefel, Matrix, Scalar, n_householder);

// qr with non-tall matrix resulting in a square q-matrix
def_qr!(QROrthogonalStruct, QROrthogonalTrait, SpecialOrthogonal, MatrixViewSquare, RealNumber);
def_qr!(QRUnitaryStruct, QRUnitaryTrait, SpecialUnitary, MatrixViewSquare, ComplexNumber);
