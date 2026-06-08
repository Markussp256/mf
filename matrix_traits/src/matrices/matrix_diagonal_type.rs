
use std::ops::Mul;
use num_traits::{Zero,One};

use algebra_traits::{Det, DivError, FloatOpError, MulError};
use container_traits::{index_iterator::ContainerIndexIterator, *};
use container_traits::LinearContainerConstructError as LCCE;
use utils::iter::{InterLeave, IntoExactSizeIterator, next_chunk_dyn};
use crate::*;
type U2=(usize,usize);

#[derive(Clone,
         Debug,
         algebra_derive::ScalarContainer,
)]
pub struct DiagonalMatrixGeneric<Diag:ItemT> {
   diag:Diag, // diagonal elements written in a Diag vector
   o_zero:Option<Diag::T>, // none if dim <= 1
}


impl<Diag : ItemT> DiagonalMatrixGeneric<Diag> {

   pub fn new(diag:impl Into<Diag>, o_zero:Option<Diag::T>) -> Self {
      Self{diag:diag.into(),o_zero}
   }

   pub fn new_with_zero(diag:impl Into<Diag>) -> Self where Diag::T : Zero {
      Self::new(diag,Some(Diag::T::zero()))
   }


   pub fn n(&self) -> usize where Diag : Len {
      self.diag
          .len()
   }

   pub fn zero(&self) -> &Diag::T {
      self.o_zero
          .as_ref()
          .expect("no zero value available, this function is not supposed to be called when n <= 1 because then there are no off diagonal entries")
   }

   pub fn diagonal<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a Diag::T>
      where Diag    : Iter<Diag::T> {
      self.diag
          .iter()
   }

   pub fn into_diagonal(self) -> impl ExactSizeIterator<Item=Diag::T>
   where Diag : IntoIter<Diag::T> {
      self.diag
          .into_iterator()
   }

   pub fn map_diagonal<Diag2 : ItemT>(self, f:impl Fn(Diag::T) -> Diag2::T, o_zero:Option<Diag2::T>) -> DiagonalMatrixGeneric<Diag2>
   where Diag : Map<Diag::T,Diag2::T,Output=Diag2>, Diag2::T : Zero {
      DiagonalMatrixGeneric::new(self.diag.map(f), o_zero)
   }
}

impl<F,Diag:ItemT<T=F>+Zeros<usize,F>> Zeros<usize,F> for DiagonalMatrixGeneric<Diag> {
    fn zeros(size:usize) -> Self where F:Zero {
        DiagonalMatrixGeneric::<Diag>::new_with_zero(Diag::zeros(size))
    }
}

impl<T:Clone+Zero,Diag:ItemT<T=T>+FromElement<usize,T>> FromElement<usize,T> for DiagonalMatrixGeneric<Diag> {
   fn from_element(size:usize,t:T) -> Self {
       Self::new_with_zero(Diag::from_element(size, t))
   }
}


macro_rules! impl_try {
   ($tr:ident, $fn:ident) => {
      paste::paste!(
      impl<Diag:LinearContainerTryConstruct<T=F>,F:Zero+std::ops::$tr<Output=F>> algebra_traits::[<Try $tr>] for DiagonalMatrixGeneric<Diag> {
         type Output=Self;
         type Error=[<$tr Error>];
         fn [<is_ $fn able_by>](&self, rhs:&Self) -> Result<(), [<$tr Error>]> {
            container_traits::LenNotEqualToRequiredLenError::try_new(self.n(),rhs.n())
               .map_err(|lnee|{
                  let float_error:FloatOpError=lnee.into();
                  <FloatOpError as Into<[<$tr Error>]>>::into(float_error)
               })
         }
         fn [<try_ $fn>](self,rhs:Self) -> Result<Self::Output, [<$tr Error>]> {
            self.[<is_ $fn able_by>](&rhs)?;
            Ok(Self::new_with_zero(
                  Diag::try_from_vec(
                  self.diag
                      .into_iterator()
                      .zip(rhs.diag.into_iterator())
                      .map(|(lhs,rhs)|<F as std::ops::$tr>::$fn(lhs,rhs))
                      .collect()).unwrap()))
         }
      });
   };
}
impl_try!(Mul,mul);
impl_try!(Div,div);

