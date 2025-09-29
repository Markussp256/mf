// nh is number of householder transforms

// for square matrix we can make Q special, i.e. det=1 so that we can compute determinant
// by computing determinant of R

// use {MatrixTryConstruct, TryIntoMatrix};
// use matrix_wrappers::{SpecialOrthogonal, Orthogonal, SpecialUnitary, Unitary, SpecialStiefel, Wide, RightTriangular, SquareRightTriangular, Stiefel};

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
             M  : MatrixNotTall<T=V>+QRTrait<MQ=MQ,MR=MR>,
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

// normal (economical) qr
// impl<F   : Scalar+Mul<V,Output=V>,
//      V   : Zero+TryDiv<Output=F>,
//      M   : matrix_traits::Matrix<T=V>
//           +ChangeT<F,Output=MF>
//           +IntoDynMatrix<Output=MDV>
//           +CropToSquareMatrixIfTall<Output=MR>,
//      MF  : CropToSquareMatrixIfWide<Output=MQ>,
//      MQ  : matrix_traits::Matrix<T=F>+TryMatrixMatrixProduct<MR,Output=M>, // same shape as M but different type
//      MR  : MatrixTryConstruct<T=V>,
//      MDF : MatrixDynamicallySized<T=F>+TryIntoMatrix<matrix_wrappers::Stiefel<MQ>>,
//      MDV : MatrixDynamicallySized<T=V>+crate::qr::QRImplBase<MDF=MDF>+TryIntoMatrix<RightTriangular<MR>>>
//      crate::QR for M {
//     type Q=Stiefel<MQ>;
//     type R=RightTriangular<MR>;
//     fn qr(self) -> (Stiefel<MQ>, RightTriangular<MR>) {
//         let sd:MDV=self.into_dyn_matrix();
//         let (q,r,_):(MDF,MDV,_)=sd.qr_impl_base(); // $crate::qr::qr_impl_base::qr_impl_base(self);
//         let q=TryIntoMatrix::try_into_matrix(q).unwrap();
//         let r=TryIntoMatrix::try_into_matrix(r).unwrap();
//         (q,r)
//     }
// }


// #[macro_export]
// macro_rules! impl_qr_non_tall_single {
//     ($o_or_u_qr:ident, $o_or_u_m:ident, $r_c_or_s:ident ) => {
//     impl<F   : $r_c_or_s+Mul<V,Output=V>,
//          V   : Zero+TryDiv<Output=F>+algebra_traits::ClosedNeg,
//          M   : MatrixTryConstruct<T=V>
//               +IntoDynMatrix<Output=MDV>
//               +ChangeT<F,Output=MF>,
//          MF  : CropToSquareMatrix<Output=MQ>,
//          MQ  : MatrixSquare<T=F>+TryMatrixMatrixProduct<M,Output=M>,
//          MDF : MatrixDynamicallySized<T=F>+TryIntoMatrix<matrix_wrappers::$o_or_u_m<MQ>>,
//          MDV : MatrixDynamicallySized<T=V>+crate::qr::QRImplBase<MDF=MDF>+TryIntoMatrix<RightTriangular<M>>>
//          crate::$o_or_u_qr for M {
//         paste::paste!(
//         type Q=matrix_wrappers::[<Special $o_or_u_m>]<MQ>;
//         type R=RightTriangular<M>;
//         fn qr(self) -> (matrix_wrappers::[<Special $o_or_u_m>]<MQ>, RightTriangular<M>) {
//             let sd:MDV=self.into_dyn_matrix();
//             let (q,r,nh)=sd.qr_impl_base(); // $crate::qr::qr_impl_base(self);
//             let (q,r)=if nh % 2 == 1 { // make special
//                 (q.try_neg_col(0).unwrap(),
//                  r.try_neg_row(0).unwrap())
//             } else { (q,r) };
//             let q:matrix_wrappers::$o_or_u_m::<MQ>=
//                   TryIntoMatrix::try_into_matrix(q).unwrap();
//             let q=matrix_wrappers::[<Special $o_or_u_m>]::<MQ>::try_new(q,F::one()).ok().unwrap();
//             let r=TryIntoMatrix::try_into_matrix(r).unwrap();
//             (q,r)
//         });
//     }}
// }

