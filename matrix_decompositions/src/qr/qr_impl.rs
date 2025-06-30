// nh is number of householder transforms

// for square matrix we can make Q special, i.e. det=1 so that we can compute determinant
// by computing determinant of R

// use matrix_traits::{MatrixTryConstruct, TryIntoMatrix};
// use matrix_wrappers::{SpecialOrthogonal, Orthogonal, SpecialUnitary, Unitary, SpecialStiefel, Wide, RightTriangular, SquareRightTriangular, Stiefel};

#[cfg(feature = "nalgebra_support")]
fn clone_col<F:nalgebra::Scalar>(m:matrix_wrappers::Wide<nalgebra::DMatrix<F>>) -> nalgebra::DVector<F> {
    let c=matrix_traits::Matrix::col(&m,0).unwrap();
    let c2=(&c).clone();
    c2
}

#[cfg(feature = "nalgebra_support")]
fn clone_m<F:nalgebra::Scalar>(m:&nalgebra::DMatrix<F>) -> nalgebra::DMatrix<F> {
    m.clone()
}

#[macro_export]
macro_rules! impl_qr_tall {
    ($m:ty, $q:ty, $r:ty, $r_or_sr:ident $(where V : nalgebra::$tr:ident)?) => {
        impl<F : 'static+Clone+algebra_traits::Scalar+std::ops::Mul<V,Output=V> $(+nalgebra::$tr)?,
             V : 'static+Clone+std::ops::SubAssign<V>+algebra_traits::TryDiv<Output=F>+num_traits::Zero+std::ops::Mul<F::RealType,Output=V>+algebra_traits::InnerProductSpace1d $(+nalgebra::$tr)?>
              $crate::QR for $m
        where $q : matrix_traits::AnyMatrixMatrixProduct<$r,Output=$m> {
    type Q=matrix_wrappers::Stiefel<$q>;
    type R=matrix_wrappers::$r_or_sr<$r>;
    fn qr(self) -> (<Self as $crate::QR>::Q,<Self as $crate::QR>::R) {
        let (q,r,_)=$crate::qr::qr_impl_base::qr_impl_base(self);
        let q=matrix_traits::TryIntoMatrix::try_into_matrix(q).unwrap();
        let r=matrix_traits::TryIntoMatrix::try_into_matrix(r).unwrap();
        (q,r)
    }
}}}

#[macro_export]
macro_rules! impl_qr_non_tall_single {
    ($m:ty, $q:ty, $r:ty, $o_or_u_qr:ident, $o_or_u_m:ident, $r_c_or_s:ident, $r_or_sr:ident $(where V : nalgebra::$tr:ident)? ) => {
        paste::paste!(
        impl<F    : 'static+Clone+algebra_traits::$r_c_or_s+std::ops::Mul<V,Output=V> $( +nalgebra::$tr)?,
             V    : 'static+Clone+algebra_traits::TryDiv<Output=F>+std::ops::Mul<F::RealType,Output=V>+algebra_traits::InnerProductSpace1d $(+nalgebra::$tr)?>
             $crate::$o_or_u_qr for $m
             where Self : Clone+matrix_traits::IntoDynMatrix<T=V>,
                    $q : matrix_traits::IntoDynMatrix<T=F>+Clone+matrix_traits::AnyMatrixMatrixProduct<$r,Output=$m>,
                    $r : matrix_traits::IntoDynMatrix<T=V>+Clone {
    type Q=matrix_wrappers::[<Special $o_or_u_m>]<$q>;
    type R=matrix_wrappers::$r_or_sr<$r>;
    fn qr(self) -> (<Self as $crate::$o_or_u_qr>::Q, <Self as $crate::$o_or_u_qr>::R) {
        use matrix_traits::MatrixTryConstruct;
        let (q,r,nh)=$crate::qr::qr_impl_base(self);
        let (q,r)=if nh % 2 == 1 { // make special
            (q.try_neg_col(0).unwrap(),
             r.try_neg_row(0).unwrap())
        } else { (q,r) };
        let q:matrix_wrappers::$o_or_u_m::<$q>=
              matrix_traits::TryIntoMatrix::try_into_matrix(q).unwrap();
        let q=matrix_wrappers::[<Special $o_or_u_m>]::<$q>::try_new(q,F::one()).unwrap();
        let r=matrix_traits::TryIntoMatrix::try_into_matrix(r).unwrap();
        (q,r)
    }
    });
}}