impl<Diag:ItemT<T=F>,F:Zero> Transpose for DiagonalMatrixGeneric<Diag> {
    type Output=Self;

    fn transpose(&self) -> Self::Output where Self : Clone {
        self.clone()
    }

    fn into_transpose(self) -> Self::Output {
        self
    }
}

impl<Diag:ItemT<T=F>+Len+Get<usize,F>,F:Zero> Get<U2,F> for DiagonalMatrixGeneric<Diag> {
   fn get(&self,(i,j):U2) -> Result<&F,IndexOutOfBoundsError<U2>> {
      IndexOutOfBoundsError::try_new(&self.size(),&(i,j))?;
      Ok(if i == j {
         self.diag
            .get(i)
            .unwrap()
      } else {
         self.zero()
      })
   }
}

impl<Diag:Iter<F>+Len+ItemT<T=F>,F:Zero> Iter<F> for DiagonalMatrixGeneric<Diag> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
         let sep=std::iter::repeat_with(||self.zero()).take(self.n()+1);
         self.diag.iter().inter_leave(sep)
    }
}

impl<Diag:Iter<F>+Len+ItemT<T=F>,F:Zero> IterIndexed<U2,F> for DiagonalMatrixGeneric<Diag> {
   fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a F)> where F:'a {
      let cii=ContainerIndexIterator::from_size((self.n(),self.n()));
      let numel=cii.numel();
      cii.zip(self.iter())
         .into_exact_size_iter(numel)
   }
}

impl<Diag:ItemT<T=F>,F:Zero> ItemT for DiagonalMatrixGeneric<Diag> {
   type T=F;
}

impl<Diag:IntoIter<F>+Len+ItemT<T=F>,F:Zero> IntoIter<F> for DiagonalMatrixGeneric<Diag> {
   fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
      let sep=utils::iter::RepeaterN::new(||F::zero(), self.n()+1);// RepeatN::new(F::zero(), self.n()+1);
      self.diag.into_iterator()
          .inter_leave(sep)
   }
}

impl<Diag:IntoIter<F>+Len+ItemT<T=F>,F:Zero> IntoIterIndexed<U2,F> for DiagonalMatrixGeneric<Diag> {
   fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(U2,F)> {
      let n=self.n();
      ContainerIndexIterator::new_exact_size((n,n))
         .zip(self.into_iterator())
   }
}

impl<Diag:TryIntoElement<usize,F>+Len+ItemT<T=F>,F:Zero> TryIntoElement<U2,F> for DiagonalMatrixGeneric<Diag> {
   fn try_into_element(self,index:U2) -> Result<F,IndexOutOfBoundsError<U2>> {
      IndexOutOfBoundsError::try_new(&self.size(),&index)?;
      if index.0 == index.1 {
         Ok(self.diag
            .try_into_element(index.0).unwrap())
      } else {
         Ok(F::zero())
      }
   }
}

impl<Diag:Len+ItemT<T=F>,F:Zero> Size<U2> for DiagonalMatrixGeneric<Diag> {
   fn size(&self) -> U2 {
       let n=self.n();
       (n,n)
   }
}

impl<Diag:IsEmpty+ItemT<T=F>, F:Zero> IsEmpty for DiagonalMatrixGeneric<Diag> {
   fn is_empty(&self) -> bool {
       self.diag
           .is_empty()
   }
}

impl<Diag:OCTSize<usize>+ItemT<T=F>,F:Zero> OCTSize<U2> for DiagonalMatrixGeneric<Diag> {
   const OCTSIZE:Option<U2>=match Diag::OCTSIZE {
      Some(n) => Some((n,n)),
      None => None
   };
}


