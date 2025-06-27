use crate::{Distance, NormSquared, Nonnegative};
use num_traits::Zero;
use container_traits::IntoSum;

// normally the Squared norm is defined via the scalarproduct by
// taking the scalar product with itself. However,
// the squared norm can exist in cases where the scalar product
// does not exist, for example for dynamically sized vectors.
// Scalar product can also be defined when the Rhs is not self

pub trait TryScalarproduct {
    type TryScProdT : Zero;
    fn try_scalar_product(self, rhs:Self) -> Option<Self::TryScProdT>;
}

macro_rules! impl_try_sc_prod {
    ($t:ty $(, const $n:ident : usize)?) => {
        impl<ScProdT:Zero,T:Scalarproduct<ScProdT = ScProdT> $(, const $n : usize)?> TryScalarproduct for $t {
            type TryScProdT = ScProdT;
            fn try_scalar_product(self, rhs:Self) -> Option<ScProdT> {
                (self.len() == rhs.len()).then(||
                    self.into_iter()
                        .zip(rhs.into_iter())
                        .map(|(a,b)|a.scalar_product(b))
                        .into_sum()
                )
            }
        }
    };
}
impl_try_sc_prod!(Vec<T>);
impl_try_sc_prod!([T;N],const N:usize);


pub trait Scalarproduct {
    type ScProdT;
    fn scalar_product(self, rhs:Self) -> Self::ScProdT;
}
impl<ScProdT:Zero, T:Scalarproduct<ScProdT = ScProdT>, const N:usize> Scalarproduct for [T;N] {
    type ScProdT = ScProdT;
    fn scalar_product(self, rhs:Self) -> Self::ScProdT {
        self.try_scalar_product(rhs).unwrap()
    }
}



// pub trait DotProduct : Conjugate+MapScalars+ScalarContainerStat
//     where Self::F : DotProduct {
//     fn scalar_product(self, rhs: Self) -> Self::F {
//         self.iter_scalars()
//             .zip(rhs.iter_scalars())
//             .map(|a,b|a.dot_product(b))
//             .fold(Self::F::zero(),|acc, new|acc+new)
//     }
// }


// use num_traits::Zero;
// pub trait TryDotProduct : Conjugate+MapScalars+ScalarContainerDyn 
//     where Self::F : DotProduct {
//     fn try_scalar_product(self, rhs: Self) -> Option<Self::F> {
//             (self.dimensions() == rhs.dimensions()
//           && self.uniformly_sized()
//           && rhs.uniformly_sized()).then(||
//                 self.iter_scalars()
//                     .zip(rhs.iter_scalars())
//                     .map(|a,b|a.dot_product(b))
//                     .fold(Self::F::zero(),|acc, new|acc+new))
//     }
// }

// pub trait TryDotProduct : Conjugate+MapScalars {
//     fn try_scalar_product()

// }





pub fn test_consistency_scalar_product_squared_norm<
    T:Clone+Scalarproduct<ScProdT=SP>+NormSquared<Norm2T=NS>,
    NS:Into<SP>,
    SP:Distance<DistT=D>,
    D:PartialOrd>(t:T, tol:Nonnegative<D>) {
    let res1=t.clone()
                  .scalar_product(t.clone());
    let res2:SP=t.clone()
                 .norm_squared()
                 .into_signed()
                 .into();
    assert!(res1.distance(res2) < tol.into_signed())
}

// for array
// impl<T: Scalarproduct<ScProdT = TR>, TR: Zero, const N: usize> // note: Zero also includes Add 
//     Scalarproduct for [T;N] {
//         type ScProdT = TR;
//         fn scalar_product(self, rhs: [T;N]) -> TR {
//             self.into_iter()
//                 .zip(rhs.into_iter())
//                 .map(|(lhs,rhs)|lhs.scalar_product(rhs))
//                 .fold(TR::zero(), |acc, new| acc + new)
//         }
// }



