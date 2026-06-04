// qr_impl_base.rs

use std::ops::Mul;
use num_traits::Zero;
use algebra::Unit;
use algebra_traits::{Conjugate, ClosedTrySub, ConstNonZero, Scalar, ScalarVector, TryDiv, Vectorspace};
use container_traits::{AnyFromIterator, ChangeT, Inner, Iter, IntoIter, IntoContainer, Get, Map, StandardBasis, TryFromFn, TryIntoContainer};
use matrix_traits::*;
use utils::{iter::ChainExactSize,kron_delta};
use super::householder_trafo::HouseholderTrafoGeneric;
use super::qr_def::QRStruct;

pub trait QRImplTrait
     : MatrixTryConstruct<T=Self::MT,Col=Self::MCol>
      +CropToSquareMatrixIfTall<Output=Self::MCT>
      +CropToSquareMatrixIfWide<Output=Self::MCW>
    {
    type F      : Clone+Scalar+Mul<Self::T,Output=Self::T>+TryDiv<Output=Self::F>;
    type MT     : Clone+Zero+TryDiv<Output=Self::F>;
    type MCol   : Clone+ColVector<T=Self::T>+TryDiv<Self::T,Output=Self::FCol>+ClosedTrySub;
    type FCol   : ScalarVector<T=Self::F>
                 +ColVectorTryConstruct<T=Self::F>
                 +StandardBasis
                 +Clone
                 +Conjugate<Output=Self::FCol>
                 +Transpose<Output=Self::FRow>
                 +Map<Self::F,Self::T,Output=Self::MCol>;
    type FRow   : RowVectorTryConstruct<T=Self::F>
                 +Conjugate<Output=Self::FRow>
                 +Transpose<Output=Self::FCol>;
    type MCTRow : RowVectorTryConstruct<T=Self::T>;
    type MCT    : MatrixTryConstruct<T=Self::T, Row=Self::MCTRow>;
    type MCW    : Matrix<T=Self::T>+ChangeT<Self::F,Output=Self::MDF>;
    type MDF    : AlgebraMatrix+MatrixTryConstruct<T=Self::F,Col=Self::FCol>;
}

impl<F      : Clone+Scalar+Mul<T,Output=T>+TryDiv<Output=F>,
     T      : Clone+Zero+TryDiv<Output=F>,
     Col    : ColVectorTryConstruct<T=T>+Clone+TryDiv<T,Output=FCol>+ClosedTrySub,
     FCol   : ScalarVector<T=F>
             +ColVectorTryConstruct<T=F>
             +StandardBasis
             +Clone
             +Conjugate<Output=FCol>
             +Transpose<Output=FRow>
             +Map<F,T,Output=Col>,
     FRow   : RowVectorTryConstruct<T=F>
             +Conjugate<Output=FRow>
             +Transpose<Output=FCol>,
     MCTRow : RowVectorTryConstruct<T=T>,
     MCT    : MatrixTryConstruct<T=T, Row=MCTRow>,
     MCW    : Matrix<T=T>+ChangeT<F,Output=MDF>,
     MDF    : AlgebraMatrix+MatrixTryConstruct<T=F,Col=FCol>,   
     M      : MatrixTryConstruct<T=T,Col=Col>
             +CropToSquareMatrixIfTall<Output=MCT>
             +CropToSquareMatrixIfWide<Output=MCW>> QRImplTrait for M
    {
    type F=F;
    type MT=T;
    type MCol=Col;
    type FCol=FCol;
    type FRow=FRow;
    type MCTRow=MCTRow;
    type MCT=MCT;
    type MCW=MCW;
    type MDF=MDF;
}