impl<Diag:Len+ItemT<T=F>,F:Zero> NumberOfDegreesOfFreedom<F> for DiagonalMatrixGeneric<Diag> {
   fn ndofs(&self) -> usize {
       self.n()
   }
}

impl<T : PartialEq, Diag:ItemT<T=T>+Len+AnyFromIterator<T,LCCE>> AnyFromIterator<T,MatrixConstructError> for DiagonalMatrixGeneric<Diag> {
   fn any_take_away<I:    Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,MatrixConstructError> {
      let (v,n,oref)=match oref {
         Some(r) => {
            let n=r.n();
            (next_chunk_dyn(iter, n*n)
               .map_err(|e|LenNotEqualToRequiredLenError::new(n*n,e.len()))?,
            n,
            Some(&r.diag))
         },
         None => {
            let vs: Vec<T>=iter.collect();
            let n = vs.len().isqrt();
            LenNotEqualToRequiredLenError::try_new(n*n,vs.len())?;
            (vs,n,None)
         }
      };
      let mut diag=Vec::new();
      let mut o_zero:Option<T>=None;
      for (i,vi) in v.into_iter().enumerate() {
         if i % n+1 == 0 {
            diag.push(vi);
         } else {
            match &o_zero {
               None => o_zero=Some(vi),
               Some(zero) => {
                  if &vi != zero {
                     return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
                  }
               }
            }
         }
      }
      Diag::any_from_iter(oref, diag.into_iter())
         .map(|diag|Self::new(diag,o_zero))
         .map_err(|e|e.into())
   }
}

impl<T:PartialEq, Diag:ItemT<T=T>+Len+AnyFromIterator<T,LCCE>> TryFromVec<T,MatrixConstructError> for DiagonalMatrixGeneric<Diag> {
   fn try_from_vec(v:Vec<T>) -> Result<Self,MatrixConstructError> {
       <Self as AnyFromIterator<T,MatrixConstructError>>::any_from_iter(None,v.into_iter())
   }
}


impl<F    : Clone+Zero+'static,
     Diag : LinearContainerView<T=F>> MatrixView for DiagonalMatrixGeneric<Diag> {
   type RowView<'a>=SparseRowView<'a,F> where Self : 'a;
   type ColView<'a>=SparseColView<'a,F> where Self : 'a;

   fn nrows(&self) -> usize {
      self.n()
   }
   fn ncols(&self) -> usize {
      self.n()
   }

   fn try_row_view<'a>(&'a self, i:usize) -> Result<Self::RowView<'a>,IndexOutOfBoundsError<usize>> {
      self.diag
          .get(i)
          .map(|t|{
               let mut res=SparseRowView::new(self.zero(),self.ncols());
               let _=res.insert(i,t);
               res
          })
   }

   fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,IndexOutOfBoundsError<usize>> {
      self.diag
          .get(j)
          .map(|t|{
               let mut res=SparseColView::new(self.zero(),self.ncols());
               let _=res.insert(j,t);
               res
          })
   }

   fn diagonal(&self) -> impl ExactSizeIterator<Item=&F> {
      self.diag
         .iter()
   }
}

impl<F    : Clone+Zero+PartialEq+'static,
     Diag : LinearContainer<T=F>> Matrix for DiagonalMatrixGeneric<Diag> {
   type Row=SparseRow<F>;
   type Col=SparseCol<F>;

   fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
      let n=self.n();
      self.diag
          .into_iterator()
          .enumerate()
          .map(move |(i,f)|Self::Row::try_put_at(i, n, f).unwrap())
   }

   fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
      let n=self.n();
      self.diag
          .into_iterator()
          .enumerate()
          .map(move |(i,f)|Self::Col::try_put_at(i, n, f).unwrap()) 
   }

   
   fn into_diagonal(self) -> impl ExactSizeIterator<Item=F> {
      self.diag
         .into_iterator()
   }
}


impl<Diag:ItemT> AsBaseMatrix for DiagonalMatrixGeneric<Diag> where Self:Matrix {
    type Output = Self;
    fn base_matrix(&self) -> &Self {
        self
    }
}

