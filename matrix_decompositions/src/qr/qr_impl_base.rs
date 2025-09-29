use std::ops::Mul;
use num_traits::Zero;
use algebra::Unit;
use algebra_traits::{ClosedTrySub, ConstNonZero, Scalar, ScalarVector, TryDiv, Vectorspace};
use container_traits::{AnyFromIterator, ChangeT, Inner, Iter, IntoIter, IntoContainer, Map, StandardBasis, TryFromFn, TryIntoContainer};
use matrix_traits::*;
use utils::{iter::ChainExactSize,kron_delta};
use super::householder_trafo::HouseholderTrafoGeneric;
use super::qr_def::QRStruct;

trait QRImplTrait
     : MatrixTryConstruct<T=Self::MT,Col=Self::MCol>
      +CropToSquareMatrixIfTall<Output=Self::MCT>
      +CropToSquareMatrixIfWide<Output=Self::MCW>
      where
      HouseholderTrafoGeneric<Self::FCol>
        : Matrix<T=Self::F>
         +TryMatrixMatrixProduct<Self,      Output=Self>
         +TryMatrixMatrixProduct<Self::MDF, Output=Self::MDF> 
{
    type F      : Clone+Scalar+Mul<Self::T,Output=Self::T>+TryDiv<Output=Self::F>;
    type MT     : Clone+Zero+TryDiv<Output=Self::F>;
    type MCol   : Clone+ColVector<T=Self::T>+TryDiv<Self::T,Output=Self::FCol>+ClosedTrySub;
    type FCol   : ScalarVector<T=Self::F>
                 +ColVectorTryConstruct<T=Self::F>
                 +StandardBasis
                 +Clone
                 +ConjugateTranspose<Output=Self::FRow>
                 +Map<Self::F,Self::T,Output=Self::Col>;
    type FRow   : RowVectorTryConstruct<T=Self::F>
                 +ConjugateTranspose<Output=Self::FCol>;
    type MCTRow : RowVectorTryConstruct<T=Self::T>;
    type MCT    : MatrixTryConstruct<T=Self::T, Row=Self::MCTRow>;
    type MCW    : Matrix<T=Self::T>+ChangeT<Self::F,Output=Self::MDF>;
    type MDF    : AlgebraMatrix+MatrixTryConstruct<T=Self::F,Col=Self::FCol>;
}

impl<F      : Clone+Scalar+Mul<T,Output=T>+TryDiv<Output=F>,
     T      : Clone+Zero+TryDiv<Output=F>,
     Col    : ColVector<T=T>+Clone+TryDiv<T,Output=FCol>+ClosedTrySub,
     FCol   : ScalarVector<T=F>
             +ColVectorTryConstruct<T=F>
             +StandardBasis
             +Clone
             +ConjugateTranspose<Output=FRow>
             +Map<F,T,Output=Col>,
     FRow   : RowVectorTryConstruct<T=F>
             +ConjugateTranspose<Output=FCol>,
     MCTRow : RowVectorTryConstruct<T=T>,
     MCT    : MatrixTryConstruct<T=T, Row=MCTRow>,
     MCW    : Matrix<T=T>+ChangeT<F,Output=MDF>,
     MDF    : AlgebraMatrix+MatrixTryConstruct<T=F,Col=FCol>,   
     M      : MatrixTryConstruct<T=T,Col=Col>
             +CropToSquareMatrixIfTall<Output=MCT>
             +CropToSquareMatrixIfWide<Output=MCW>> QRImplTrait for M
