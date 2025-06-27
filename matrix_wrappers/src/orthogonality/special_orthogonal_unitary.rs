// we can not implement MatrixTryConstruct for SpecialOrthogonal/SpecialUnitary
// we would need to compute determinant which requires QR which requires matrix_wrappers, hence a circular dependency

use algebra::{quaternion::ProjectiveQuaternion, Quaternion};
use num_traits::{Zero,One};

use matrix_traits::{matrix_shapes::{Matrix33,Matrix44}, AlgebraMatrix, ChangeDim, IntoBaseSquareMatrix, MatrixMut, MatrixConstructError, MatrixMatrixProduct, MatrixSquare, MatrixSquareTryConstruct, MatrixVectorProduct, Transpose};
use matrix_traits::identity::for_static::Identity;
use algebra_traits::{AdditiveGroup, ClosedTryInv, ComplexNumber, Norm, RealNumber, Scalar, TryInv, TryNormalize};
use super::{Homogeneous, Orthogonal, Stiefel, Unitary};
use crate::{SkewSymmetric, SkewSymmetricPart};
use container_traits::{for_static::{FromFn, TryFromParameters}, Get, IntoInner, IntoIter, IntoParameters, TryFromIterator};
use std::fmt::Debug;

type U2=(usize,usize);

macro_rules! def_orthogonal_or_unitary {
    ($uc:ident, $tr:ident $(, $name:ident)?) => {
        paste::paste!(
            #[derive(Clone, Debug, PartialEq,
              algebra_derive::Conjugate,
              container_derive::IntoInner,
              container_derive::JustContainer,
              derive_more::AsRef,
              derive_more::Index,
              matrix_derive::Display,
              matrix_derive::Empty,
              matrix_derive::Identity,
              matrix_derive::IntoBaseSquareMatrix,
              matrix_derive::IntoBaseMatrix,
              matrix_derive::AsBaseMatrix,
              matrix_derive::AsBaseSquareMatrix,
              matrix_derive::Matrix,
              matrix_derive::MatrixVectorProduct,
              matrix_derive::MatrixMatrixProduct,
              matrix_derive::ClosedTranspose,
              matrix_derive::StaticMatrix,
              matrix_derive::MatrixShape)]
             pub struct [<Special $uc>]<M:MatrixSquare>(M) where M::T : $tr;

             impl<M:MatrixSquare> [<Special $uc>]<M> where M::T : $tr {
                 pub fn try_new(m:$uc<M>, det:M::T) -> Result<Self,$uc<M>> {
                     if det == M::T::one() {
                         Ok(Self(m.into_inner()))
                     } else {
                         Err(m)
                     }
                 }
             }

            impl<M:MatrixSquare+Transpose<Output=M>> algebra_traits::Inv for [<Special $uc>]<M> where M::T : $tr {
                type Output=Self;
                fn inv(self) -> Self {
                    self.transpose()
                }
            }

            //  $(
            //  impl<M:MatrixSquare> IntoParameters<M::T> for [<Special $uc>]<M>
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

impl<M:MatrixSquare> From<Homogeneous<M>> for SpecialOrthogonal<M> where M::T : RealNumber {
    fn from(value: Homogeneous<M>) -> Self {
        Self(value.into_inner())
    }
}



impl<M : Matrix33<T=F>+FromFn<U2,F>, F : Clone+RealNumber> From<ProjectiveQuaternion<F>> for SpecialOrthogonal<M> {
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
impl<M   : Matrix33<T=F>+ChangeDim<Output<4,4>=M44>,
     M44 : Matrix44<T=F>+Clone+MatrixMut<T=F>+Zero+MatrixVectorProduct<M44::Col,Output=M44::Col>,
     F : Clone+RealNumber,
     E : Debug> Into<ProjectiveQuaternion<F>> for SpecialOrthogonal<M>
     where M44::Col : TryNormalize<Error=E> + Norm<NormT=F> {
    fn into(self) ->  ProjectiveQuaternion<F> {       
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
        let mut x:M44::Col=
            m44.col(last_max_index).unwrap()
               .try_divide_by_norm().unwrap().1;
        
        // power iteration to improve result
        for _ in 0..5 {
            x=m44.clone()
                 .matrix_vector_product(x)
                 .try_divide_by_norm().unwrap().1;
        }
        let q=Quaternion::try_from_iter(x.into_iterator()).unwrap();
        ProjectiveQuaternion::try_from(q).ok().unwrap()
    }
}

// for parametrization we use cayley transform to map to skew symmetric matrices

trait MConditions : Clone
                    +AlgebraMatrix
                    +IntoBaseSquareMatrix<Output=Self>
                    +SkewSymmetricPart
                    +MatrixSquareTryConstruct
                    +Identity
                    +AdditiveGroup
                    +ClosedTryInv
                    +MatrixMatrixProduct<Self,Output=Self> where Self::T : RealNumber {}

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
        +MatrixMatrixProduct<M,Output=M>> MConditions for M {}

fn skew2special<M : MConditions>(skew:SkewSymmetric<M>) -> SpecialOrthogonal<M> where M::T : RealNumber, <M as TryInv>::Error : Debug {
    let m=skew.into_inner();
    let mt=(M::identity()+m.clone()).try_inv().unwrap().matrix_matrix_product(M::identity()-m);
    let o=Orthogonal::try_new(mt).unwrap();
    SpecialOrthogonal::try_new(o,M::T::one()).ok().unwrap()
}

fn special2skew<M : MConditions>(special:SpecialOrthogonal<M>) -> SkewSymmetric<M> where M::T : RealNumber, <M as TryInv>::Error : Debug {
    let m=special.into_inner();
    let mt=(M::identity()+m.clone()).try_inv().unwrap().matrix_matrix_product(m-M::identity());
    mt.skew_part()
}

impl<M : MConditions> IntoParameters<M::T> for SpecialOrthogonal<M> where M::T : RealNumber, <M as TryInv>::Error : Debug {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=M::T> {
        special2skew(self)
            .into_parameters()
    }
}

impl<M : MConditions> TryFromParameters<M::T,MatrixConstructError> for SpecialOrthogonal<M>
where M::T             : Clone+RealNumber,
      SkewSymmetric<M> : TryFromParameters<M::T,MatrixConstructError>,
      <M as TryInv>::Error : Debug {
    fn try_take_away<I:    Iterator<Item=M::T>>(iter:& mut I) -> Result<Self,MatrixConstructError> {
        let m=SkewSymmetric::<M>::try_take_away(iter)?;
        Ok(skew2special(m))
    }
    container_traits::try_from_parameters_impl!(M::T,MatrixConstructError);
}