impl<Diag:ItemT> IntoBaseMatrix for DiagonalMatrixGeneric<Diag> where Self:Matrix {
    type Output = Self;
    fn into_base_matrix(self) -> Self {
        self
    }
}

impl<Diag:ItemT<T=F>,F:Zero> MatrixViewNotTall for DiagonalMatrixGeneric<Diag> where Self : Matrix<T=F,Row=Diag> {}
impl<Diag:ItemT<T=F>,F:Zero> MatrixViewNotWide for DiagonalMatrixGeneric<Diag> where Self : Matrix<T=F,Row=Diag> {}
impl<Diag:ItemT<T=F>,F:Zero> MatrixViewSquare  for DiagonalMatrixGeneric<Diag> where Self : Matrix<T=F,Row=Diag> {}

impl<T:Zero,const N:usize, Diag:ItemT<T=T>+LinearContainerStatic<N>> MatrixViewFixedNumberOfCols<N> for DiagonalMatrixGeneric<Diag> where Self : MatrixDiagonal<T=T> {}
impl<T:Zero,const N:usize, Diag:ItemT<T=T>+LinearContainerStatic<N>> MatrixViewFixedNumberOfRows<N> for DiagonalMatrixGeneric<Diag> where Self : MatrixDiagonal<T=T> {}

impl<Diag:ItemT<T=F>+TryPutAt<usize,F>,F:Zero> TryAccept<U2,F,MatrixConstructError> for DiagonalMatrixGeneric<Diag> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
         MatrixSquareError::try_new(nrows,ncols)?;
         for i in 0..nrows {
            for j in 0..ncols {
               if i != j && !f((i,j)).is_zero() {
                  return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
               }
            }
         }
         Ok(())
    }
}

impl<F : Zero, FOut : Zero,
     Row    : ItemT<T=F   >+Len+AnyFromIterator<F,LinearContainerConstructError>+TryMap<F,FOut,LinearContainerConstructError,Output=RowOut>,
     RowOut : ItemT<T=FOut>+AnyFromIterator<FOut,LinearContainerConstructError>> TryMap<F,FOut,MatrixConstructError> for DiagonalMatrixGeneric<Row> {
    type Output=DiagonalMatrixGeneric<RowOut>;

    fn try_map(self, f:impl Fn(F) -> FOut) -> Result<Self::Output,MatrixConstructError> {
        if self.n() > 1 && !f(F::zero()).is_zero() {
            Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
        } else {
            self.diag
                .try_map(f)
                .map(|cout|DiagonalMatrixGeneric::new_with_zero(cout))
                .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
        }
    }
}


impl<Diag : ItemT<T=F>+TryFromFn<usize,F,ContainerConstructError<usize>>, F : Zero> TryFromFn<U2,F,MatrixConstructError> for DiagonalMatrixGeneric<Diag> {
   fn try_from_fn(size:U2,f:impl Fn(U2) -> F) -> Result<Self,MatrixConstructError> {
      for i in 0..size.0 {
         for j in 0..size.1 {
            if i != j && !f((i,j)).is_zero() {
               return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
            }
         }
      }
      Diag::try_from_fn(size.0, |i|f((i,i)))
         .map(|diag|DiagonalMatrixGeneric::new_with_zero(diag))
         .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
   }
}

impl<Row: TryClosedMap<F,LinearContainerConstructError>
         +AnyFromIterator<F,LinearContainerConstructError>
         +ItemT<T=F>,
     F:Zero> MatrixTryConstruct for DiagonalMatrixGeneric<Row>
   where Self : MatrixDiagonal<T=F>+MatrixTryConstruct {
   fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError> {
      let rows:Vec<Self::Row>=rows.collect();
      Self::try_accept_vec_of_rows(rows.iter())?;
      let diag=Row::any_from_iter(
         None,
         rows.into_iter()
                   .enumerate()
                   .map(|(i,row)|row.try_into_element(i).unwrap().into())).unwrap();
       Ok(Self::new_with_zero(diag))
   }
}