#[macro_export]
macro_rules! impl_qr_non_tall {
    ($m:ty, $q:ty, $r:ty, $r_or_sr:ident $(where V : nalgebra::$tr:ident)? ) => {
        $crate::impl_qr_non_tall_single!($m, $q, $r, QR,           Stiefel,    Scalar,        $r_or_sr $(where V : nalgebra::$tr)?);
        $crate::impl_qr_non_tall_single!($m, $q, $r, OrthogonalQR, Orthogonal, RealNumber,    $r_or_sr $(where V : nalgebra::$tr)?);
        $crate::impl_qr_non_tall_single!($m, $q, $r, UnitaryQR,    Unitary,    ComplexNumber, $r_or_sr $(where V : nalgebra::$tr)?);
    }
}


// squareness of R is known at compile time so we do not have to use SquareRightTriangular
#[macro_export]
macro_rules! impl_qr_tall_static {
    ($name:ident, $i:literal, $j:literal $(where V : nalgebra::$tr:ident)? ) => {
        $crate::impl_qr_tall!($name<V,$i,$j>, $name<F,$i,$j>, $name<V,$j,$j>, RightTriangular $( where V:nalgebra::$tr)?);
    }
}

#[macro_export]
macro_rules! impl_qr_non_tall_static {
    ($name:ident, $i:literal, $j:literal $(where V : nalgebra::$tr:ident)? ) => {
        $crate::impl_qr_non_tall!($name<V,$i,$j>, $name<F,$i,$i>, $name<V,$i,$j>, RightTriangular $( where V:nalgebra::$tr)?);
    }
}

#[macro_export]
macro_rules! impls_qr_tall_static {
    ($i0:literal, $name:ident) => {};

    ($i0:literal $(,$i:literal)+ ,$name:ident) => {
        $( $crate::impl_qr_tall_static!($name, $i, $i0); )*
        $crate::impls_qr_tall_static!($($i,)* $name);
    }
}

#[macro_export]
macro_rules! impls_qr_non_tall_static {
    ($i0:literal, $name:ident) => {};

    ($i0:literal $(,$i:literal)+ ,$name:ident) => {
        $crate::impl_qr_non_tall_static!($name, $i0, $i0);
        $( $crate::impl_qr_non_tall_static!($name, $i0, $i); )*
        $crate::impls_qr_non_tall_static!($($i,)* $name);
    }
}

#[cfg(feature = "nalgebra_support")]
mod impl_nalgebra {
    use nalgebra::{DMatrix,SMatrix};
    use matrix_wrappers::{Square,Wide};
    impl_qr_tall!    (       DMatrix<V>,         DMatrix<F>,         DMatrix<V>,        RightTriangular where V:nalgebra::Scalar);
    impl_qr_non_tall!(Wide  <DMatrix<V>>, Square<DMatrix<F>>, Wide  <DMatrix<V>>,       RightTriangular where V:nalgebra::Scalar);
    impl_qr_non_tall!(Square<DMatrix<V>>, Square<DMatrix<F>>, Square<DMatrix<V>>, SquareRightTriangular where V:nalgebra::Scalar);

    // because of where V : nalgebra::Scalar trait bound we define its own macro
    macro_rules! impls_qr_tall_nalgebra_static {
        ($i0:literal, $name:ident) => {};
    
        ($i0:literal $(,$i:literal)+ ,$name:ident) => {
            $( $crate::impl_qr_tall_static!($name, $i, $i0 where V : nalgebra::Scalar); )*
            impls_qr_tall_nalgebra_static!($($i,)* $name);
        }
    }
    impls_qr_tall_nalgebra_static!(1,2,3,4, SMatrix);

    macro_rules! impls_non_tall_nalgebra {
        ($i0:literal, $name:ident) => {};
    
        ($i0:literal $(,$i:literal)+ ,$name:ident) => {
               impl_qr_non_tall_static!($name, $i0, $i0 where V : nalgebra::Scalar);
            $( $crate::impl_qr_non_tall_static!($name, $i0, $i  where V : nalgebra::Scalar); )*
            impls_non_tall_nalgebra!($($i,)* $name);
        }
    }
    impls_non_tall_nalgebra!(1,2,3,4, SMatrix);

    #[test]
    fn test_new_qr() {
        use algebra_traits::TryMaxNormOfEntries;
        use matrix_traits::AnyMatrixMatrixProduct;
        let m:SMatrix<f64,3,2>=nalgebra::matrix![1.0, 0.0; 0.0, 1.0; -1.0, 1.0];
        super::super::qr_impl_base::check_qr(m.clone());
        let (q,r)=super::super::QR::qr(m.clone());
        println!("{}",q);
        println!("{}",r);
        let err:SMatrix<f64,3,2>=q.any_matrix_matrix_product(r).unwrap()-m;
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