// impl_qr_non_tall_single!(OrthogonalQR, Orthogonal, RealNumber   );
// impl_qr_non_tall_single!(UnitaryQR,    Unitary,    ComplexNumber);



//         paste::paste!(
//         impl<F    : 'static+Clone+$r_c_or_s+Mul<V,Output=V> $( +nalgebra::$tr)?,
//              V    : 'static+Clone+Mul<F::RealType,Output=V>
//                     +TryDiv<Output=F>
//                     +InnerProductSpace1d $(+nalgebra::$tr)?,
//              MDF  : MatrixDynamicallySized<T=F>
//                    +TryIntoMatrix<matrix_wrappers::$o_or_u_m<$q>>,
//              SelfDyn : MatrixDynamicallySized<T=V>
//                       +TryIntoMatrix<matrix_wrappers::$r_or_sr<$r>>+$crate::qr::QRImplBase<T=V,MDF=MDF>>
//              $crate::$o_or_u_qr for $m
//              where  $q : Clone+TryMatrixMatrixProduct<$r,Output=$m>,
//                     $r : Clone+matrix_traits::MatrixTryConstruct<T=V>,
//                     Self : matrix_traits::Matrix<T=V>+IntoDynMatrix<Output=SelfDyn> {
//     type Q=matrix_wrappers::[<Special $o_or_u_m>]<$q>;
//     type R=$r_or_sr<$r>;
//     fn qr(self) -> (<Self as $crate::$o_or_u_qr>::Q, <Self as $crate::$o_or_u_qr>::R) {
//         // use $crate::qr::QRImplBase;
//         use IntoDynMatrix;
//         let sd:SelfDyn=self.into_dyn_matrix();//<Self as IntoDynMatrix>::into_dyn_matrix(self);
//         let (q,r,nh)=sd.qr_impl_base(); // $crate::qr::qr_impl_base(self);
//         let (q,r)=if nh % 2 == 1 { // make special
//             (q.try_neg_col(0).unwrap(),
//              r.try_neg_row(0).unwrap())
//         } else { (q,r) };
//         let q:matrix_wrappers::$o_or_u_m::<$q>=
//               TryIntoMatrix::try_into_matrix(q).unwrap();
//         let q=matrix_wrappers::[<Special $o_or_u_m>]::<$q>::try_new(q,F::one()).ok().unwrap();
//         let r=TryIntoMatrix::try_into_matrix(r).unwrap();
//         (q,r)
//     }
//     });
// }}






// #[macro_export]
// macro_rules! impl_qr_non_tall_static {
//     ($name:ident, $i:literal, $j:literal $(where V : nalgebra::$tr:ident)? ) => {
//         $crate::impl_qr_non_tall!($name<V,$i,$j>, $name<F,$i,$i>, $name<V,$i,$j>, RightTriangular $( where V:nalgebra::$tr)?);
//     }
// }


// #[macro_export]
// macro_rules! impls_qr_tall_static {
//     ($i0:literal, $name:ident) => {};

//     ($i0:literal $(,$i:literal)+ ,$name:ident) => {
//         $( $crate::impl_qr_tall_static!($name, $i, $i0); )*
//         $crate::impls_qr_tall_static!($($i,)* $name);
//     }
// }

// #[macro_export]
// macro_rules! impls_qr_non_tall_static {
//     ($i0:literal, $name:ident) => {};

//     ($i0:literal $(,$i:literal)+ ,$name:ident) => {
//         $crate::impl_qr_non_tall_static!($name, $i0, $i0);
//         $( $crate::impl_qr_non_tall_static!($name, $i0, $i); )*
//         $crate::impls_qr_non_tall_static!($($i,)* $name);
//     }
// }


