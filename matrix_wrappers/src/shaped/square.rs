use std::ops::Sub;
use algebra_traits::{TryAdd, TrySub};
use container_traits::{GetMut, Size, SizesNotEqualError, TryAccept};
use num_traits::Zero;
use matrix_traits::{matrix_shapes::{MatrixNotTall, MatrixNotWide}, MatrixSquareTryConstruct, AsBaseMatrix, AsBaseSquareMatrix, IntoBaseMatrix, IntoBaseSquareMatrix, Matrix, MatrixConstructError, MatrixDiagonal, MatrixMut, MatrixSquare, MatrixTryConstruct};

// we can not use matrix_derive::Inherit because we want to implement IntoMatrix, IntoSquareMatrix

type U2=(usize,usize);

#[derive(Clone,
         Debug,
         PartialEq,
         algebra_derive::ScalarContainer,
         algebra_derive::TryDiv,
         algebra_derive::One,
         container_derive::ContainerMut,
         derive_more::Index,
         derive_more::IndexMut,
         matrix_derive::Display,
         matrix_derive::Empty,
         matrix_derive::Identity,
         matrix_derive::Matrix,
         matrix_derive::MatrixTryConstruct,
         matrix_derive::MatrixVectorProduct,
         matrix_derive::MatrixMatrixProduct,
         matrix_derive::Transpose)]
pub struct Square<M:Matrix>(M);

impl<M:Matrix> AsBaseMatrix for Square<M> {
   type Output=M;
   fn base_matrix(&self) -> &M {
      &self.0
   }
}

impl<M:Matrix> IntoBaseMatrix for Square<M> {
   type Output=M;
   fn into_base_matrix(self) -> M {
      self.0
   }
}

impl<M:Matrix> MatrixNotWide for Square<M> {}
impl<M:Matrix> MatrixNotTall for Square<M> {}
impl<M:Matrix> MatrixSquare  for Square<M> {} 

impl<M:MatrixTryConstruct> MatrixSquareTryConstruct for Square<M> {}


impl<M:Matrix> AsBaseSquareMatrix for Square<M> {
   type Output = Self;
   fn base_square_matrix(&self) -> &Self::Output {
       &self
   }
}

impl<M:Matrix> IntoBaseSquareMatrix for Square<M> {
   type Output = Self;
   fn into_base_square_matrix(self) -> Self::Output {
       self
   }
}

impl<M:Matrix> TryAccept<U2,M::T,MatrixConstructError> for Square<M> {
   fn try_accept<'a>((nrows,ncols):U2,_:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
      if nrows == ncols {
          Ok(())
      } else {
          Err(MatrixConstructError::DimensionMismatch)
      }
   }
}

impl<F:Clone+Zero,M:MatrixMut<T=F>, D:MatrixDiagonal<T=F>> TryAdd<D> for Square<M> {
   type Output=Self;
   type Error=SizesNotEqualError<U2>;
   fn is_addable_by(&self,rhs:&D) -> Result<(),SizesNotEqualError<U2>> {
      SizesNotEqualError::try_new(self.size(),rhs.size())
   }

   fn try_add(self, rhs:D) -> Result<Self::Output,SizesNotEqualError<U2>> {
      self.is_addable_by(&rhs)?;
      let mut s=self;
      for (i, di) in rhs.into_diagonal().enumerate() {
         let sii=s.get_mut((i,i)).unwrap();
         *sii=sii.clone()+di;
      }
      Ok(s)
   }
}

impl<F:Clone+Zero+Sub<Output=F>,M:MatrixMut<T=F>, D:MatrixDiagonal<T=F>> TrySub<D> for Square<M> {
   type Output=Self;
   type Error=SizesNotEqualError<U2>;
   fn is_subable_by(&self,rhs:&D) -> Result<(),SizesNotEqualError<U2>> {
      SizesNotEqualError::try_new(self.size(),rhs.size())
   }

   fn try_sub(self, rhs:D) -> Result<Self::Output,SizesNotEqualError<U2>> {
      self.is_subable_by(&rhs)?;
      let mut s=self;
      for (i, di) in rhs.into_diagonal().enumerate() {
         let sii=s.get_mut((i,i)).unwrap();
         *sii=sii.clone()-di;
      }
      Ok(s)
   }
}

// impl<F:'static, M:MatrixTryConstruct<T=F>> TryFrom<MatrixDyn<F>> for Square<M> {
//    type Error=MatrixDyn<F>;
//    fn try_from(m: MatrixDyn<F>) -> Result<Self, Self::Error> {
//       if m.nrows() != m.ncols() {
//          return Err(m);
//       }
//       M::try_from_matrix(m)
//          .map(|m|Self(m))
//    }
// }

impl<D:MatrixSquare> From<D> for Square<D> {
   fn from(m:D) -> Self {
      Self(m)
   }
}