impl<Diag:ItemT<T=F>,F:Zero> MatrixSquareTryConstruct for DiagonalMatrixGeneric<Diag> where Self : MatrixTryConstruct<T=F,Row=Diag> {}


impl<Diag:ItemT<T=F>,F:Zero> MatrixDiagonal for DiagonalMatrixGeneric<Diag> where Self : MatrixTryConstruct<T=F,Row=Diag> {}

impl<Diag:ItemT<T=F>+TryFromVec<F,LinearContainerConstructError>,F:Zero> TryFromVec<F,LinearContainerConstructError> for DiagonalMatrixGeneric<Diag> {
   fn try_from_vec(vs:Vec<F>) -> Result<Self,LinearContainerConstructError> {
      Diag::try_from_vec(vs)
         .map(|row|Self::new_with_zero(row))
   }
}

impl<Diag:ItemT<T=F>+IntoIter<F>,F:Zero+One> Det for DiagonalMatrixGeneric<Diag> {
   type Output=F;
   fn det(self) -> F  {
      self.diag
          .into_iterator()
          .into_product()
   }
}

impl<F   : Zero+Mul<F2,Output=F3>,
     F2  : Clone,
     F3,
     Diag  : ItemT<T=F>+IntoIter<F>+VectorVectorProduct<Vec2,Output=F3>,
     Vec2 : ColVector<T=F2>+Rebind<LCCE,With<F3>=Vec3>,
     Vec3 : ColVectorTryConstruct<T=F3>> MatrixVectorProduct<Vec2> for DiagonalMatrixGeneric<Diag>
     where Self : MatrixTryConstruct<T=F,Row=Diag> {
   type Output=Vec3;
   fn into_matrix_vector_product(self, rhs:&Vec2) -> Vec3 {
       Vec3::any_from_iter(None,
         self.diag.into_iterator()
            .zip(rhs.iter().cloned())
            .map(|(dii,ri)|dii*ri)).unwrap()
   }
}

// macro_rules! diag_times_vector {
//    ($name:ident) => {
//          impl<F:Zero+Mul<Output=F>, const N:usize> MatrixVectorProduct<$name<F,N>> for DiagonalMatrix<F,N> {
//             type Output=$name<F,N>;
//             fn matrix_vector_product(self, rhs:$name<F,N>) ->$name<F,N> {
//                  <$name<F,N> as TryFromIterator<F,ItemTConstructError>>::try_from_iter(
//                      self.diag
//                          .into_iterator()
//                          .zip(rhs.into_iterator())
//                          .map(|(dii,vi)|dii*vi)).ok().unwrap()
//             }
//          }
//    };
// }
// diag_times_vector!(Vector);
// diag_times_vector!(MatrixCol);


impl<F:Zero+Mul<F2,Output=F3>,F2 : Clone,F3,
     Diag  : ItemT<T=F>+IntoIter<F>+TryVectorVectorProduct<Col2,Output=F3>,
     Col2 : ColVector<T=F2>+Rebind<LCCE,With<F3>=Col3>,
     Col3 : ColVectorTryConstruct<T=F3>> TryMatrixVectorProduct<Col2> for DiagonalMatrixGeneric<Diag>
     where Self : MatrixTryConstruct<T=F,Row=Diag> {
   type Output=Col3;
   fn try_into_matrix_vector_product(self, rhs:&Col2) -> Result<Col3,VectorConstructError> {
      MatrixCanNotBeMultipliedWithVectorError::try_new(self.n(), rhs.len())?;
      Col3::any_from_iter(None,
         self.diag.into_iterator()
            .zip(rhs.iter().cloned())
            .map(|(dii,ri)|dii*ri))
         .map_err(|e|e.into())
   }
}

