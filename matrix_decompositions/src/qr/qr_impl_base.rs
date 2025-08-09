use std::ops::Mul;

use algebra::Unit;
use algebra_traits::{InnerProductSpace1d, Scalar, ScalarVector, TryDiv};

use container_traits::{ChangeT, Inner, IntoContainer, Iter, StandardBasis, TryIntoContainer};
#[cfg(test)]
use matrix_traits::TryFromMatrix;
use matrix_traits::{AlgebraMatrix, TryMatrixMatrixProduct, BlockDiagonal, ColVectorTryConstruct, IntoMatrix, Matrix, MatrixDynamicallySized, TrySubMatrix};

use super::householder_trafo::HouseholderTrafoGeneric;

pub trait QRImplBase : MatrixDynamicallySized where Self::T : TryDiv {

    type MDF : MatrixDynamicallySized<T=<Self::T as TryDiv>::Output>;
    
    fn qr_impl_base(self) -> (Self::MDF, Self, usize);
}


impl<F    : Clone+Scalar+Mul<V,Output=V>,
     V    : Clone+TryDiv<V,Output=F>+Mul<F::RealType,Output=V>+InnerProductSpace1d,
     M    : MatrixDynamicallySized<T=V>+ChangeT<F,Output=MDF>,
     MDF  : AlgebraMatrix+MatrixDynamicallySized<T=F,Col=FCol>,
     FCol : ScalarVector+ColVectorTryConstruct<T=F>+StandardBasis+Clone> QRImplBase for M
     where M::Col : Clone + TryDiv<V,Output=FCol>,
     HouseholderTrafoGeneric<FCol> : Matrix<T=F>+TryMatrixMatrixProduct<M, Output=M>
                                                +TryMatrixMatrixProduct<MDF, Output=MDF> {
    type MDF=MDF;
    
    fn qr_impl_base(self) -> (Self::MDF, Self, usize) {
        let mut m:M=self;
        let (nrows,ncols)=m.matrix_dimensions();

        if nrows == 0 || ncols == 0 {
            return (MDF::empty(),
                    Self::empty(),
                    0);
        }

        if nrows == 1 {
            let q=MDF::identity(1);
            let r=m.into_matrix();
            return (q, r, 0);
        }

        let col0=m.col(0).unwrap();
        let oucol0=Unit::<FCol>::try_dir(col0.clone());
        let e0=Unit::<FCol>::try_standard_basis_element(nrows,0).ok().unwrap();
        if ncols == 1 {
            let (r00,qcol0dyn)=oucol0.unwrap_or((V::zero(),e0));
            let qcol0=qcol0dyn.inner().clone().try_into_container().unwrap();
            let q=MDF::try_from_cols(std::iter::once(qcol0)).ok().unwrap();
            // let r00:V=V::linear_combination(qcol0dyn.conjugate().into_iter().zip(col0.into_iter()));
            let r=Self::scalar(r00);
            return (q, r, 0);
        }
        let do_housholder=col0.iter().skip(1).any(|v:&V|!v.is_zero());
        let oh:Option<HouseholderTrafoGeneric<FCol>>=if do_housholder {
            let h=HouseholderTrafoGeneric::try_froma2b(e0, oucol0.unwrap().1).unwrap();
            m=h.clone().try_matrix_matrix_product(m).unwrap();
            Some(h)
        } else {
            None
        };

        // subproblem
        let msub:Self=m.try_submatrix::<Self>(1..nrows, 1..ncols).unwrap();
        let (qsub, rsub, mut nh)=msub.qr_impl_base();
           // qr_impl_base::<F,V,MDV,MDV,MDF,FCol>(msub);

        // construct q
        let q:MDF=BlockDiagonal::new(MDF::identity(1),qsub).into_matrix();
        let q=if let Some(h)=oh {{
            nh+=1;
            h.try_matrix_matrix_product(q).unwrap()
        }} else { q };

        // construct R 
        let rsub_ext=Self::try_concat_horizontally(Self::zeros((rsub.nrows(),1)),rsub).unwrap();
        let mrow0=Self::try_from_rows(std::iter::once(m.row(0).unwrap().into_container())).unwrap();
        let r=Self::try_concat_vertically(mrow0,rsub_ext).unwrap();
        
        // return
        (q, r, nh)
}
}

#[cfg(test)]
pub fn check_qr
    <M : Clone+QRImplBase>(m:M) -> bool
    where M::T : Scalar,
          M::MDF : TryMatrixMatrixProduct<M,Output=M> {
        let (q,r,_)=m.clone().qr_impl_base();
        let qr:M=M::try_from_matrix(q.try_matrix_matrix_product(r).unwrap()).unwrap();
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