
use std::ops::Mul;
use num_traits::{Zero,One};

use algebra_traits::{Det, DivError, FloatOpError, MulError};
use container_traits::{index_iterator::ContainerIndexIterator, *};
use utils::iter::{InterLeave, IntoExactSizeIterator, RepeatN};

use crate::*;

type U2=(usize,usize);

#[derive(Clone,
         Debug,
         algebra_derive::ScalarContainer,
)]
pub struct DiagonalMatrixGeneric<Diag:ItemT> {
   diag:Diag, // diagonal elements written in a Diag vector
   zero:Diag::T
}




impl<Diag:ItemT+ChangeT<T2,Output=Diag2>,
     T2,
     Diag2:ItemT> ChangeT<T2> for DiagonalMatrixGeneric<Diag> {
   type Output = DiagonalMatrixGeneric<Diag2>;
}


impl<Diag : ItemT<T=F>, F:Zero> DiagonalMatrixGeneric<Diag> {
   pub fn new(diag:impl Into<Diag>) -> Self {
      Self{diag:diag.into(),zero:F::zero()}
   }

   pub fn n(&self) -> usize where Diag : Len {
      self.diag
          .len()
   }

   pub fn diagonal<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F>
      where Diag : Iter<F>,
               F : 'static {
      self.diag
          .iter()
   }

   pub fn into_diagonal(self) -> impl ExactSizeIterator<Item=F>
   where Diag : IntoIter<F> {
      self.diag
          .into_iterator()
   }

   pub fn map_diagonal<Diag2 : Zero+ItemT>(self, f:impl Fn(F) -> Diag2::T) -> DiagonalMatrixGeneric<Diag2>
   where Diag : Map<F,Diag2::T,Output=Diag2>, Diag2::T : Zero {
      DiagonalMatrixGeneric::new(self.diag.map(f))
   }
}

impl<F,Diag:ItemT<T=F>+Zeros<usize,F>> Zeros<usize,F> for DiagonalMatrixGeneric<Diag> {
    fn zeros(size:usize) -> Self where F:Zero {
        DiagonalMatrixGeneric::<Diag>::new(Diag::zeros(size))
    }
}

impl<T:Clone+Zero,Diag:ItemT<T=T>+FromElement<usize,T>> FromElement<usize,T> for DiagonalMatrixGeneric<Diag> {
   fn from_element(size:usize,t:T) -> Self {
       Self::new(Diag::from_element(size, t))
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
            Ok(Self::new(
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

impl<Diag:ItemT<T=F>+Clone,F:Clone+Zero> Transpose for DiagonalMatrixGeneric<Diag> {
    type Output=Self;

    fn transpose(&self) -> Self::Output {
        self.clone()
    }
}

impl<Diag:ItemT<T=F>+Clone,F:Zero> IntoTranspose for DiagonalMatrixGeneric<Diag> {
    type Output=Self;

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
         &self.zero
      })
   }
}

impl<Diag:Iter<F>+Len+ItemT<T=F>,F:Zero> Iter<F> for DiagonalMatrixGeneric<Diag> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
        let sep=RepeatN::new(&self.zero, self.n()+1);
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
         Ok(self.zero)
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
               let mut res=SparseRowView::new(&self.zero,self.ncols());
               let _=res.insert(i,t);
               res
          })
   }

   fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,IndexOutOfBoundsError<usize>> {
      self.diag
          .get(j)
          .map(|t|{
               let mut res=SparseColView::new(&self.zero,self.ncols());
               let _=res.insert(j,t);
               res
          })
   }

   fn diagonal(&self) -> impl ExactSizeIterator<Item=&F> {
      self.diag
         .iter()
   }
}

impl<F    : Clone+Zero+'static,
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

impl<Diag:ItemT<T=F>,F:Zero> MatrixNotTall for DiagonalMatrixGeneric<Diag> where Self : Matrix<T=F,Row=Diag> {}
impl<Diag:ItemT<T=F>,F:Zero> MatrixNotWide for DiagonalMatrixGeneric<Diag> where Self : Matrix<T=F,Row=Diag> {}
impl<Diag:ItemT<T=F>,F:Zero> MatrixSquare  for DiagonalMatrixGeneric<Diag> where Self : Matrix<T=F,Row=Diag> {}