impl<F   : 'static+Clone+Zero+Mul<F2,Output=F3>,
     F2  : Clone,
     F3,
     Row : RowVectorView<T=F>+ItemT<T=F>,
     M   : Clone+Matrix<T=F2,Row=MRow>+Rebind<MatrixConstructError,With<F3>=Out>,
     MRow: RowVector<T=F2>+std::ops::Mul<F,Output=Out::Row>,
     Out : MatrixTryConstruct<T=F3>> TryMatrixMatrixProduct<M> for DiagonalMatrixGeneric<Row> {
         type Output=Out;
         fn try_matrix_matrix_product(&self, rhs:&M) -> Result<Out,MatrixConstructError> {
            MatricesCanNotBeMultipliedError::try_new(&self.size(),&rhs.size())?;
            Out::try_from_rows(
               rhs.clone()
                  .into_rows()
                  .zip(self.diag.iter().cloned())
                  .map(|(ri,dii)|ri*dii)
            ).map_err(|e|e.into())
         }
}



// add, sub, mul, div with diagonalmatrix
#[macro_export]
macro_rules! impl_op_diag_stat {
    ($Diag_t:ident, $m_t:ident $(,$ns:ident)?) => {

        impl<F  : 'static+std::ops::AddAssign<F2>$(+$ns)?,
             F2 : 'static+num_traits::Zero$(+$ns)?,
             const M:usize> algebra_traits::TryAdd<DiagonalMatrixGeneric<$Diag_t<F2,M>>> for $m_t<F,M,M> {
                  type Output=$m_t<F,M,M>;
                  type Error=algebra_traits::AddError;
                  fn is_addable_by(&self,_:&DiagonalMatrixGeneric<$Diag_t<F2,M>>) -> Result<(),algebra_traits::AddError> {
                     Ok(())
                  }

                  fn try_add(self, rhs:DiagonalMatrixGeneric<$Diag_t<F2,M>>) -> Result<$m_t<F,M,M>,algebra_traits::AddError> {
                     let res=Self::Output::try_from_rows(
                        <$m_t<F,M,M> as $crate::Matrix>::into_rows(self)
                           .zip(rhs.into_diagonal())
                           .enumerate()
                           .map(|(i,(ri,dii))|<<Self as $crate::Matrix>::Row as container_traits::TryMapI::<usize,F>>::try_map_i(ri,i,|rii|*rii += dii).unwrap())
                     ).ok().unwrap();
                     Ok(res)
                  }
             }

        impl<F  : 'static+std::ops::SubAssign<F2>$(+$ns)?,
             F2 : 'static+num_traits::Zero$(+$ns)?,
             const M:usize> algebra_traits::TrySub<DiagonalMatrixGeneric<$Diag_t<F2,M>>> for $m_t<F,M,M> {
                  type Output=$m_t<F,M,M>;
                  type Error=algebra_traits::SubError;
                  fn is_subable_by(&self,_:&DiagonalMatrixGeneric<$Diag_t<F2,M>>) -> Result<(),algebra_traits::SubError> {
                     Ok(())
                  }

                  fn try_sub(self, rhs:DiagonalMatrixGeneric<$Diag_t<F2,M>>) -> Result<$m_t<F,M,M>,algebra_traits::SubError> {
                     let res=Self::Output::try_from_rows(
                        <$m_t<F,M,M> as $crate::Matrix>::into_rows(self)
                           .zip(rhs.into_diagonal())
                           .enumerate()
                           .map(|(i,(ri,dii))|<<Self as $crate::Matrix>::Row as container_traits::TryMapI::<usize,F>>::try_map_i(ri,i,|rii|*rii -= dii).unwrap())
                     ).ok().unwrap();
                     Ok(res)
                  }
             }

             impl<F  : 'static+Clone+std::ops::Mul<F2,Output=F3>$(+$ns)?,
             F2 : 'static+Clone+num_traits::Zero+Clone$(+$ns)?,
             F3 : 'static$(+$ns)?,
             const M:usize,
             const N:usize> $crate::TryMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2,N>>> for $m_t<F,M,N> {
                  type Output=$m_t<F3,M,N>;
                  fn try_matrix_matrix_product(&self, rhs:&DiagonalMatrixGeneric<$Diag_t<F2,N>>) -> Result<$m_t<F3,M,N>,$crate::MatrixConstructError> {
                     Self::Output::try_from_cols(
                        self.clone()
                            .into_cols()
                            .zip(rhs.diagonal().cloned())
                            .map(|(ci,dii)|<<Self as $crate::Matrix>::Col as container_traits::Map::<F,F3>>::map(ci,|cij|cij*dii.clone()))
                     ).map_err(|e|e.into())
                  }
             }

             impl<F  : 'static+std::ops::Mul<F2,Output=F3>$(+$ns)?,
             F2 : 'static+num_traits::Zero+Clone$(+$ns)?,
             F3 : 'static$(+$ns)?,
             const M:usize,
             const N:usize> MatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2,N>>> for $m_t<F,M,N> where Self : $crate::MatrixView<T=F> {
                  type Output=$m_t<F3,M,N>;
                  fn matrix_matrix_product(&self, rhs:&DiagonalMatrixGeneric<$Diag_t<F2,N>>) -> $m_t<F3,M,N> {
                     <Self as $crate::TryMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2,N>>>>::
                        try_matrix_matrix_product(self,rhs)
                         .unwrap()
                  }
             }
    };
}

