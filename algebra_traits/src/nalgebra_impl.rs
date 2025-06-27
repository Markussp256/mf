use container_traits::{Map,Size, SizesNotEqualError};
use num_traits::Zero;
use std::ops::Mul;
use crate::*;

use nalgebra::{
    allocator::Allocator, ComplexField, DMatrix, DefaultAllocator, Dim, Matrix, OMatrix, RawStorage, RawStorageMut, SMatrix, Scalar, SquareMatrix, Storage, Vector3
};

type U2=(usize,usize);

impl<T  : Scalar+Mul<T2,Output=T3>,
     T2 : Scalar,
     T3 : Sub<Output=TR>,
     TR : Scalar> Crossproduct<Vector3<T2>> for Vector3<T> {
    type Output = Vector3<TR>;
    fn cross_product(self, rhs:Vector3<T2>) -> Self::Output {
            Vector3::from_fn(|i,_|{
                let i1=(i+1) % 3;
                let i2=(i+2) % 3;
                self[i1].clone()*rhs[i2].clone()
               -self[i2].clone()*rhs[i1].clone()})
    }
}

impl<ScProdT : Zero,
      F : Scalar+Scalarproduct<ScProdT = ScProdT>,
      R : Dim,
      C : Dim,
      S : RawStorage<F,R,C>> TryScalarproduct for Matrix<F,R,C,S> {
    type TryScProdT = ScProdT;
    fn try_scalar_product(self, rhs:Self) -> Option<ScProdT> {
       (self.nrows() == rhs.nrows() &&
        self.ncols() == rhs.ncols()).then(||
        self.iter().cloned()
            .zip(rhs.iter().cloned())
            .map(|(l,r)|l.scalar_product(r))
            .fold(ScProdT::zero(),|acc,new|acc+new))
    }
}

impl< F : Scalar+Zero,
      R : Dim,
      C : Dim,
      S : RawStorage<F,R,C>> IsAZero for Matrix<F,R,C,S> {
    fn is_a_zero(&self) -> bool {
       self.into_iter()
           .all(Zero::is_zero)
    }
}

impl<ScProdT : Zero,
    F : Scalar+Scalarproduct<ScProdT = ScProdT>,
    const M:usize,
    const N:usize> Scalarproduct for SMatrix<F,M,N> {
    type ScProdT=ScProdT;
    fn scalar_product(self, rhs:Self) -> Self::ScProdT {
        self.try_scalar_product(rhs).unwrap()
    }
}

impl<T  : NormSquared<Norm2T = TR>+Scalar,
     TR : Zero+Max,
     R  : Dim,
     C  : Dim,
     S  : RawStorage<T,R,C>> NormSquared for Matrix<T,R,C,S> {
    type Norm2T=TR;

    fn norm_squared(self) -> Nonnegative<Self::Norm2T> {
        self.iter()
            .cloned()
            .map(NormSquared::norm_squared)
            .fold(<crate::Nonnegative<TR> as num_traits::Zero>::zero(),
                  |acc,new|acc+new)
    }
}

crate::impl_norm_from_squared_norm_generic!(
    impl Norm for Matrix<T,R,C,S>
    where T : Scalar,
          R : Dim,
          C : Dim,
          S : RawStorage<T,R,C>);

impl<T  : Clone+Scalar,
     NT : Clone, 
     R  : Dim,
     C  : Dim,
     S  : RawStorage<T,R,C>> TryNormalize for Matrix<T,R,C,S>
where Self : Clone+Norm<NormT=NT>
            +TryDiv<NT,Output=Self> {}

macro_rules! add_or_sub {
    ($uc:ident,$lc:ident) => {
        paste::paste!(
            impl<F : Scalar+std::ops::[<$uc Assign>],
                 R : Dim,
                 C : Dim,
                 S : RawStorageMut<F,R,C>> [<Try $uc>] for Matrix<F,R,C,S> {
                type Output = Self;
                type Error  = SizesNotEqualError<U2>;
                fn [<is_ $lc able_by>](&self,rhs:&Self) -> Result<(),SizesNotEqualError<U2>> {
                    SizesNotEqualError::try_new(self.size(),rhs.size())
                }
                
                fn [<try_ $lc>](self, rhs:Self) -> Result<Self,SizesNotEqualError<U2>> {
                    self.[<is_ $lc able_by>](&rhs)?;
                    let mut s=self;
                    s.iter_mut()
                     .zip(rhs.iter().cloned())
                     .for_each(|(l,r)|l.[<$lc _assign>](r));
                    Ok(s)
                }
            }
            

            // impl<F : Scalar+std::ops::[<$uc Assign>]> [<Try $uc>] for DMatrix<F> {
            //     $crate::impl_tryop_from_anyop_impl!($uc, $lc,by rhs:Self);
            // }

            // impl<F : Scalar+std::ops::[<$uc Assign>]> [<Try $uc>] for DVector<F> {
            //     $crate::impl_tryop_from_anyop_impl!($uc, $lc,by rhs:Self);
            // }
            // impl<F : Scalar+std::ops::[<$uc Assign>]> [<Try $uc>] for RowDVector<F> {
            //     $crate::impl_tryop_from_anyop_impl!($uc, $lc,by rhs:Self);
            // }
        );
    };
}
add_or_sub!(Add,add);
add_or_sub!(Sub,sub);