impl<T:Zero,const N:usize, Diag:ItemT<T=T>+LinearContainerStatic<N>> MatrixFixedNumberOfCols<N> for DiagonalMatrixGeneric<Diag> where Self : MatrixDiagonal<T=T> {}
impl<T:Zero,const N:usize, Diag:ItemT<T=T>+LinearContainerStatic<N>> MatrixFixedNumberOfRows<N> for DiagonalMatrixGeneric<Diag> where Self : MatrixDiagonal<T=T> {}

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
                .map(|cout|DiagonalMatrixGeneric::new(cout))
                .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
        }
    }
}

impl<Diag : AnyFromIterator<F,LinearContainerConstructError>+IntoIter<F>+ItemT<T=F>+Len,
     F : Zero> AnyFromIterator<F,MatrixConstructError> for DiagonalMatrixGeneric<Diag>
     where Self : SizeFromORef<U2> {
   fn any_take_away<I:Iterator<Item=F>>(oref:Option<&Self>, iter: & mut I) -> Result<Self, MatrixConstructError> {
      let n=SizeFromORef::size_from_oref(oref).0;
      let vs:Vec<F>=utils::iter::next_chunk_dyn(iter, n*n)
            .map_err(|e|LenTooSmallError::new(n*n,e.len()))?;
      let mut iter=vs.into_iter();
      let mut first=true;
      let fiter=||{
         let res=if first {
            iter.next()
         } else {
            iter.nth(n)
         };
         first=false;
         res
      };
      Diag::any_from_iter(oref.map(|s|&s.diag),std::iter::from_fn(fiter))
         .map(|diag|DiagonalMatrixGeneric::new(diag))
         .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
   }

   container_traits::any_from_iter_impl!(F,MatrixConstructError);
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
         .map(|diag|DiagonalMatrixGeneric::new(diag))
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
       Ok(Self::new(diag))
   }
}

impl<Diag:ItemT<T=F>,F:Zero> MatrixSquareTryConstruct for DiagonalMatrixGeneric<Diag> where Self : MatrixTryConstruct<T=F,Row=Diag> {}


impl<Diag:ItemT<T=F>,F:Zero> MatrixDiagonal for DiagonalMatrixGeneric<Diag> where Self : MatrixTryConstruct<T=F,Row=Diag> {}

