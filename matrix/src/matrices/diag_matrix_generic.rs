
use algebra_traits::{Det,MulError,DivError,FloatOpError};
use algebra::{EnhancedArray, EnhancedVec, Vector, VectorDyn};
use container_traits::for_static::TryFromIterator;
use num_traits::{Zero,One};
use container_traits::{AnyFromIterator, AnyFromVec, AnyMap, ContainerIndex, Get, IndexedIter,  IntoIndexedIter, IntoIter, IntoProduct, ItemT, Iter, Len, LenTooSmallError, LinearContainer, LinearContainerTryConstruct, LinearContainerConstructError, Map, NumberOfDegreesOfFreedom, OCTSize, Size, SizeFromORef, TryAccept, TryFromFn, TryIntoElement, TryPutAt, Zeros};
use utils::iter::{InterLeave, RepeatN,};

use crate::row_col::{MatrixColGeneric, MatrixRowGeneric};
use crate::{MatrixColDyn, MatrixCol};

use matrix_traits::*;

type U2=(usize,usize);

#[derive(Clone,
         Debug,
         algebra_derive::ScalarContainer,
)]
pub struct DiagonalMatrixGeneric<C:LinearContainer> {
   diag:C,
   zero:C::T
}

pub type DiagonalMatrixDyn<F>=DiagonalMatrixGeneric<EnhancedVec<F>>;
pub type DiagonalMatrix<F,const N:usize>=DiagonalMatrixGeneric<EnhancedArray<F,N>>;

impl<F:Zero, const N:usize> SquareStaticMatrix for DiagonalMatrix<F,N> {
   const M:usize = N;
}

impl<F:Zero,const N:usize> StaticMatrix for DiagonalMatrix<F,N> {
   const M:usize = N;
   const N:usize = N;
}

impl<C:LinearContainer<T=F>,F:Zero> DiagonalMatrixGeneric<C> {
   pub fn new(diag:impl Into<C>) -> Self {
      Self{diag:diag.into(),zero:F::zero()}
   }

   pub fn n(&self) -> usize {
      self.diag
          .len()
   }

   pub fn map_diagonal<C2:LinearContainerTryConstruct<E>,E>(self, f:impl Fn(F)->C2::T) -> DiagonalMatrixGeneric<C2> where C:Map<F,C2::T,Output=C2>, C2::T:Zero {
      DiagonalMatrixGeneric::new(self.diag.map(f))
   }
}