impl<T: Scalar, E,
     D: Norm<NormT=NormT>,
     NormT,
     R: Dim,
     C: Dim,
     S: RawStorage<T,R,C>> TryDistance for Matrix<T,R,C,S> where Self : TrySub<Output=D,Error=E> {
        type TryDistT=NormT;
        type Error=E;
    fn try_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<Self::TryDistT>,E> {
        let rhs:Self=rhs.into();
        rhs.try_sub(self)
           .map( |d|d.norm())
    }
}

// macro_rules! impl_try_dist {
//     ($name:ident) => {
//         impl<T: Scalar,
//              D: Norm<NormT=NormT>,
//              NormT> TryDistance for $name<T> where Self : TrySub<Output=D> {
//             type TryDistT=NormT;
//             type Error=<Self as TrySub>::Error;
//             fn try_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<NormT>,<Self as TrySub>::Error> {
//                 let rhs:Self=rhs.into();
//                 rhs.try_sub(self)
//                    .map( |d|d.norm())
//             }
//         }
//     };
// }
// impl_try_dist!(DVector);
// impl_try_dist!(RowDVector);
// impl_try_dist!(DMatrix);

impl<T  : Conjugate+Scalar,
     R  : Dim,
     C  : Dim> Conjugate for OMatrix<T,R,C> where DefaultAllocator : Allocator<R,C> {
    fn conjugate(self) -> Self {
        self.map(Conjugate::conjugate)
    }
}

impl<T : Scalar+ScalarMul<F>,
     F,
     R : Dim,
     C : Dim,
     S : RawStorageMut<T,R,C>> ScalarMul<F> for Matrix<T,R,C,S> {
    fn scalar_mul(self, f:&F) -> Self {
        let mut s=self;
        s.iter_mut()
         .for_each(|t|*t=t.clone().scalar_mul(f));
        s
    }
}


 impl<T : Scalar+TryScalarDiv<F,Error=E>,E,
      F,
      R : Dim,
      C : Dim,
      S : RawStorageMut<T,R,C>> TryScalarDiv<F> for Matrix<T,R,C,S> {
    type Error=E;
    fn try_scalar_div(self, f:&F) -> Result<Self,E> {
        let mut s=self;
        for t in s.iter_mut() {
            *t=t.clone().try_scalar_div(f)?
        }
        Ok(s)
    }
}


 impl<T : Scalar+crate::TryDiv<F,Output = T2, Error=E>, E : std::fmt::Debug,
      T2: Scalar,
      F : Clone,
      R : Dim,
      C : Dim,
      S : RawStorage<T,R,C>> crate::TryDiv<F> for Matrix<T,R,C,S>
      where Self : Map<T,T2> {
    type Output=<Self as Map<T,T2>>::Output;
    type Error=E;
    fn is_divable_by(&self,rhs:&F) -> Result<(),E> {
        self.iter()
            .map(|t|t.is_divable_by(rhs))
            .collect::<Result<Vec<_>,E>>()
            .map(|_|())
    }
    
    fn try_div(self, rhs:F) -> Result<Self::Output,E> {
        self.is_divable_by(&rhs)?;
        Ok(self.map(|t|t.try_div(rhs.clone()).unwrap()))
    }
}

impl<T : ComplexField,
     D : Dim,
     S : Storage<T, D, D>> crate::TryInv for SquareMatrix<T,D,S> where DefaultAllocator : Allocator<D,D> {
    type Output=OMatrix<T,D,D>;
    type Error=InvError;
    fn is_invertible(&self) -> Result<(),          InvError> {
        let m=
            DMatrix::from_fn(self.nrows(),self.ncols(),|i,j|self[(i,j)].clone());
        m.try_inv()
         .map(|_|())
    }

    fn try_inv(self) -> Result<Self::Output,InvError> {
        self.try_inverse()
            .ok_or(InvError::NotInvertible(NotInvertibleError::Other))
    }
}