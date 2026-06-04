// we can not implement MatrixTryConstruct for SpecialOrthogonal/SpecialUnitary
// we would need to compute determinant which requires QR which requires matrix_wrappers, hence a circular dependency

use algebra::{Quaternion, quaternion::ProjectiveQuaternion};
use algebra_traits::{Exp, TryLog, Conjugate};
use num_traits::{Zero,One};

use matrix_traits::{*,identity::for_static::Identity};
use algebra_traits::{AdditiveGroup, ClosedTryInv, ComplexNumber, RealNumber, Scalar, TryInv, TryNormalize};
use super::{Homogeneous, Orthogonal, Stiefel, Unitary};
use crate::{SkewSymmetric, SkewSymmetricPart};
use container_traits::{AnyFromIterator, ChangeDim, ContainerConstructError, Get, IntoInner, IntoIter, IntoParameters, Iter, IntoSum, NewUnchecked, TryFromIterator, for_static::{FromFn, TryFromParameters}};
use std::fmt::Debug;

type U2=(usize,usize);

macro_rules! def_orthogonal_or_unitary {
    ($uc:ident, $tr:ident $(, $name:ident)?) => {
        paste::paste!(
            #[derive(Clone, Debug, PartialEq,
              algebra_derive::ClosedConjugate,
              container_derive::Empty,
              container_derive::IntoInner,
              container_derive::Inner,
              container_derive::JustContainer,
              container_derive::NewUnchecked,
              derive_more::AsRef,
              derive_more::Index,
              matrix_derive::Display,
              matrix_derive::Identity,
              matrix_derive::IntoBaseSquareMatrix,
              matrix_derive::IntoBaseMatrix,
              matrix_derive::AsBaseMatrix,
              matrix_derive::AsBaseSquareMatrix,
              matrix_derive::Matrix,
              matrix_derive::MatrixView,
              matrix_derive::AlgebraMatrix,
              matrix_derive::MatrixNormal,
              matrix_derive::MatrixVectorProduct,
              matrix_derive::ClosedMatrixMatrixProduct,
              matrix_derive::MatrixMatrixProductRightTriangular,
              matrix_derive::ClosedTranspose,
              matrix_derive::StaticMatrix,
              matrix_derive::MatrixShape)]
             pub struct [<Special $uc>]<M:MatrixViewSquare>(M) where M::T : $tr;

             impl<M:MatrixViewSquare> [<Special $uc>]<M> where M::T : $tr {
                 pub fn try_new(m:$uc<M>, det:M::T) -> Result<Self,$uc<M>> {
                     if det == M::T::one() {
                         Ok(Self(m.into_inner()))
                     } else {
                         Err(m)
                     }
                 }
             }

            impl<M:MatrixViewSquare+Conjugate<Output=M>+Transpose<Output=M>> algebra_traits::Inv for [<Special $uc>]<M> where M::T : $tr {
                type Output=Self;
                fn inv(self) -> Self {
                    self.into_conjugate_transpose()
                }
            }

            impl<M:MatrixViewSquare+Conjugate<Output=M>+Transpose<Output=M>> algebra_traits::TryInv for [<Special $uc>]<M> where M::T : $tr {
                type Output=Self;
                type Error=();
                fn is_invertible(&self) -> Result<(),()> { Ok(()) }
                fn try_inv(self) -> Result<Self,()> {
                    Ok(self.into_conjugate_transpose())
                }
            }

            impl<F : $tr, M:MatrixSquareTryConstruct<T=F>+TryPopCol> TryPopCol for [<Special $uc>]<M> {
                type Output=Stiefel<<M as TryPopCol>::Output>;
                fn try_pop_col(self) -> Option<(<Self as TryPopCol>::Output,M::Col)> {
                    self.0
                        .try_pop_col()
                        .map(|(ms,col)|(Stiefel::new_unchecked(ms),col))
                }
            }

            //  $(
            //  impl<M:MatrixViewSquare> IntoParameters<M::T> for [<Special $uc>]<M>
            //  where M::T : $tr,
            //        M    : $name,
            //        <M as matrix_traits::IntoBaseSquareMatrix>::Output : MatrixSquareTryConstruct<T=M::T> {
            //      fn into_parameters(self) -> impl ExactSizeIterator<Item=M::T> {
            //         <M as $name>::skew_part(self.0)
            //         .into_parameters()
            //      }
            //  })?
        );
    };
}
def_orthogonal_or_unitary!(Orthogonal, RealNumber   , SkewSymmetricPart);
def_orthogonal_or_unitary!(Unitary,    ComplexNumber, AntiHermitianPart);
def_orthogonal_or_unitary!(Stiefel,    Scalar);

