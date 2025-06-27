
use algebra_traits::{Scalar, Sqrt, TryAdd, TryDiv, TryMul, TrySub, ScalarMul, CastFromf64};

use algebra_traits::operators::div_by_small_natural::Div2;
use num_traits::{Zero, One, Inv};

use super::square_dyn::SquareMatrixDyn;

use crate::utils::kronecker_delta as delta;

#[derive(Clone,
         Debug,
         derive_more::Into,
         derive_more::Index)]
pub struct NearIdentityMatrixDyn<F>(SquareMatrixDyn<F>);

crate::inherit_arithmetic!(NearIdentityMatrixDyn);
crate::inherit_square!(NearIdentityMatrixDyn);
crate::new_unchecked_square!(NearIdentityMatrixDyn);
utils::inherit_map!(NearIdentityMatrixDyn<T>);


impl<F:Clone+Scalar> TryFrom<SquareMatrixDyn<F>> for NearIdentityMatrixDyn<F> {
    type Error = SquareMatrixDyn<F>;
    fn try_from(value:SquareMatrixDyn<F>) -> Result<Self,Self::Error> {
        for i in 0..value.n() {
            for j in 0..value.n() {
                if  (value[(i,j)].clone()-delta(i,j)).norm() > F::RealType::from_f64(0.1) {
                    return Err(value)
                }
            }
        }
        Ok(Self(value))
    }
}

impl<F:Zero+One> NearIdentityMatrixDyn<F> {
    pub fn identity(m:usize) -> Self {
        Self(SquareMatrixDyn::<F>::identity(m))
    }
}


// fn from<F:Scalar>(f:f64) -> F {
//     F::from(F::RealType::from(f))
// }

fn from<F:Scalar>(i:i16) -> F {
    F::from(F::RealType::from(i))
}


impl<F:Clone+Scalar> NearIdentityMatrixDyn<F> {
    // ^(-1/2)
    // note: this algo does not need a try_div
    pub fn msqrt(self) -> Self {
        let id=SquareMatrixDyn::<F>::identity(self.n());
        // initial guess
        let mut msqrt=id.clone();
        // use newtons-method
        for _ in 0..5 {
            let msqrt2=msqrt.pow2();
            let selfmsqrt2=<SquareMatrixDyn<F> as TryMul>::try_mul(self.0.clone(),msqrt2).unwrap();
            let fac=(id.clone().scalar_mul(&from::<F>(3 as i16).div2())).try_sub(selfmsqrt2.map(Div2::div2)).unwrap();
            msqrt=<SquareMatrixDyn<F> as TryMul>::try_mul(msqrt, fac).unwrap();
        }
        Self(msqrt)
    }
}

impl<F:Clone+Scalar> TryDiv for NearIdentityMatrixDyn<F> {
    type Output=Self;
    fn try_div(self, rhs:Self) -> Option<Self> {
        let rhs:Self=rhs.into();
        if self.n() != rhs.n() {
            return None;
        }
        let id=SquareMatrixDyn::<F>::identity(self.n());
        let rhs_inv_appr=id.clone().try_sub(rhs.0.clone().scalar_mul(&from::<F>(2))).unwrap();
        let fac=id.try_sub(rhs_inv_appr.clone().try_mul(rhs.0).unwrap()).unwrap();
        let c:SquareMatrixDyn<F>=rhs_inv_appr.clone().try_mul(self.0.clone()).unwrap();
        let mut res:SquareMatrixDyn<F>=self.0.clone().try_mul(rhs_inv_appr).unwrap();
        // fixpointiteration
        for _ in 0..5 {
            let mul=<SquareMatrixDyn<F> as TryMul>::try_mul(fac.clone(), res.clone()).unwrap();
            res=<SquareMatrixDyn<F> as TryAdd>::try_add(mul, c.clone()).unwrap()
        }
        Some(Self(res))
    }
}

impl<F:Clone+Scalar> Sqrt for NearIdentityMatrixDyn<F> {
    type Output=Self;
    fn sqrt(self) -> Self {
        let id=SquareMatrixDyn::<F>::identity(self.n());
        // initial guess
        let mut sqrt=id.clone();
        // use herons-method
        for _ in 0..5 {
            sqrt=sqrt.clone()
                     .try_add(self.clone()
                                  .try_div(Self(sqrt.clone())).unwrap().0)
                     .unwrap()
                     .map(Div2::div2);
        }
        Self(sqrt)
    }
}

impl<F:One+Zero> Inv for NearIdentityMatrixDyn<F> where Self : TryDiv {
    type Output=<Self as TryDiv>::Output;
    fn inv(self) -> Self::Output {
        let n=self.n();
        Self::identity(n).try_div(self).unwrap()
    }
}