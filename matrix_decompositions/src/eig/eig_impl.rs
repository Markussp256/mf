
use algebra::Unit;
use algebra_traits::{Conjugate, ConstNonZero, NormSquared, Scalar, ScalarVector, Tolerance, TrySolve, TrySolveHomogeneous, TrySub};
use num_traits::Zero;
use container_traits::{AnyFromIterator, ChangeDim, FromElement, Inner, IntoInner, Iter, StandardBasis, TryFromFn};

use matrix_traits::*;
use matrix_wrappers::{RightTriangular, Stiefel};
use utils::{iter::ChainExactSize, kron_delta};

use super::eig_def::EigBaseConditions;

use crate::{qr::HouseholderTrafoGeneric, QRStruct, QRTrait};

// pub fn rayleight_quotient<
//     R   : RealNumber,
//     F   : ComplexNumber<RealType=R>+Clone,
//     M   : AlgebraMatrix<T=F,Row=Row,Col=Col>+MatrixViewSquare+TryMatrixVectorProduct<Col,Output=Col>,
//     Row : RowVector<T=F>+TryVectorVectorProduct<Col,Output=F>,
//     Col : Clone+Zero+NormSquared<Norm2T=R>+ColVector<T=F>+Transpose<Output=Row>>(m:M,col:Col) -> Option<F> {
//     if col.is_zero() {
//         return None;
//     }
//     let row=col.clone().conjugate_transpose();
//     let coln2=col.clone().into_norm_squared().into_signed();
//     let mcol=m.try_matrix_vector_product(col)?;
//     row.try_vector_vector_product(mcol).unwrap()
//        .try_div(coln2).ok()
// }

pub fn rayleight_quotient_unit_vector<
    F   : Scalar+Clone,
    M   : MatrixSquare<T=F,Row=Row,Col=Col>+TryMatrixVectorProduct<Col,Output=Col>,
    Row : RowVector<T=F>+TryVectorVectorProduct<Col,Output=F>+Conjugate<Output=Row>,
    Col : Clone+Zero+NormSquared<Norm2T=F::RealType>+ColVector<T=F>+Transpose<Output=Row>>(m:M,col:Unit<Col>) -> Option<F> {
    let row=col.inner().conjugate_transpose();
    let mcol=m.try_matrix_vector_product(col.inner())?;
    row.try_vector_vector_product(&mcol)
}


pub trait EigImpl : MatrixSquare where Self::T : Scalar {
    fn eig       (self, init_ew:Self::T) -> (Stiefel<Self>,DiagonalMatrixGeneric<Self::Row>);
    fn eigen_pair(self, init_ew:Self::T) -> (Self::T, Unit<Self::Col>);
}


