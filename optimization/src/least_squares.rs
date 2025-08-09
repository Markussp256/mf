use std::ops::{Mul,SubAssign};
use algebra_traits::{InnerProductSpace1d, Scalar, TryDiv};

use algebra::VectorDyn;
use matrix::{MatrixColDyn, MatrixDyn};

use matrix_decompositions::QR;

pub fn try_solve_least_squares<
    F : 'static+Scalar+Mul<V,Output=V>,
    V : 'static+SubAssign+InnerProductSpace1d+Clone+TryDiv<Output=F>+Mul<F,Output=V>+Mul<F::RealType,Output=V>>(
        a:MatrixDyn<V>,
        b:VectorDyn<V>) -> Option<VectorDyn<F>> {
    let b:MatrixColDyn<V>=b.try_into().unwrap();
    <MatrixDyn<V> as QR>::try_solve_least_squares(a, b)
        .map(|c:MatrixColDyn<F>|c.into())
}

// 

// pub fn try_solve_least_squares
//     <F   : 'static+Clone+Scalar+Mul<V,Output=V>,
//      V   : 'static+Clone+TryDiv<Output=F>+Mul<F::RealType,Output=V>+Mul<F,Output=V>+InnerProductSpace1d,
//      MQR : QR<M=MatrixDyn<V>,F=F,V=V>,
//      E>(a:MatrixDyn<V>, b:VectorDyn<V>) -> Option<VectorDyn<F>>
//      where MQR::Q : ConjugateTranspose,
//      <MQR::Q as Transpose>::Output : TryMatrixVectorProduct<VectorDyn<V>,T=F>,
//      MQR::R : TrySolve<VectorDyn<V>,E,Output=VectorDyn<F>> {
//         MQR::new(a)
//             .try_solve_least_squares(b)
// }

    // doent work, error is "unable to solve nonsquare system"
    // a.qr().solve(&b)
    // let qr=a.qr();
    // let qtb=qr.q().transpose()*&b;
    // let r=qr.unpack_r();
    // let res=r.solve_upper_triangular(&qtb);
    // res

       // match a.pseudo_inverse(1e-10) {
    //     Ok(pinv) => (pinv*b).into(),
    //     _ => None
    // }