impl<M:MatrixViewSquare> From<Homogeneous<M>> for SpecialOrthogonal<M> where M::T : RealNumber {
    fn from(value: Homogeneous<M>) -> Self {
        Self(value.into_inner())
    }
}



impl<M : MatrixViewFixedSize<3,3,T=F>+MatrixViewSquare+FromFn<U2,F>, F : Clone+RealNumber> From<ProjectiveQuaternion<F>> for SpecialOrthogonal<M> {
    fn from(value: ProjectiveQuaternion<F>) -> Self {
        let (real, imag):(F,[F;3])=value.quaternion().clone().into_real_imag();
        let pow2=|x:&F|x.clone()*x.clone();
        let f=|(i,j):U2|
            if i == j {
                let i1=i+1 % 3;
                let i2=i+2 % 3;
                F::one()-pow2(&imag[i1])-pow2(&imag[i2])
            } else {
                let k=3-i-j;
                let sign=if i-1 == j || i+2 == j { F::one() } else { -F::one() };
                let i:F=imag[i].clone();
                let j:F=imag[j].clone();
                let k:F=imag[k].clone();
                let r:F=real.clone();
                let two=F::one()+F::one();
                two*(i*j+sign*k*r)
            };
        SpecialOrthogonal(M::from_fn(f))
    }
}

// since this would create circular dependency we can not use optimization
// for this impl
impl<
    M33: MatrixViewFixedSize<3,3,T=F>
       + Matrix
       + AlgebraMatrix
       + MatrixViewSquare
       + ChangeDim<Output<4,4> = M44>,

    M44: MatrixViewFixedSize<4,4,T=F>
       + Matrix
       + AlgebraMatrix
       + MatrixViewSquare
       + Clone
       + MatrixViewMut<T=F>
       + Zero,
    F: Clone + RealNumber> Into<ProjectiveQuaternion<F>> for SpecialOrthogonal<M33> {
    fn into(self) -> ProjectiveQuaternion<F> {
        // unsafe get and clone
        let usg=|i,j|self.get((i,j)).unwrap().clone();
        let sm=|i,j|usg(i,j)+usg(j,i);
        let sk=|i,j|usg(i,j)-usg(j,i);

        let d=|i|usg(i,i);
        // create rank 1 matrix with all products (times 4) of the quaternion entries
        let mut m44=M44::zero();
        let sum_d=d(0)+d(1)+d(2);
        let mut set=|i,j,v:F|*m44.get_mut((i,j)).unwrap()=v;
        set(0,0,sum_d.clone()+F::one());
        let c=F::one()-sum_d;
        for i in 0..3 {
            let i1=(i+1) % 3;
            let i2=(i+2) % 3;
            set(i+1, i+1,  d(i)+d(i)+c.clone());
            set(0,   i+1,  sk(i2,i1));
            set(i+1, 0,    sk(i2,i1));
            set(i1+1,i2+1, sm(i1,i2));
            set(i2+1,i1+1, sm(i1,i2));
        }

        // find max on diagonal 
        let mut max_value = &F::zero();
        let mut last_max_index = 0;
    
        for (index, value) in m44.diagonal().enumerate() {
            if value >= max_value {
                max_value = value;
                last_max_index = index;
            }
        }
        let iter0_to_f4=|iter|<[F;4] as AnyFromIterator<F,ContainerConstructError<usize>>>::any_from_iter(Option::<_>::None,iter).unwrap();
        let iter1_to_f4=|iter|<[F;4] as AnyFromIterator<F,ContainerConstructError<usize>>>::any_from_iter(Option::<_>::None,iter).unwrap();
        let mut x=iter0_to_f4(m44.try_col(last_max_index).unwrap().iter().cloned());

        x=x.try_divide_by_norm().unwrap().1;

        // power iteration to improve result
        for _ in 0..5 {
            // matrix multiplication
            let xc=x.clone();
            x=iter1_to_f4(m44.rows().into_iter().map(move |r|r.into_iterator().zip(xc.clone()).map(|(ri,xi)|ri*xi).into_sum()));
            x=x.try_divide_by_norm().unwrap().1;
        }
        let q=Quaternion::try_from_iter(x.into_iterator()).unwrap();
        ProjectiveQuaternion::try_from(q).ok().unwrap()
    }
}

