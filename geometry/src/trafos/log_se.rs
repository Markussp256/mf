use core::fmt::Debug;

//use derive_more::{Add, Sub, Neg};
// doesnt work since T would need to be restricted in struct definition

use algebra_traits::{Exp, RealNumber, TryDiv, TryLog, Vectorspace, Vectorspace1d, DivError, LogError, ScalarMul, TryScalarDiv};
use algebra::{Complex, special_functions::{expm1dz, lndzm1}};
use crate::Vector;

use crate::trafos::{Translation, SE};


use matrix_wrappers::SkewSymmetricMatrix;
use matrix::Matrix;

use std::ops::Mul;

#[derive(
    Clone,
    PartialEq,
    Debug,
    algebra_derive::Vectorspace,
    container_derive::IntoParameters,
    container_derive::TryFromParameters,
)]
pub struct LogSE<F : RealNumber,V,const N:usize> {
    lnrot: SkewSymmetricMatrix<F,N>,
    t: Vector<V,N>
}

// impl<F:Clone+Scalar, V:Vectorspace<F>, const N:usize> ScalarMul<F> for LogSE<F, V, N> {
//     fn scalar_mul(self, rhs: &F) -> Self {
//         Self{
//             lnrot: self.lnrot.scalar_mul(rhs),
//             t: self.t.scalar_mul(rhs)
//         }
//     }
// }

impl<F:Clone+RealNumber, V:Vectorspace<F>, const N:usize> Mul<F> for LogSE<F, V, N> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self {
        Self{
            lnrot: self.lnrot.scalar_mul(&rhs),
            t: self.t.scalar_mul(&rhs)
        }
    }
}

impl<F:Clone+RealNumber, V:TryDiv<Output=F>+Vectorspace1d, const N:usize> TryDiv<F> for LogSE<F, V, N> {
    type Output = Self;
    fn try_div(self, rhs: F) -> Result<Self::Output,DivError> {
        Ok(Self{
            lnrot: self.lnrot.try_scalar_div(&rhs)?,
            t: self.t.try_scalar_div(&rhs)?
        })
    }
}