impl<Diag:ItemT<T=F>+TryFromVec<F,LinearContainerConstructError>,F:Zero> TryFromVec<F,LinearContainerConstructError> for DiagonalMatrixGeneric<Diag> {
   fn try_from_vec(vs:Vec<F>) -> Result<Self,LinearContainerConstructError> {
      Diag::try_from_vec(vs)
         .map(|row|Self::new(row))
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


impl<F   : Zero+Clone+Mul<F2,Output=F3>,
     F2  : Clone,
     F3,
     Diag  : ItemT<T=F>+VectorVectorProduct<Vec2,Output=F3>,
     Vec2 : ColVectorView<T=F2>+ChangeT<F3,Output=Vec3>,
     Vec3 : ColVectorTryConstruct<T=F3>> MatrixVectorProduct<Vec2> for DiagonalMatrixGeneric<Diag>
     where Self : MatrixTryConstruct<T=F,Row=Diag> {
   type Output=Vec3;
   fn matrix_vector_product(&self, rhs:&Vec2) -> Vec3 {
       Vec3::any_from_iter(None,
         self.diag.iter().cloned()
            .zip(rhs.iter().cloned())
            .map(|(dii,ri)|dii*ri)).unwrap()
   }
}


impl<F   : Zero+Mul<F2,Output=F3>,
     F2  : Clone,
     F3,
     Diag  : ItemT<T=F>+IntoIter<F>+VectorVectorProduct<Vec2,Output=F3>,
     Vec2 : ColVector<T=F2>+ChangeT<F3,Output=Vec3>,
     Vec3 : ColVectorTryConstruct<T=F3>> IntoMatrixVectorProduct<Vec2> for DiagonalMatrixGeneric<Diag>
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

impl<F  : Clone+Zero+Mul<F2,Output=F3>,
     F2 : Clone,
     F3,
     Diag  : ItemT<T=F>+TryVectorVectorProduct<Col2,Output=F3>,
     Col2 : ColVectorView<T=F2>+ChangeT<F3,Output=Col3>,
     Col3 : ColVectorTryConstruct<T=F3>> TryMatrixVectorProduct<Col2> for DiagonalMatrixGeneric<Diag>
     where Self : MatrixView<T=F> {
   type Output=Col3;
   fn try_matrix_vector_product(&self, rhs:&Col2) -> Result<Col3,VectorConstructError> {
      MatrixCanNotBeMultipliedWithVectorError::try_new(self.n(),rhs.len())?;
      Col3::any_from_iter(None,
         self.diag.iter().cloned()
            .zip(rhs.iter().cloned())
            .map(|(dii,ri)|dii*ri))
         .map_err(|e|e.into())
   }
}

impl<F:Zero+Mul<F2,Output=F3>,F2 : Clone,F3,
     Diag  : ItemT<T=F>+IntoIter<F>+TryVectorVectorProduct<Col2,Output=F3>,
     Col2 : ColVector<T=F2>+ChangeT<F3,Output=Col3>,
     Col3 : ColVectorTryConstruct<T=F3>> TryIntoMatrixVectorProduct<Col2> for DiagonalMatrixGeneric<Diag>
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
     M   : Clone+Matrix<T=F2,Row=MRow>+ChangeT<F3,Output=Out>,
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


impl<F   : 'static+Clone+Zero+Mul<F2,Output=F3>,
     F2  : Clone,
     F3,
     Row : RowVectorTryConstruct<T=F>,
     M   : MatrixView<T=F2>+ChangeT<F3,Output=Out>,
     Out : MatrixTryConstruct<T=F3>> TryIntoMatrixMatrixProduct<M> for DiagonalMatrixGeneric<Row>
     where Out::Row       : RowVectorTryConstruct<T=F3> {
         type Output=Out;
         fn try_into_matrix_matrix_product(self, rhs:&M) -> Result<Out,MatrixConstructError> {
            MatricesCanNotBeMultipliedError::try_new(&self.size(),&rhs.size())?;
            let mut rows=Vec::with_capacity(rhs.nrows());
            for (fi,rowi) in self.into_diagonal().zip(rhs.row_views()) {
               rows.push(Out::Row::any_from_iter(
                  None,
                  rowi.iter().cloned()
                            .map(|rvi|fi.clone()*rvi))?);
            }
            Out::try_from_rows(rows.into_iter())
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


             impl<F  : 'static+std::ops::Mul<F2,Output=F3>$(+$ns)?,
             F2 : 'static+num_traits::Zero+Clone$(+$ns)?,
             F3 : 'static$(+$ns)?,
             const M:usize,
             const N:usize> $crate::TryIntoMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2,N>>> for $m_t<F,M,N> {
                  type Output=$m_t<F3,M,N>;
                  fn try_into_matrix_matrix_product(self, rhs:&DiagonalMatrixGeneric<$Diag_t<F2,N>>) -> Result<$m_t<F3,M,N>,MatrixConstructError> {
                     Self::Output::try_from_cols(
                        <$m_t<F,M,N> as $crate::Matrix>::into_cols(self)
                           .zip(rhs.diagonal().cloned())
                           .map(|(ci,dii)|<<Self as $crate::Matrix>::Col as container_traits::Map::<F,F3>>::map(ci,|cij|cij*dii.clone()))
                     ).map_err(|e|e.into())
                  }
             }


             impl<F  : 'static+Clone+std::ops::Mul<F2,Output=F3>$(+$ns)?,
             F2 : 'static+Clone+num_traits::Zero+Clone$(+$ns)?,
             F3 : 'static$(+$ns)?,
             const M:usize,
             const N:usize> $crate::TryMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2,N>>> for $m_t<F,M,N> {
                  type Output=$m_t<F3,M,N>;
                  fn try_matrix_matrix_product(&self, rhs:&DiagonalMatrixGeneric<$Diag_t<F2,N>>) -> Result<$m_t<F3,M,N>,MatrixConstructError> {
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
             const N:usize> IntoMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2,N>>> for $m_t<F,M,N> where Self : $crate::Matrix<T=F> {
                  type Output=$m_t<F3,M,N>;
                  fn into_matrix_matrix_product(self, rhs:&DiagonalMatrixGeneric<$Diag_t<F2,N>>) -> $m_t<F3,M,N> {
                     <Self as $crate::TryIntoMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2,N>>>>::
                        try_into_matrix_matrix_product(self,rhs)
                         .unwrap()
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

        impl<F  : 'static+Zero+std::ops::Mul<F2,Output=F3>$(+$ns)?,
             F2 : 'static+Clone+Zero$(+$ns)?,
             F3 : 'static+Zero$(+$ns)?> TryIntoMatrixMatrixProduct<DiagonalMatrixGeneric<$Diag_t<F2>>> for $m_t<F> {
                  type Output=$m_t<F3>;
                  fn try_into_matrix_matrix_product(self, rhs:&DiagonalMatrixGeneric<$Diag_t<F2>>) -> Result<$m_t<F3>,MatrixConstructError> {
                     $crate::MatricesCanNotBeMultipliedError::try_new(
                        &<Self as container_traits::Size<(usize,usize)>>::size(&self),
                        &(rhs.n(),rhs.n()))?;
                     Self::Output::try_from_cols(
                        self.into_cols()
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
            +ChangeT<F2,Output=Diag2>
            +ChangeT<F3,Output=Diag3>,
     Diag2 : ItemT<T=F2>+Iter<F2>,
     Diag3 : ItemT<T=F3>
            +AnyFromIterator<F3,LinearContainerConstructError>> MatrixMatrixProduct<DiagonalMatrixGeneric<Diag2>> for DiagonalMatrixGeneric<Diag>
   where DiagonalMatrixGeneric<Diag>  : MatrixView<T=F>,
         DiagonalMatrixGeneric<Diag2> : MatrixView<T=F2>,
         DiagonalMatrixGeneric<Diag3> : Matrix<T=F3> {

         type Output=DiagonalMatrixGeneric<Diag3>;

         fn matrix_matrix_product(&self, rhs:&DiagonalMatrixGeneric<Diag2>) -> DiagonalMatrixGeneric<Diag3> {
             DiagonalMatrixGeneric::<Diag3>::new(Diag3::any_from_iter(None,
                 self.diagonal().cloned()
                     .zip(rhs.diagonal().cloned())
                     .map(|(lhs,rhs)|lhs*rhs)).ok().unwrap())
         }
}


impl<F     : Zero+Mul<F2,Output=F3>,
     F2    : Zero+Clone,
     F3    : Zero,
     Diag  : ItemT<T=F>+LinearContainerSized<T=F>
            +ChangeT<F2,Output=Diag2>
            +ChangeT<F3,Output=Diag3>,
     Diag2 : ItemT<T=F2>+Iter<F2>,
     Diag3 : ItemT<T=F3>
            +AnyFromIterator<F3,LinearContainerConstructError>> IntoMatrixMatrixProduct<DiagonalMatrixGeneric<Diag2>> for DiagonalMatrixGeneric<Diag>
   where DiagonalMatrixGeneric<Diag>  : Matrix<T=F>,
         DiagonalMatrixGeneric<Diag2> : Matrix<T=F2>,
         DiagonalMatrixGeneric<Diag3> : Matrix<T=F3> {

         type Output=DiagonalMatrixGeneric<Diag3>;

         fn into_matrix_matrix_product(self, rhs:&DiagonalMatrixGeneric<Diag2>) -> DiagonalMatrixGeneric<Diag3> {
             DiagonalMatrixGeneric::<Diag3>::new(Diag3::any_from_iter(None,
                 self.diag
                     .into_iterator()
                     .zip(rhs.diag.iter().cloned())
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