// for parametrization we use cayley transform to map to skew symmetric matrices

trait MConditions<T : RealNumber> : Clone
                    +AlgebraMatrix<T=T>
                    +IntoBaseSquareMatrix<Output=Self>
                    +SkewSymmetricPart
                    +MatrixSquareTryConstruct<T=T>
                    +Identity
                    +AdditiveGroup
                    +ClosedTryInv
                    +MatrixMatrixProduct<Output=Self> {}

impl<T : RealNumber,
     E : Debug,
     M : Clone
        +AlgebraMatrix<T=T>
        +IntoBaseSquareMatrix<Output=M>
        +SkewSymmetricPart
        +MatrixSquareTryConstruct
        +Identity
        +AdditiveGroup
        +ClosedTryInv<Error=E>
        +MatrixMatrixProduct<M,Output=M>> MConditions<T> for M {}

fn skew2special<T:RealNumber, M : MConditions<T>>(skew:SkewSymmetric<M>) -> SpecialOrthogonal<M> where <M as TryInv>::Error : Debug {
    let m=skew.into_inner();
    let mt=(M::identity()+m.clone()).try_inv().ok().unwrap().matrix_matrix_product(&(M::identity()-m));
    let o=Orthogonal::try_new(mt).unwrap();
    SpecialOrthogonal::try_new(o,M::T::one()).ok().unwrap()
}

fn special2skew<T:RealNumber, M : MConditions<T>>(special:SpecialOrthogonal<M>) -> SkewSymmetric<M> where <M as TryInv>::Error : Debug {
    let m=special.into_inner();
    let mt=(M::identity()+m.clone()).try_inv().unwrap().matrix_matrix_product(&(m-M::identity()));
    mt.skew_part()
}

impl<T:RealNumber, M : MConditions<T>> IntoParameters<T> for SpecialOrthogonal<M> where <M as TryInv>::Error : Debug {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=M::T> {
        special2skew(self)
            .into_parameters()
    }
}

impl<F : Clone+RealNumber, M : MConditions<F>> TryFromParameters<F,MatrixConstructError> for SpecialOrthogonal<M> where
      SkewSymmetric<M> : TryFromParameters<F,MatrixConstructError>,
      <M as TryInv>::Error : Debug {
    fn try_take_away<I:    Iterator<Item=F>>(iter:& mut I) -> Result<Self,MatrixConstructError> {
        let m=SkewSymmetric::<M>::try_take_away(iter)?;
        Ok(skew2special(m))
    }
    container_traits::try_from_parameters_impl!(F,MatrixConstructError);
}

impl<M    : AlgebraMatrix+MatrixViewSquare+TryLog<Output=MLog>,
     MLog : MatrixSquareTryConstruct> TryLog for SpecialOrthogonal<M>
     where M::T    : RealNumber,
           MLog::T : RealNumber {
    type Output=SkewSymmetric<MLog>;
    type Error = <M as TryLog>::Error;

    fn is_logable(&self) -> Result<(),Self::Error> {
        self.0
            .is_logable()
    }

    fn try_log(self) -> Result<Self::Output,Self::Error> {
        self.0
            .try_log()
            .map(|m|SkewSymmetric::<MLog>::try_new(m).unwrap())
    }
}


impl<M    : MatrixViewSquare+Exp<Output=MExp>,
     MExp : AlgebraMatrix+MatrixSquareTryConstruct> Exp for SkewSymmetric<M>
     where M::T    : RealNumber,
           MExp::T : RealNumber {
    type Output=SpecialOrthogonal<MExp>;

    fn exp(self) -> Self::Output {
        let o=Orthogonal::<MExp>::try_from_matrix(self.into_inner().exp()).ok().unwrap();
        SpecialOrthogonal::<MExp>::try_new(o,MExp::T::one()).ok().unwrap()
    }
}