where HouseholderTrafoGeneric<FCol> : Matrix<T=F>
                                     +TryMatrixMatrixProduct<M,   Output=M>
                                     +TryMatrixMatrixProduct<MDF, Output=MDF> {
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
     F    : Clone+Scalar+Mul<T,Output=T>+TryDiv<Output=F>,
     T    : Clone+ConstNonZero+TryDiv<Output=F>+Vectorspace<F>+Mul<F::RealType,Output=T>,
     M    : QRImplTrait<MT=T,F=F>+TryIntoSubMatrix<Output=MSub>,
     MSub : QRImplTrait<MT=T,F=F>+TryIntoSubMatrix+IntoBaseMatrix<Output=MSub>> From<MIn> for QRStruct<M::MDF,M::MCT> {
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

        let col0=m.col(0).unwrap();
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
            m=h.clone().try_matrix_matrix_product(m).unwrap();
            Some(h)
        } else {
            None
        };

        // subproblem
        let row0=m.row(0).unwrap();
        let msub:MSub=m.try_into_sub_matrix((0,0)).unwrap();
        let qrsub=QRStruct::<MSub::MDF,MSub::MCT>::from(msub);
        let mut nh=qrsub.n_householder();
        let (qsub, rsub)=qrsub.into_parts();
        // qr_impl_base::<F,V,MDV,MDV,MDF,FCol>(msub);

        // construct q
        let q=M::MDF::try_from_matrix(BlockDiagonal::new(identity(1),qsub)).unwrap();
        let q=if let Some(h)=oh {{
            nh+=1;
            h.try_matrix_matrix_product(q).unwrap()
        }} else { q };

        // construct R
        let r=M::MCT::try_from_rows(std::iter::once(row0.into_container())
            .chain_exact_size(
                rsub.into_rows()
                    .map(|r|M::MCTRow::any_from_iter(None,std::iter::once(M::T::zero())
                    .chain_exact_size(r.into_iterator())).unwrap()))).unwrap();
        
        // let rsub_ext=Self::try_concat_horizontally(Self::zeros((rsub.nrows(),1)),rsub).unwrap();
        // let mrow0=Self::try_from_rows(std::iter::once(row0.into_container())).unwrap();
        // let r=MCT::try_concat_vertically(mrow0,rsub_ext).unwrap();
        
        // return
        Self::new_unchecked(q, r, nh)
}
}

#[cfg(test)]
pub fn check_qr
    <M : Clone+QRTrait>(m:M) -> bool
    where M::T : Scalar+TryDiv,
          <M as QRTrait>::MQ : Matrix<T=<M::T as TryDiv>::Output>+TryMatrixMatrixProduct<<M as QRTrait>::MR,Output=M>,
          <M as QRTrait>::MR : Matrix<T=M::T> {
        let (q,r)=m.clone().into_qr().into_parts();
        let qr:M=q.try_matrix_matrix_product(r).unwrap();
        qr.is_close_to(m)
}

// fn my_mul<F   : Clone+Zero+Mul<Output=F>,
//           Q   : Clone+MatrixTryConstruct<F=F>+AnyMatrixVectorProduct<Col>+AnyMatrixMatrixProduct<R>,
//           R   : Clone+MatrixTryConstruct<F=F,Col=Col>+Transpose<Output=L>,
//           L   : MatrixTryConstruct<F=F>+Transpose<Output=R>,
//           Col : Clone+ColVectorAnyConstruct<T=F>>(q:Q,col:Col,r:R){
//             // let qc=q.clone().try_matrix_vector_product(col);
//             // let qr=q.try_matrix_matrix_product::<Q>(r);
//             let r=RightTriangular::<R>::try_from_matrix(r).ok().unwrap();
//             let qrr=<Q as AnyMatrixMatrixProduct<RightTriangular<R>>>::try_matrix_matrix_product::<Q>(q,r);
//         }




// impl<F  : Clone+Scalar,
//      Row: RowVectorAnyConstruct<T=F>,
//      Col: ColVectorAnyConstruct<T=F>
//          +Clone
//          +Norm<NormT=F::RealType>
//          +TryNormalize
//          +TrySub<Output=Col>
//          +TryScalarproduct<ScProdT=F>
//          +TryDiv<F,Output=Col>
//          +ScalarMul<F>
//          +ChangeT<Row>
//          +ChangeT<F,Output=Col>
//          +ChangeT<MatrixRowDyn<F>>> QRBase for MatrixGeneric<Row,Col>
// where HouseholderTrafoGeneric<Col> : Matrix<F=F>+AnyMatrixMatrixProduct<Self>,
//                                    // +AnyMatrixMatrixProduct<MatrixDyn<F>>,
//       MatrixColDyn<F> : ChangeT<Row>,
//      <MatrixColDyn<F> as ChangeT<Row>>            ::Output : Clone+ColVectorAnyConstruct<T=Row>,
//      <Col             as ChangeT<Row>>            ::Output : Clone+ColVectorAnyConstruct<T=Row>,    
//      <Col             as ChangeT<MatrixRowDyn<F>>>::Output : Clone+ColVectorAnyConstruct<T=MatrixRowDyn<F>> {
//     type Q=MatrixGeneric<MatrixRowDyn<F>,Col>;
//     type R=MatrixGeneric<Row,MatrixColDyn<F>>;