impl<F   : Clone+Scalar+ConstNonZero,
     Row : RowVectorTryConstruct<T=F>
          +FromElement<usize,F>
          +TryVectorVectorProduct<Col,Output=F>
          +Conjugate<Output=Row>
          +Transpose<Output=Col>,
     Col : ColVectorTryConstruct<T=F>
          +ScalarVector<T=F>
          +Zero
          +Conjugate<Output=Col>
          +Transpose<Output=Row>
          +Clone
          +StandardBasis,
     M   : MatrixSquareTryConstruct<T=F,Row=Row,Col=Col>
          +AlgebraMatrix
          +QRTrait<MQ=M,MR=M>
          +ChangeDim
          +EigBaseConditions
          +TryMatrixMatrixProduct<MP,Output=MP>
          +TryMatrixVectorProduct<Col,Output=Col>
          +TrySub<DiagonalMatrixGeneric<Row>,Output=M>
          +TryPopCol<Output=MP>,
      MP  : Clone
           +Matrix<T=F,Col=Col>
           +Conjugate<Output=MP>
           +Transpose<Output=MPT>
           +TryMatrixMatrixProduct<MSub,Output=MP>
           +TryPopRow<Output=MSub>,
      MPT : Matrix<T=F,Row=Row>
           +TryMatrixMatrixProduct<MP,Output=MSub>,
      MSub: Matrix<T=F>
           +TryMatrixMatrixProduct<MPT,Output=MPT>
           +EigImpl
           +EigBaseConditions> EigImpl for M
    where   RightTriangular<M> : TrySolveHomogeneous<MatrixSolveError,Output=Col>
                                 +TrySolve<Col,MatrixSolveError, Output=Col>,
            QRStruct<M,M> : TrySolve<Col,MatrixSolveError, Output=Col>,
         DiagonalMatrixGeneric<M   ::Row> : TryMatrixMatrixProduct<M   ,Output=M>,
         DiagonalMatrixGeneric<MSub::Row> : TryMatrixMatrixProduct<MSub,Output=MSub> {

// inverse iteration to find eigen pair
    fn eigen_pair(self,init_ew:F) -> (F, Unit<Col>) {
    // rayleight quotient iteration
    let n=self.n();
    let mut ev=Unit::<Col>::try_standard_basis_element(n, 0).unwrap();
    // choose complex first initial eigenvalue approximation to break symmetry. 
    let mut ew=init_ew;
    for iter in 0..10 {
        let m_shift:Self=self.clone().try_sub(DiagonalMatrixGeneric::<Row>::from_element(n, ew.clone())).ok().unwrap();
        let qr:QRStruct::<M,M>=m_shift.clone().into_qr();
        if qr.r().diagonal().any(|rii|rii.is_zero()) {
            let (_,r)=qr.into_parts();
            let dbn:Col=r.try_solve_homogeneous().unwrap()
                .try_divide_by_norm().unwrap().1;
            ev=Unit::<Col>::try_new(dbn).ok().unwrap(); // Unit::<Col>::try_dir::<F,Col>(r.try_solve_homogeneous().unwrap()).unwrap().1;
            return (ew,ev);
        } else {
            let x:Col=qr.try_solve(ev.inner().clone()).unwrap();
            let dbn:Col=x.try_divide_by_norm().unwrap().1;
            ev=Unit::<Col>::try_new(dbn).ok().unwrap();
            if m_shift
                 .try_matrix_vector_product(ev.inner()).unwrap()
                 .into_norm()
                 .is_small() {
                return (ew,ev);
            }
        }
        ew=rayleight_quotient_unit_vector(self.clone(), ev.clone()).unwrap();
    }
    panic!("failed to find eigen pair");
}

fn eig(self,init_ew:F) -> (Stiefel<M>,DiagonalMatrixGeneric<Row>) {
        let n=self.n();
        let identity=|i:usize|Stiefel::<M>::try_from_fn((n,n),|(j,k)| kron_delta::<usize,F>(j,k)).ok().unwrap();
        if n <= 1 {
            return  (identity(n),
                     DiagonalMatrixGeneric::<Row>::new(self.into_rows().next().unwrap()));
        }
        let (ew, ev)=self.clone().eigen_pair(init_ew.clone());
        let ee=Unit::<Col>::try_standard_basis_element(n,n-1).ok().unwrap();
        let do_housholder=ev.iter().take(n-1).any(|v:&F|!v.is_zero());
        let (u,d)=if do_housholder {
            let h:Stiefel<MP>=HouseholderTrafoGeneric::try_froma2b(ee, ev.clone()).unwrap()
                .try_into_unitary_matrix::<M>().unwrap()
                .try_pop_col().unwrap().0;
            let msub=h.inner().clone().conjugate_transpose().try_matrix_matrix_product(self.clone()
                    .try_matrix_matrix_product(h.inner()).unwrap()).unwrap();
            let (qsub,d)=msub.eig(init_ew).into_parts();
            let qsubh=h.into_inner().try_matrix_matrix_product(qsub.into_inner()).unwrap();
            (Stiefel::<M>::try_from_cols(
                qsubh.into_cols()
                     .chain_exact_size(std::iter::once(-ev.into_inner())) // minus to get special unitary
            ).ok().unwrap(),d)
        } else {
            let msub=self.try_pop_col().unwrap().0
                         .try_pop_row().unwrap().0;
            let (q,d)=msub.eig(init_ew).into_parts();
            (Stiefel::<M>::try_from_Matrix(BlockDiagonal::new(q,identity(1)).unwrap()), d)
        };
        (u,
        DiagonalMatrixGeneric::<Row>::any_from_iter(None,
                d.into_diagonal()
                 .chain_exact_size(std::iter::once(ew))
            ).ok().unwrap())
}

}

#[cfg(test)]
use algebra::c64;

#[cfg(test)]
use algebra_traits::ComplexNumber;


#[cfg(test)]
pub fn check_eig
    <M : Clone+EigImpl+Transpose<Output=M>+TryMatrixMatrixProduct<Output=M>>(m:M) -> bool
    where M::T : ComplexNumber,
    DiagonalMatrixGeneric<M::Row> : TryMatrixMatrixProduct<M,Output=M> {
        let eig=m.clone().eig(M::T::one());
        let m_approx=eig.into_matrix();
        m_approx.is_close_to(m)
}

#[test]
pub fn test_eig() {
    use algebra_traits::RealAndImag;
    use container_traits::Map;
    let m:nalgebra::SMatrix<c64,2,2>=nalgebra::matrix![1.0,2.0;2.0,3.0]
        .map(|r|c64::new(r,0.0));
    check_eig(m);
}