#[macro_export]
macro_rules! impl_op_diag_dyn {
    ($Diag_t:ident, $m_t:ident $(,$ns:ident)?) => {


        impl<F  : 'static+std::ops::AddAssign<F2>$(+$ns)?,
             F2 : 'static+num_traits::Zero$(+$ns)?> algebra_traits::TryAdd<DiagonalMatrixGeneric<$Diag_t<F2>>> for $m_t<F> {
                  type Output=$m_t<F>;
                  type Error=algebra_traits::AddError;
                  fn is_addable_by(&self,rhs:&DiagonalMatrixGeneric<$Diag_t<F2>>) -> Result<(),algebra_traits::AddError> {
                     if self.matrix_dimensions() == rhs.matrix_dimensions() {
                        Ok(())
                     } else {
                        Err(algebra_traits::AddError::NotAvailableForProvidedInstances)
                     }
                  }

                  fn try_add(self, rhs:DiagonalMatrixGeneric<$Diag_t<F2>>) -> Result<$m_t<F>,algebra_traits::AddError> {
                     self.is_addable_by(&rhs)?;
                     let res=Self::Output::try_from_rows(
                        <$m_t<F> as $crate::Matrix>::into_rows(self)
                           .zip(rhs.into_diagonal())
                           .enumerate()
                           .map(|(i,(ri,dii))|<<Self as $crate::Matrix>::Row as container_traits::TryMapI::<usize,F>>::try_map_i(ri,i,|rii|*rii += dii).unwrap())
                     ).ok().unwrap();
                     Ok(res)
                  }
             }

        impl<F  : 'static+std::ops::SubAssign<F2>$(+$ns)?,
             F2 : 'static+num_traits::Zero$(+$ns)?> algebra_traits::TrySub<DiagonalMatrixGeneric<$Diag_t<F2>>> for $m_t<F> {
                  type Output=$m_t<F>;
                  type Error=algebra_traits::SubError;
                  fn is_subable_by(&self,rhs:&DiagonalMatrixGeneric<$Diag_t<F2>>) -> Result<(),algebra_traits::SubError> {
                     if self.matrix_dimensions() == rhs.matrix_dimensions() {
                        Ok(())
                     } else {
                        Err(algebra_traits::SubError::NotAvailableForProvidedInstances)
                     }
                  }

                  fn try_sub(self, rhs:DiagonalMatrixGeneric<$Diag_t<F2>>) -> Result<$m_t<F>,algebra_traits::SubError> {
                     self.is_subable_by(&rhs)?;
                     let res=Self::Output::try_from_rows(
                        <$m_t<F> as $crate::Matrix>::into_rows(self)
                           .zip(rhs.into_diagonal())
                           .enumerate()
                           .map(|(i,(ri,dii))|<<Self as $crate::Matrix>::Row as container_traits::TryMapI::<usize,F>>::try_map_i(ri,i,|rii|*rii -= dii).unwrap())
                     ).ok().unwrap();
                     Ok(res)
                  }
             }

        impl<F  : 'static+Clone+Zero+std::ops::Mul<F2,Output=F3>$(+$ns)?,
             F2 : 'static+Clone+Zero$(+$ns)?,
             F3 : 'static+Zero$(+$ns)?> TryMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2>>> for $m_t<F> {
                  type Output=$m_t<F3>;
                  fn try_matrix_matrix_product(&self, rhs:&DiagonalMatrixGeneric<$Diag_t<F2>>) -> Result<$m_t<F3>,MatrixConstructError> {
                     $crate::MatricesCanNotBeMultipliedError::try_new(
                        &<Self as container_traits::Size<(usize,usize)>>::size(self),
                        &(rhs.n(),rhs.n()))?;
                     Self::Output::try_from_cols(
                        self.cols()
                            .zip(rhs.diagonal().cloned())
                            .map(|(ci,dii)|<<Self as Matrix>::Col as container_traits::Map<F,F3>>::map(ci,|cij|cij*dii.clone()))
                     ).map_err(|e|e.into())
                  }
             }
    };
}

