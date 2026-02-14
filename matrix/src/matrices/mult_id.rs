use container_traits::{AnyFromIterator, ClosedMap, Get, IndexOutOfBoundsError, IntoIter, IntoIterIndexed, IsEmpty, ItemT, Iter, IterIndexed, LenTooSmallError, Map, NumberOfDegreesOfFreedom, OCTSize, Size, TryAccept, TryFromFn, TryIntoElement, TryMap};
use num_traits::Zero;
use std::ops::Mul;

use matrix_traits::{Matrix, MatrixConstructError, MatrixTryConstruct, MatrixView};

use crate::{MatrixCol, MatrixRow, MatrixColView, MatrixRowView};

type U2=(usize,usize);

// this matrix can magically change its size to whatever is needed for multiplication

pub struct MultId<F:Zero>{
    factor:F,
    zero:F
}

impl<F:Zero> MultId<F> {
    pub fn new(factor:F) -> Self {
        Self{factor,zero:F::zero()}
    }
}

impl<F:Zero> Get<U2,F> for MultId<F> {
    fn get(&self, (i,j):U2) -> Result<&F,IndexOutOfBoundsError<U2>> {
        if i == j {
            Ok(&self.factor)
        } else {
            Ok(&self.zero)
        }
    }
}

impl<F:Zero> Iter<F> for MultId<F> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
       std::iter::once(&self.factor)
    }
}

impl<F:Zero> IterIndexed<U2,F> for MultId<F> {
   fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a F)> where F:'a {
        std::iter::once(((0,0),&self.factor))
   }
}

impl<F:Zero> ItemT for MultId<F> {
    type T=F;
}

impl<F:Zero> TryIntoElement<U2,F> for MultId<F> {
    fn try_into_element(self,index:U2) -> Result<F,IndexOutOfBoundsError<U2>> {
        if index == (0,0) {
            Ok(self.factor)
        } else {
            Err(IndexOutOfBoundsError::new(&(1,1),&index))
        }
    }
}

impl<F:Zero> IntoIter<F> for MultId<F> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
        std::iter::once(self.factor)
    }
}

impl<F:Zero> IntoIterIndexed<U2,F> for MultId<F> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(U2,F)> {
        std::iter::once(((0,0),self.factor))
    }
}

impl<F:Zero> Size<U2> for MultId<F> {
    fn size(&self) -> U2 {
        (1,1)
    }
}

impl<F:Zero> NumberOfDegreesOfFreedom<F> for MultId<F> {
    fn ndofs(&self) -> usize {
        1
    }
}

impl<F:Zero> OCTSize<U2> for MultId<F> {
    const OCTSIZE:Option<U2> = Some((1,1));
}

impl<F:Zero> IsEmpty for MultId<F> {
    fn is_empty(&self) -> bool {
        false
    }
}


impl<F:Zero> MatrixView for MultId<F> {
    type RowView<'a>=MatrixRowView<'a,F,1> where Self : 'a;
    type ColView<'a>=MatrixColView<'a,F,1> where Self : 'a;
    
    fn try_row_view<'a>(&'a self, i:usize) -> Result<Self::RowView<'a>,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&1,&i)?;
        Ok(MatrixRowView::<'a,F,1>::from(self.factor.clone()))
    }
    
    fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&1,&j)?;
        Ok(MatrixColView::<'a,F,1>::from(self.factor.clone()))
    }
}

impl<F:Zero> Matrix for MultId<F> {
    type Row=MatrixRow<F,1>;

    type Col=MatrixCol<F,1>;

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        std::iter::once(MatrixRow::<F,1>::from([self.factor]))
    }

    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        std::iter::once(MatrixCol::<F,1>::from([self.factor]))
    }
}

impl<F:Zero+PartialEq> TryAccept<U2,F,MatrixConstructError> for MultId<F> {
    fn try_accept<'a>(size:U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        if size.0 != size.1 || (size == (0,0)) { return Err(MatrixConstructError::DimensionMismatch)}
        let n=size.0;
        let factor=f((0,0));
        for i in 0..n {
            for j in 0..n {
                let rij=f((i,j));
                 if i != j && !rij.is_zero() ||
                    i == j &&  rij != factor {
                    return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
                }
            }
        }
        Ok(())
    }
}

impl<F:Zero,F2:Zero> TryMap<F,F2,MatrixConstructError> for MultId<F> {
    type Output = MultId<F2>;
    fn try_map(self, f: impl Fn(F) -> F2) -> Result<MultId<F2>,MatrixConstructError> {
        if !f(F::zero()).is_zero() {
            return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
        }
        Ok(MultId::new(f(self.factor)))
    }
}

impl<F:Zero> AnyFromIterator<F,MatrixConstructError> for MultId<F> {
    fn any_take_away<I:    Iterator<Item=F>>(_:Option<&Self>, iter:& mut I) -> Result<Self,MatrixConstructError> {
        match iter.next() {
            Some(e) => Ok(Self::new(e)),
            None => Err(LenTooSmallError::new(1,0).into())
        }
    }

    container_traits::any_from_iter_impl!(F,MatrixConstructError);
}

impl<F:Zero> TryFromFn<U2,F, MatrixConstructError> for MultId<F> {
    fn try_from_fn(_:U2, f:impl Fn(U2) -> F) -> Result<Self,MatrixConstructError> {
        Ok(Self::new(f((0,0))))
    }
}

impl<F:Zero+PartialEq> MatrixTryConstruct for MultId<F> {
    fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError> {
        let rows:Vec<Self::Row>=rows.collect();
        if rows.is_empty() { return Err(MatrixConstructError::DimensionMismatch); }
        Self::try_accept_vec_of_rows(rows.iter())?;
        Ok(rows.try_into_element(0).unwrap()
               .try_into_element(0).unwrap().into())
    }
}


impl<F:Zero> From<F> for MultId<F> {
    fn from(factor: F) -> Self {
        Self::new(factor)
    }
}

impl<F:Zero+Clone+PartialEq> MultId<F> {
        // overwrites std::ops::Mul
        pub fn mul<RHS:MatrixTryConstruct>(self,rhs:RHS) -> RHS
        where    F : Mul<RHS::T,Output=RHS::T>,
               RHS : ClosedMap<RHS::T> {
            rhs.map(|f2|self.factor.clone()*f2)
        }
}

impl<F:Zero+Mul<F2,Output=F3>+Clone,
     F2,
     F3,
     M2:Map<F2,F3, Output=M3>+Matrix<T=F2>,
     M3:Matrix<T=F3>> Mul<M2> for MultId<F> {
        type Output=M3;
        
        fn mul(self, rhs: M2) -> Self::Output {
            rhs.map(|f2|self.factor.clone()*f2)
        }
}