impl<MIn  : IntoBaseMatrix<Output=M>,
     F    : Clone+Scalar+Mul<T,Output=T>,
     T    : Clone+ConstNonZero+TryDiv<Output=F>+Vectorspace<F>+Mul<F::RealType,Output=T>,
     M    : QRImplTrait<MT=T,F=F>+TryIntoSubMatrix<Output=MSub>,
     MSub : QRImplTrait<MT=T,F=F>+TryIntoSubMatrix<Output=MSub>+IntoBaseMatrix<Output=MSub>> From<MIn> for QRStruct<M::MDF,M::MCT> {
    fn from(m:MIn) -> Self {
        let mut m:M=m.into_base_matrix();
        let (nrows,ncols)=m.matrix_dimensions();
        let identity=|i|M::MDF::try_from_fn((i,i),|(j,k)|kron_delta::<usize,M::F>(j,k)).unwrap();

        if nrows <= 1 {
            return Self::new_unchecked(
                identity(nrows),
                m.crop_to_square_matrix_if_tall(),
                0);
        }

        let col0=m.try_col(0).unwrap();
        let oucol0=Unit::<M::FCol>::try_dir(col0.clone());
        let e0=Unit::<M::FCol>::try_standard_basis_element(nrows,0).ok().unwrap();
        if ncols == 1 {
            let (r00,qcol0dyn)=oucol0.unwrap_or((M::T::zero(),e0));
            let qcol0=qcol0dyn.inner().clone().try_into_container().unwrap();
            let q=M::MDF::try_from_cols(std::iter::once(qcol0)).ok().unwrap();
            // let r00:V=V::linear_combination(qcol0dyn.conjugate().into_iter().zip(col0.into_iter()));
            let r=M::MCT::try_from_fn((1,1),|(_,_)|r00.clone()).unwrap();
            return Self::new_unchecked(q, r, 0);
        }
        let do_housholder=col0.iter().skip(1).any(|v:&M::T|!v.is_zero());
        let oh:Option<HouseholderTrafoGeneric<M::FCol>>=if do_housholder {
            let h=HouseholderTrafoGeneric::try_froma2b(e0, oucol0.unwrap().1).unwrap();
            m=h.try_matrix_matrix_product(&m).unwrap();
            Some(h)
        } else {
            None
        };

        // subproblem
        let row0=m.try_row(0).unwrap();
        let msub:MSub=m.try_into_sub_matrix((0,0)).unwrap();
        let qrsub=QRStruct::<MSub::MDF,MSub::MCT>::from(msub);
        let mut nh=qrsub.n_householder();
        let (qsub, rsub)=qrsub.into_parts();
        // qr_impl_base::<F,V,MDV,MDV,MDF,FCol>(msub);

        // construct q
        let q=M::MDF::try_from_fn((qsub.nrows()+1,qsub.ncols()+1),|(i,j)|
            if        i == 0 && j == 0 {
                F::one()
            } else if i == 0 || j == 0 {
                F::zero()
            } else {
                qsub.get((i-1,j-1)).unwrap().clone()
            }).unwrap();
        let q=if let Some(h)=oh {{
            nh+=1;
            h.try_matrix_matrix_product(&q).unwrap()
        }} else { q };

        // construct R
        let r=M::MCT::try_from_rows(std::iter::once(row0.into_container())
            .chain_exact_size(
                rsub.into_rows()
                    .map(|r|M::MCTRow::any_from_iter(None,std::iter::once(M::T::zero())
                    .chain_exact_size(r.into_iterator())).unwrap()))).unwrap();

        Self::new_unchecked(q, r, nh)
}
}

#[cfg(test)]
use crate::QRTrait;

#[cfg(test)]
pub fn check_qr
    <M : Clone+QRTrait>(m:&M) -> bool
    where M::T : Scalar+TryDiv,
          <M as QRTrait>::MQ : Matrix<T=<M::T as TryDiv>::Output>+TryMatrixMatrixProduct<<M as QRTrait>::MR,Output=M>,
          <M as QRTrait>::MR : Matrix<T=M::T> {
        let (q,r)=m.clone().into_qr().into_parts();
        let qr:M=q.try_matrix_matrix_product(&r).unwrap();
        qr.is_close_to(m)
}