//     fn decompose(self) -> (Stiefel<MatrixDyn<F>>, RightTriangular<MatrixDyn<V>>, usize) {
//         let mut m=self;
//         let (nrows,ncols)=m.matrix_dimensions();
//         if nrows == 0 || ncols == 0 {
//             // if M : Empty we could use trait to generate empty matrices
//             return (Stiefel::try_empty().unwrap(),
//                     RightTriangular::try_empty().unwrap(),
//                     0);
//         }

//         if nrows == 1 {
//             let q=Stiefel::try_from_matrix(crate::matrix![F::one()]).ok().unwrap();
//             let r=RightTriangular::try_from_matrix(m).ok().unwrap();   
//             return (q, r, 0);
//         }

//         let col0=m.col(0).unwrap();
//         let ucol0=Unit::try_new(col0.clone().try_divide_by_norm().unwrap()).ok().unwrap();
//         let e0=Unit::try_new(Col::colvector_try_put_at(0, nrows, F::one()).ok().unwrap()).ok().unwrap();
//         if ncols == 1 {
//             let norm:F::RealType=
//                 col0.clone()
//                     .norm()
//                     .into_signed();
//             let q=if col0.colvector_iter().all(|vi:&F|vi.is_zero()) {
//                 e0
//             } else {
//                 ucol0
//             }.try_into().ok().unwrap();
//             let r=RightTriangular::try_from_matrix(crate::matrix![norm]).ok().unwrap();
//             return (q, r, 0);
//         }
//         let do_housholder=col0.colvector_iter().skip(1).any(|v:&F|!v.is_zero());
//         let oh=do_housholder.then(||{  
//             let h=HouseholderTrafoGeneric::try_froma2b(e0, ucol0).unwrap();
//             m=h.clone()
//                .try_matrix_matrix_product::<Self>(m.clone()).unwrap();
//             h}
//         );

//         // subproblem
//         let asub:MatrixDyn<F>=m.try_submatrix((1..nrows).collect(), (1..ncols).collect()).unwrap();
//         let (qsub, rsub, mut nh)=asub.decompose();

//         // construct q
//         let q=MatrixDyn<F>::try_from_matrix(BlockDiagonal::new(crate::matrix![F::one()],qsub)).ok().unwrap();
//         let q=if let Some(h)=oh {{ 
//             nh+=1;
//             h.try_matrix_matrix_product::<MatrixDyn<F>>(q).unwrap()
//         }} else {
//             q
//         };

//         // construct R 
//         let rsub_ext=MatrixDyn::try_concat_horizontally(MatrixDyn::zero_dyn(rsub.nrows(),1),rsub.into_inner()).unwrap();
//         let mrow0=m.try_submatrix(vec![0],(0..m.ncols()).collect()).unwrap();
//         let rsub_ext=MatrixDyn::try_concat_vertically(mrow0,rsub_ext).unwrap();
//         let r=MatrixDyn<V>::try_from_matrix(rsub_ext).ok().unwrap();

//         (q, r, nh)
//     }
// }




// impl<T:Clone+Field
//     +Conjugate
//     
//     +Vectorspace<f64>
//     +Norm<N2=f64>> QRGeneric<T> {

//     fn gram_schmidt(vs:Vec<MatrixColGeneric<T>>) -> Vec<UnitMatrixColGeneric<T>> {
//         let mut res:Vec<UnitMatrixColGeneric<T>>=Vec::new();
//         let n=vs[0].len();
//         // in case a col vector is orthogonal to the previous col vector we have to find
//         // some vector that is orthogonal to the previous ones 
//         let sb=MatrixColGeneric::standard_basis(n).into_iter();
//         for vsi in vs {
//             for mut vsi in std::iter::once(vsi).chain(sb.clone()) {
//                 if vsi.is_zero() {
//                     continue;
//                 }
//                 for resj in res.iter() {
//                     let resj=resj.vector().clone();
//                     let sp:T=vsi.clone().try_scalar_product::<T,T>(resj.clone()).unwrap();
//                     vsi=vsi.try_sub(resj*sp).unwrap();
//                 }
//                 if !vsi.is_zero() {
//                     let vsin=vsi.try_normalize().unwrap();
//                     res.push(vsin);
//                     break;
//                 }
//             }
//         }
//         res
//     }
// }