impl<F     : 'static+Zero+Clone+Mul<F2,Output=F3>,
     F2    : 'static+Zero+Clone,
     F3    : Zero,
     Diag  : ItemT<T=F>+LinearContainerSized<T=F>+Iter<F>
            +Rebind<LCCE,With<F2>=Diag2>
            +Rebind<LCCE,With<F3>=Diag3>,
     Diag2 : ItemT<T=F2>+Iter<F2>,
     Diag3 : ItemT<T=F3>
            +AnyFromIterator<F3,LinearContainerConstructError>> MatrixMatrixProduct<DiagonalMatrixGeneric<Diag2>> for DiagonalMatrixGeneric<Diag>
   where DiagonalMatrixGeneric<Diag>  : MatrixView<T=F>,
         DiagonalMatrixGeneric<Diag2> : MatrixView<T=F2>,
         DiagonalMatrixGeneric<Diag3> : Matrix<T=F3> {

         type Output=DiagonalMatrixGeneric<Diag3>;

         fn matrix_matrix_product(&self, rhs:&DiagonalMatrixGeneric<Diag2>) -> DiagonalMatrixGeneric<Diag3> {
             DiagonalMatrixGeneric::<Diag3>::new_with_zero(Diag3::any_from_iter(None,
                 self.diagonal().cloned()
                     .zip(rhs.diagonal().cloned())
                     .map(|(lhs,rhs)|lhs*rhs)).ok().unwrap())
         }
}



#[macro_export]
macro_rules! impl_mul_diag {
    ($name:ident<$f:ident, $m:ident, $n:ident>) => {
      crate::impl_mul_diag!($name<$f,$m,$n>, $f, $m, $n, $m, $n);
    };
    ($name:ident<$f:ident, $m:ident>) => {
      crate::impl_mul_diag!($name<$f,$m>, $f, $m, $m, $m);
    };

    ($t:ty, $f:ident, $m:ident, $n:ident $(,$ns:ident)*) => {
         
         impl<$f:'static+Clone+Zero+Mul<Output=$f>
                  +algebra_traits::ScalarMul<$f> $(, const $ns:usize)*>
               $crate::MatrixMatrixProduct<$crate::DiagonalMatrix<$f,N>> for $t {
            type Output=$t;
            fn matrix_matrix_product(self, rhs:$crate::DiagonalMatrix<$f,N>) -> Self::Output {
               <Self::Output as $crate::MatrixTryConstruct>::try_from_cols(
                <Self as $crate::Matrix>::into_cols(self)
                    .zip(<_ as $crate::Matrix>::into_diagonal(rhs))
                    .map(|(Diag,dii)|<_ as algebra_traits::ScalarMul<$f>>::scalar_mul(Diag,&dii))).unwrap()
            }
         }
    };
}