macro_rules! impl_try {
   ($tr:ident, $fn:ident) => {
      paste::paste!(
      impl<C:LinearContainerTryConstruct<T=F>,F:Zero+std::ops::$tr<Output=F>> algebra_traits::[<Try $tr>] for DiagonalMatrixGeneric<C> {
         type Output=Self;
         fn [<try_ $fn>](self,rhs:Self) -> Result<Self::Output, [<$tr Error>]> {
            if self.n() != rhs.n() { return Err([<$tr Error>]::FloatOp(FloatOpError::not_same_dim()))}
            Ok(Self::new(
                  C::any_from_vec(
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

impl<C:LinearContainer<T=F>,F:Zero> Transpose for DiagonalMatrixGeneric<C> {
    type Output=Self;

    fn transpose(self) -> Self::Output {
        self
    }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> Get<U2,F> for DiagonalMatrixGeneric<C> {
   fn get(&self,(i,j):U2) -> Option<&F> {
      if i >= self.n() ||
         j >= self.n() {
         return None;
      }
      if i == j {
         self.diag.get(i)
      } else {
         Some(&self.zero)
      }
   }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> Iter<F> for DiagonalMatrixGeneric<C> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
        let sep=RepeatN::new(&self.zero, self.n()+1);
        self.diag.iter().inter_leave(sep)
    }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> IndexedIter<U2,F> for DiagonalMatrixGeneric<C> {
   fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a F)> where F:'a {
       (self.n(),self.n()).index_iterator()
                          .zip(self.iter())
   }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> ItemT for DiagonalMatrixGeneric<C> {
   type T=F;
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> IntoIter<F> for DiagonalMatrixGeneric<C> {
   fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
      let sep=utils::iter::RepeaterN::new(||F::zero(), self.n()+1);// RepeatN::new(F::zero(), self.n()+1);
      self.diag.into_iterator()
          .inter_leave(sep)
   }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> IntoIndexedIter<U2,F> for DiagonalMatrixGeneric<C> {
   fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(U2,F)> {
       (self.n(),self.n()).index_iterator()
                          .zip(self.into_iterator())
   }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> TryIntoElement<U2,F> for DiagonalMatrixGeneric<C> {
   fn try_into_element(self,index:U2) -> Option<F> {
      if index.0 == index.1 {
         self.diag
            .try_into_element(index.0)
      } else if index.0 < self.n() &&
                index.1 < self.n() {
         Some(self.zero)
      } else {
         None
      }
   }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> Size<U2> for DiagonalMatrixGeneric<C> {
   fn size(&self) -> U2 {
       let n=self.n();
       (n,n)
   }
}

impl<C:LinearContainerTryConstruct<T=F>,F:Zero> OCTSize<U2> for DiagonalMatrixGeneric<C> {
   const OCTSIZE:Option<U2>=match C::OCTSIZE {
      Some(n) => Some((n,n)),
      None => None
   };
}


impl<C:LinearContainerTryConstruct<T=F>,F:Zero> NumberOfDegreesOfFreedom<F> for DiagonalMatrixGeneric<C> {
   fn ndofs(&self) -> usize {
       self.n()
   }
}

impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>, F:Zero> Matrix for DiagonalMatrixGeneric<C> {
   type Row=MatrixRowGeneric<C>;
   type Col=MatrixColGeneric<C>;

   fn nrows(&self) -> usize {
      self.n()
   }
   fn ncols(&self) -> usize {
      self.n()
   }

   fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
      let n=self.n();
      self.diag
          .into_iterator()
          .enumerate()
          .map(move |(i,f)|Self::Row::try_put_at(i, n, f).unwrap())
   }

   fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
      let n=self.nrows();
      self.diag
          .into_iterator()
          .enumerate()
          .map(move |(i,f)|Self::Col::try_put_at(i, n, f).unwrap()) 
   }
}

impl<C:LinearContainer> AsBaseMatrix for DiagonalMatrixGeneric<C> where Self:Matrix {
    type Output = Self;
    fn base_matrix(&self) -> &Self {
        self
    }
}

impl<C:LinearContainer> IntoBaseMatrix for DiagonalMatrixGeneric<C> where Self:Matrix {
    type Output = Self;
    fn into_base_matrix(self) -> Self {
        self
    }
}

impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> matrix_traits::matrices::matrix_shapes::MatrixNotTall for DiagonalMatrixGeneric<C> {}
impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> matrix_traits::matrices::matrix_shapes::MatrixNotWide for DiagonalMatrixGeneric<C> {}
impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> matrix_traits::matrices::matrix_shapes::MatrixSquare  for DiagonalMatrixGeneric<C> {}
matrix_traits::impl_matrixii_one_param!(DiagonalMatrix, Zero, 1,2,3,4,5,6,7,8,9);

impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> TryAccept<U2,F,MatrixConstructError> for DiagonalMatrixGeneric<C> {
    fn try_accept<'a>(size:U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
         let (nrows,ncols)=id.size();
         if nrows == ncols {
            return Err(MatrixConstructError::DimensionMismatch);
         }
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
     C    : LinearContainerTryConstruct<T=F>+AnyMap<F,FOut,LinearContainerConstructError,Output=COut>,
     COut : LinearContainerTryConstruct<T=FOut>> AnyMap<F,FOut,MatrixConstructError> for DiagonalMatrixGeneric<C> {
    type Output=DiagonalMatrixGeneric<COut>;

    fn any_map(self, f:impl Fn(F) -> FOut) -> Result<Self::Output,MatrixConstructError> {
        if self.n() > 1 && !f(F::zero()).is_zero() {
            Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
        } else {
            self.diag
                .any_map(f)
                .map(|cout|DiagonalMatrixGeneric::new(cout))
                .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
        }
    }
}

impl<C : LinearContainerTryConstruct<T=F>, F : Zero> AnyFromIterator<F,MatrixConstructError> for DiagonalMatrixGeneric<C> {
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
      C::any_from_iter(oref.map(|s|&s.diag),std::iter::from_fn(fiter))
         .map(|c|DiagonalMatrixGeneric::new(c))
         .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
   }

   container_traits::any_from_iter_impl!(F,MatrixConstructError);
}

impl<C : LinearContainerTryConstruct<T=F>, F : Zero> TryFromFn<U2,F,MatrixConstructError> for DiagonalMatrixGeneric<C> {
   fn try_from_fn(id:InstanceStructureDescriptor<Self,U2>,f:impl Fn(U2) -> F) -> Result<Self,MatrixConstructError> {
      let (nrows,ncols)=id.size();
      for i in 0..nrows {
         for j in 0..ncols {
            if i != j && !f((i,j)).is_zero() {
               return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
            }
         }
      }
      C::try_from_fn(InstanceStructureDescriptor::Size(nrows), |i|f((i,i)))
         .map(|c|DiagonalMatrixGeneric::new(c))
         .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
   }
}

impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> MatrixTryConstruct for DiagonalMatrixGeneric<C> {
   fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError> {
      let rows:Vec<Self::Row>=rows.collect();
      Self::try_accept_vec_of_rows(rows.iter())?;
      let c=C::any_from_iter(
         None,
         rows.into_iter()
                   .enumerate()
                   .map(|(i,row)|row.try_into_element(i).unwrap().into())).unwrap();
       Ok(Self::new(c))
   }
}

impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> MatrixSquareTryConstruct for DiagonalMatrixGeneric<C> {}


impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> MatrixDiagonal for DiagonalMatrixGeneric<C> {}

impl<C:LinearContainerTryConstruct<T=F>+TryPutAt<usize,F>,F:Zero> AnyFromVec<F,LinearContainerConstructError> for DiagonalMatrixGeneric<C> {
   fn any_from_vec(vs:Vec<F>) -> Result<Self,LinearContainerConstructError> {
      C::any_from_vec(vs)
         .map(|c|Self::new(c))
   }
}

impl<C:LinearContainer<T=F>+TryPutAt<usize,F>,F:Zero+One> Det for DiagonalMatrixGeneric<C> {
   type Output=F;
   fn det(self) -> F  {
      self.diag
          .into_iterator()
          .into_product()
   }
}

impl<F> Zeros<usize,F> for DiagonalMatrixDyn<F> {
    fn zeros(size:usize) -> Self where F:Zero {
        let c=EnhancedVec::<F>::zeros(size);
        DiagonalMatrixDyn::new(c)
    }
}


macro_rules! diag_times_vector {
   ($name:ident) => {
         impl<F:std::ops::Mul<Output=F>, const N:usize> MatrixVectorProduct<$name<F,N>> for DiagonalMatrix<F,N> {
            type Output=$name<F,N>;
            fn matrix_vector_product(self, rhs:$name<F,N>) ->$name<F,N> {
                 <$name<F,N> as TryFromIterator<F,LinearContainerConstructError>>::try_from_iter(
                     self.diag
                         .into_iterator()
                         .zip(rhs.into_iterator())
                         .map(|(dii,vi)|dii*vi)).ok().unwrap()
            }
         }
   };
}
diag_times_vector!(Vector);
diag_times_vector!(MatrixCol);

macro_rules! try_diag_times_vector {
   ($name:ident) => {
         impl<F:std::ops::Mul<Output=F>> TryMatrixVectorProduct<$name<F>> for DiagonalMatrixDyn<F> {
            type Output=$name<F>;
            fn try_matrix_vector_product(self, rhs:$name<F>) -> Option<$name<F>> {
                 (self.diag.len() == rhs.len()).then(||
                     $name::from_iter(
                        self.diag
                            .into_iterator()
                            .zip(rhs.into_iterator())
                            .map(|(dii,vi)|dii*vi)))
            }
         }
   };
}
try_diag_times_vector!(VectorDyn);
try_diag_times_vector!(MatrixColDyn);


impl<F:Zero+std::ops::Mul<Output=F>,const N:usize> MatrixMatrixProduct<Self> for DiagonalMatrix<F,N> {
    type Output=Self;

    fn matrix_matrix_product(self, rhs:Self) -> Self {
        Self::new(<EnhancedArray::<F,N> as TryFromIterator<F,LinearContainerConstructError>>::try_from_iter(
            self.diag
                .into_iterator()
                .zip(rhs.diag.into_iterator())
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
         
         impl<$f:'static+Clone+Zero+std::ops::Mul<Output=$f>
                  +algebra_traits::ScalarMul<$f> $(, const $ns:usize)*>
               matrix_traits::MatrixMatrixProduct<$crate::DiagonalMatrix<$f,N>> for $t {
            type Output=$t;
            fn matrix_matrix_product(self, rhs:$crate::DiagonalMatrix<$f,N>) -> Self::Output {
               <Self::Output as matrix_traits::MatrixTryConstruct>::try_from_cols(
                <Self as matrix_traits::Matrix>::into_cols(self)
                    .zip(<_ as matrix_traits::Matrix>::into_diagonal(rhs))
                    .map(|(c,dii)|<_ as algebra_traits::ScalarMul<$f>>::scalar_mul(c,&dii))).unwrap()
            }
         }
    };
}