// impl_qr_non_tall!(Wide<MatrixDyn<V>>,  SquareMatrixDyn<F>, Wide<MatrixDyn<V>>, RightTriangular);
// impl_qr_non_tall!(SquareMatrixDyn<V>, SquareMatrixDyn<F>, SquareMatrixDyn<V>, SquareRightTriangular);



#[cfg(test)]
mod tes_with_nalgebra {
    use nalgebra::{DMatrix,SMatrix};
    // impl_qr_tall!    (       DMatrix<V>,         DMatrix<F>,         DMatrix<V>,        RightTriangular where V:nalgebra::Scalar);
    // impl_qr_non_tall!(Wide  <DMatrix<V>>, Square<DMatrix<F>>, Wide  <DMatrix<V>>,       RightTriangular where V:nalgebra::Scalar);
    // impl_qr_non_tall!(Square<DMatrix<V>>, Square<DMatrix<F>>, Square<DMatrix<V>>, SquareRightTriangular where V:nalgebra::Scalar);

    // because of where V : nalgebra::Scalar trait bound we define its own macro
    // macro_rules! impls_qr_tall_nalgebra_static {
    //     ($i0:literal, $name:ident) => {};
    
    //     ($i0:literal $(,$i:literal)+ ,$name:ident) => {
    //         $( $crate::impl_qr_tall_static!($name, $i, $i0 where V : nalgebra::Scalar); )*
    //         impls_qr_tall_nalgebra_static!($($i,)* $name);
    //     }
    // }
    // impls_qr_tall_nalgebra_static!(1,2,3,4, SMatrix);

    // macro_rules! impls_non_tall_nalgebra {
    //     ($i0:literal, $name:ident) => {};
    
    //     ($i0:literal $(,$i:literal)+ ,$name:ident) => {
    //            impl_qr_non_tall_static!($name, $i0, $i0 where V : nalgebra::Scalar);
    //         $( $crate::impl_qr_non_tall_static!($name, $i0, $i  where V : nalgebra::Scalar); )*
    //         impls_non_tall_nalgebra!($($i,)* $name);
    //     }
    // }
    // impls_non_tall_nalgebra!(1,2,3,4, SMatrix);

    use super::super::qr_impl_base::check_qr;

    #[test]
    fn test_qr_smatrix() {
        let m: nalgebra::SMatrix<f64, 3,2>=nalgebra::matrix![1.0,2.0;3.0,4.0;5.0,6.0];
        check_qr(m.clone());
        let m_dyn:DMatrix<f64>=m.into_dyn_matrix();
        check_qr(m_dyn);
    }


    #[test]
    fn test_new_qr() {
        use algebra_traits::TryMaxNormOfEntries;
        use matrix_traits::TryMatrixMatrixProduct;
        let m:SMatrix<f64,3,2>=nalgebra::matrix![1.0, 0.0; 0.0, 1.0; -1.0, 1.0];
        check_qr(m.clone());
        let (q,r)=m.clone().into_qr().into_parts();
        println!("{}",q);
        println!("{}",r);
        let qr=q.try_matrix_matrix_product(r).unwrap();
        //let qrs:SMatrix<f64,3,2>=qr.try_into_matrix().unwrap();
        let err:SMatrix<f64,3,2>=qr-m;
        assert!(err.try_max_norm_of_entries().unwrap() < 1e-12);
    }
}

// impl<F,
//      Row         : RowVectorTryConstruct<T=F>,
//      Col         : ColVectorTryConstruct<T=F>,
//      CroppedRowT : ColVectorTryConstruct<T=F>,
//      M           : Matrix<F=F,Row=Row,Col=Col>> QR for UnknownShape<M> where
//     Cropped<Row> : Transpose<Output=CroppedRowT> {
//     type Q = Stiefel<        MatrixGeneric<Cropped<Row>, Col>>;
//     type R = RightTriangular<MatrixGeneric<Row,          CroppedRowT>>
// }