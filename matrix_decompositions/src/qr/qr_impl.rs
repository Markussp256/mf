// qr_impl.rs

use num_traits::Zero;
use algebra_traits::{ClosedNeg, RealNumber, ComplexNumber, TryDiv};
use container_traits::IntoInner;
// pub use matrix::{Matrix,MatrixDyn,SquareMatrixDyn};
use matrix_traits::*;

use super::qr_def::*;

macro_rules! impl_qr_special {
    ($tr:ident, $qr:ident, $qn:ident) => {
        impl<F  : $tr,
             V  : Zero+TryDiv<Output=F>+ClosedNeg,
             M  : MatrixViewNotTall<T=V>+QRTrait<MQ=MQ,MR=MR>,
             MQ : MatrixSquareTryConstruct<T=F>,
             MR : MatrixTryConstruct<T=V>> From<M> for $qr<MQ,MR> {
            fn from(m:M) -> Self {
                let qr:QRStruct::<MQ,MR>=m.into_qr();
                let n_householder=qr.n_householder();
                let (mut q,mut r)=qr.into_parts();
                if n_householder % 2 == 1 {
                    q=q.try_neg_col(0).ok().expect("matrix empty");
                    r=r.try_neg_row(0).ok().expect("matrix empty");
                }
                Self::new_unchecked(q.into_inner(),r.into_inner())
            }
        }
    }
}

impl_qr_special!(RealNumber,    QROrthogonalStruct, Orthogonal);
impl_qr_special!(ComplexNumber, QRUnitaryStruct,    Unitary);


#[cfg(test)]
mod tes_with_nalgebra {
    use nalgebra::{DMatrix,SMatrix};  

    use super::super::qr_impl_base::check_qr;

    #[test]
    fn test_qr_smatrix() {
        let m: nalgebra::SMatrix<f64, 3,2>=nalgebra::matrix![1.0,2.0;3.0,4.0;5.0,6.0];
        check_qr(&m);
        let m_dyn:DMatrix<f64>=matrix_traits::IntoDynMatrix::into_dyn_matrix(m);
        check_qr(&m_dyn);
    }


    #[test]
    fn test_new_qr() {
        use super::QRTrait;
        use algebra_traits::TryMaxNormOfEntries;
        use matrix_traits::TryMatrixMatrixProduct;
        let m:SMatrix<f64,3,2>=nalgebra::matrix![1.0, 0.0; 0.0, 1.0; -1.0, 1.0];
        check_qr(&m);
        let (q,r)=m.into_qr().into_parts();
        println!("{}",q);
        println!("{}",r);
        let qr=q.try_matrix_matrix_product(&r).unwrap();
        //let qrs:SMatrix<f64,3,2>=qr.try_into_matrix().unwrap();
        let err:SMatrix<f64,3,2>=qr-m;
        assert!(err.try_max_norm_of_entries().unwrap() < 1e-12);
    }
}