// impl < T, const N : usize > Vectorspace for LogSE < T, N > where T :
// Vectorspace < f64 > { type f64 ; } impl < T, const N : usize >
// AdditiveGroup for LogSE < T, N > where T : AdditiveGroup {} impl < T, const N
// : usize > std :: ops :: Mul :: < f64 > for LogSE < T, N > where T : std :: ops
// :: Mul < f64, Output = T >
// {
//     type Output = Self ; fn mul(self, rhs : f64) -> Self
//     {
//         {
//             let lnrot = < SkewSymmetricMatrix< f64, N > as std :: ops :: Mul :: < f64 > > ::
//             mul(self.lnrot rhs) ; let t = < Vector < T, N > as std :: ops ::
//             Mul :: < f64 > > :: mul(self.t rhs) ; Self { lnrot, t }
//         }
//     }
// } impl < T, const N : usize > std :: ops :: Mul < LogSE < T, N > > for f64
// where LogSE < T, N > : std :: ops :: Mul < f64, Output = LogSE < T, N > >
// {
//     type Output = LogSE < T, N > ; fn mul(self, rhs : LogSE < T, N >) -> Self
//     :: Output { rhs * self }
// } impl < T, const N : usize > std :: ops :: Div :: < f64 > for LogSE < T, N >
// where T : std :: ops :: Div < f64, Output = T >
// {
//     type Output = Self ; fn div(self, rhs : f64) -> Self
//     {
//         {
//             let lnrot = < SkewSymmetricMatrix< f64, N > as std :: ops :: Div :: < f64 > > ::
//             div(self.lnrot rhs) ; let t = < Vector < T, N > as std :: ops ::
//             Div :: < f64 > > :: div(self.t rhs) ; Self { lnrot, t }
//         }
//     }
// } impl < T, const N : usize > num_traits :: Zero for LogSE < T, N > where T :
// num_traits :: Zero + std :: ops :: Add < Output = T >
// {
//     fn zero() -> Self
//     {
//         {
//             let lnrot = < SkewSymmetricMatrix< f64, N > as num_traits :: Zero > ::
//             zero(.lnrot) ; let t = < Vector < T, N > as num_traits :: Zero >
//             :: zero(.t) ; Self { lnrot, t }
//         }
//     } fn is_zero(& self,) -> bool
//     {
//         < SkewSymmetricMatrix< f64, N > as num_traits :: Zero > :: is_zero(& self.lnrot) &&
//         < Vector < T, N > as num_traits :: Zero > :: is_zero(& self.t)
//     }
// } impl < T, const N : usize > std :: ops :: Add for LogSE < T, N > where T :
// std :: ops :: Add < Output = T >
// {
//     type Output = Self ; fn add(self, rhs : Self) -> Self
//     {
//         {
//             let lnrot = < SkewSymmetricMatrix< f64, N > as std :: ops :: Add > ::
//             add(self.lnrot, rhs.lnrot) ; let t = < Vector < T, N > as std ::
//             ops :: Add > :: add(self.t, rhs.t) ; Self { lnrot, t }
//         }
//     }
// } impl < T, const N : usize > std :: ops :: Sub for LogSE < T, N > where T :
// std :: ops :: Sub < Output = T >
// {
//     type Output = Self ; fn sub(self, rhs : Self) -> Self
//     {
//         {
//             let lnrot = < SkewSymmetricMatrix< f64, N > as std :: ops :: Sub > ::
//             sub(self.lnrot, rhs.lnrot) ; let t = < Vector < T, N > as std ::
//             ops :: Sub > :: sub(self.t, rhs.t) ; Self { lnrot, t }
//         }
//     }
// } impl < T, const N : usize > std :: ops :: Neg for LogSE < T, N > where T :
// std :: ops :: Neg < Output = T >
// {
//     type Output = Self ; fn neg(self,) -> Self
//     {
//         {
//             let lnrot = < SkewSymmetricMatrix< f64, N > as std :: ops :: Neg > ::
//             neg(self.lnrot) ; let t = < Vector < T, N > as std :: ops :: Neg >
//             :: neg(self.t) ; Self { lnrot, t }
//         }
//     }
// }

pub type LogSE2<F=f64, V = f64> = LogSE<F, V, 2>;
pub type LogSE3<F=f64, V = f64> = LogSE<F, V, 3>;

// crate::impl_parameters!(LogSE2<T:Parameters1|>,lnrot:Skew2<f64>:1,t:Vector2<T>:2);
// crate::impl_parameters!(LogSE3<T:Parameters1|>,lnrot:Skew3<f64>:3,t:Vector3<T>:3);

macro_rules! impl_exp_log {
    ($N:tt) => {
        impl<F:RealNumber, V:Clone+TryDiv<Output=F>+Vectorspace1d> Exp for LogSE<F, V, $N> where F : std::ops::Mul<V, Output=V> {
            type Output=SE<F, V, $N>;
            fn exp(self) -> Self::Output {
                let m=self.lnrot
                          .clone()
                          .into_this::<Eig<Complex<F>,$N>>()
                          .apply_fn(expm1dz)
                          .into_matrix()
                          .into_real();
                let t_alg:algebra::Vector<V, $N>=self.t.into();
                SE::from_parts(self.lnrot.exp(), Translation::from(m * t_alg))
            }
        }

        impl<F:RealNumber+Mul<V,Output=V>, V:Clone+TryDiv<Output=F>+Vectorspace1d> TryLog for SE<F, V, $N> {
            type Output=LogSE<F, V, $N>;
            fn try_log(self) -> Result<Self::Output, LogError> {
                let rot = self.rot();
                let m: Matrix<F, $N, $N> = rot.clone().apply_fn(lndzm1);
                let t:algebra::Vector::<V,$N>=
                self.t()
                    .vector()
                    .clone()
                    .into();
                rot.clone()
                   .try_log()
                   .map(|lnrot| LogSE::<F, V, $N> {lnrot, t:(m*t).into()} )
            }
        }
    };
}

impl_exp_log!(2);
impl_exp_log!(3);