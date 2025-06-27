
use algebra_traits::{ComplexNumber, RealNumber, TryLog, TryPow};
use matrix_decompositions_dyn::eig_dyn::EigRealDyn;
use utils::IntoThis;

use crate::matrix::*;


use crate::{Complex, Matrix, UnitVector, VectorDyn};

macro_rules! eig {
    ($name:ident, $mname:ident, $tr:ident) => {
        paste::paste!(
            pub struct $name<F:$tr, const N:usize>([<$name Dyn>]<F>);

            impl<F:$tr+Clone, const N:usize> $name<F,N> {
                pub fn q(&self) -> $mname<F,N> {
                    self.0
                        .q()
                        .clone()
                        .try_into()
                        .ok()
                        .unwrap()
                }
            
                pub fn eigenvalues(&self) -> [F;N] {
                    let mut iter=self.0
                                     .d()
                                     .diagonal()
                                     .clone()
                                     .into_iter();
                    utils::iter::next_chunk(& mut iter).ok().unwrap()
                }
            
                pub fn eigenvector(self, i:usize) -> UnitVector<F,N> {
                    self.0
                        .q()
                        .col(i)
                        .into_this::<VectorDyn<F>>()
                        .try_into()
                        .ok()
                        .unwrap()
                }
            
                pub fn apply_fn(self, f: impl Fn(F) -> F) -> Self {
                    Self(self.0.apply_fn(f))
                }
            
                pub fn try_apply_fn(self, f: impl Fn(F) -> Option<F>) -> Option<Self> {
                    self.0.try_apply_fn(f)
                        .map(|e|Self(e))
                }
            
                pub fn into_matrix(self) -> Matrix<F,N,N> {
                    self.0
                        .into_matrix()
                        .try_into()
                        .ok().unwrap()
                }
            }


        );
    };
}
eig!(Eig, UnitaryMatrix, ComplexNumber);
eig!(EigReal, OrthogonalMatrix, RealNumber);


// impl<R:RealNumber, const N:usize> From<SymmetricMatrix<R,N>> for Eig<R,N> {
//     fn from(m:SymmetricMatrix<R, N>) -> Self {
//         let eig:EigDyn<R>=EigDyn::<Complex<R>>::new(m.into_complex()).try_into_real().unwrap();
//         Self(eig)
//     }
// }

impl<R:RealNumber, const N:usize> From<SymmetricMatrix<R, N>> for EigReal<R, N> {
    fn from(m:SymmetricMatrix<R, N>) -> EigReal<R, N> {
        Self(EigRealDyn::<R>::new(m))
    }
}

macro_rules! from_real {
    ($name:ident) => {
        impl<R:RealNumber, const N:usize> From<$name<R,N>> for Eig<Complex<R>, N> {
            fn from(m:$name<R, N>) -> Self {
                Self(EigDyn::<Complex<R>>::new(m))
            }
        }
    };
}
from_real!(SkewSymmetricMatrix);
from_real!(OrthogonalMatrix);
from_real!(SpecialOrthogonalMatrix);

macro_rules! from_complex {
    ($name:ident) => {
        impl<C:ComplexNumber, const N:usize> From<$name<C,N>> for Eig<C, N> {
            fn from(m:$name<C, N>) -> Self {
                Self(EigDyn::<C>::new(m))
            }
        }
    };
}
from_complex!(UnitaryMatrix);
from_complex!(SpecialUnitaryMatrix);




// eigenvalues of symmetric matrix are real, therefore function real to real is required
impl<R:RealNumber, const N: usize> SymmetricMatrix<R, N> {
    pub fn apply_fn(self, f: impl Fn(R) -> R) -> Self {
        Self::new_unchecked(
            self.into_this::<EigReal<R,N>>()
                .apply_fn(f)
                .into_matrix())
    }
}

impl<R:RealNumber, const N: usize> SymmetricMatrix<R, N> {
    pub fn try_apply_fn(self, f: impl Fn(R) -> Option<R>) -> Option<Self> {
        self.into_this::<EigReal<R,N>>()
            .try_apply_fn(f)
            .map(|e| Self::new_unchecked(e.into_matrix()))
    }
}


impl<R:RealNumber, const N:usize> TryLog for SymmetricMatrix<R, N> {
    type Output=SymmetricMatrix<R, N>;
    fn try_log(self) -> Option<Self> {
        self.try_apply_fn(<R as TryLog>::try_log)
    }
}


impl<R:RealNumber, const N:usize> TryPow<R> for SymmetricMatrix<R, N> {
    type Output=Self;

    fn try_pow(self, rhs: R) -> Option<Self> {
        self.try_apply_fn(|r| <R as TryPow<R>>::try_pow(r, rhs.clone()))
    }
}