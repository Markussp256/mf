use crate::container::index_iterator::{column_major_index_iterator,row_major_index_iterator};
use nalgebra::{allocator::Allocator, DMatrix, DefaultAllocator, Dim, IsContiguous, Matrix, OMatrix, RawStorage, RawStorageMut, Scalar};
use num_traits::Zero;
use crate::*;
use super::*;

use crate::LinearContainerConstructError as LCCE;

type U2=(usize,usize);
type CCE=ContainerConstructError<U2>;

impl<T : Scalar,
     R : Dim,
     C : Dim> FromElement<U2,T> for OMatrix<T,R,C>
    where DefaultAllocator: Allocator<R, C>, Self : OCTSize<U2> {
    fn from_element(size:U2,t:T) -> Self {
        let r=R::new(Some(size.0));
        let c=C::new(Some(size.1));
        Self::from_element_generic(r,c,t)
    }
}


impl<T : Scalar,
     R : Dim,
     C : Dim,
     E : From<LCCE>> TryFromFn<usize,T,E> for OMatrix<T,R,C>
    where DefaultAllocator: Allocator<R, C> , Self : OCTLen {
    fn try_from_fn(size:usize, f:impl Fn(usize) -> T) -> Result<Self,E> {
        let (r,c)=
        if        R::try_to_usize() == Some(1) {
            (1,size)
        } else if C::try_to_usize() == Some(1) {
            (size,1)
        } else {
            assert!(false); // panic!("number of rows or of columns must be 1")
            (0,0)
        };
        let r=R::from_usize(r);
        let c=C::from_usize(c); 
        Ok(Self::from_fn_generic(r,c,|i,j| f(if i == 0 {j} else {i})))
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     E : From<CCE>> TryFromFn<U2,T,E> for OMatrix<T,R,C>
    where DefaultAllocator: Allocator<R, C>, Self : OCTSize<U2> {

    fn try_from_fn((r,c):U2, f:impl Fn(U2) -> T) -> Result<Self,E> {
        let r=R::from_usize(r);
        let c=C::from_usize(c);
        Ok(Self::from_fn_generic(r,c,|i,j| f((i,j))))
    }
}

impl<T:Scalar> FromFn<U2,T> for DMatrix<T> {
    fn from_fn((r,c):U2, f:impl Fn(U2) -> T) -> Self {
        Self::from_fn(r,c, |i,j|f((i,j)))
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     E : From<LCCE>> TryFromVec<T,E> for OMatrix<T,R,C> where DefaultAllocator : Allocator<R, C> {
    fn try_from_vec(v:Vec<T>) -> Result<Self,E> {
        let len=v.len();
        let (nrows,ncols)=get_dims_from_len::<R,C>(len)
            .map_err(|_|OtherDimensionMismatchError.into())?;
        let r=R::from_usize(nrows);
        let c=C::from_usize(ncols);
        Ok(Self::from_iterator_generic(r, c, v.into_iter()))
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> NumberOfDegreesOfFreedom<T> for Matrix<T,R,C,S> {
    fn ndofs(&self) -> usize {
        self.nrows()*self.ncols()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> Size<U2> for Matrix<T,R,C,S> {
    fn size(&self) -> U2 {
        (self.nrows(),self.ncols())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> IsEmpty for Matrix<T,R,C,S> {
    fn is_empty(&self) -> bool {
        self.nrows() == 0 ||
        self.ncols() == 0
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> Size<usize> for Matrix<T,R,C,S> {
    fn size(&self) -> usize {
        if self.nrows() == 1 {
            self.ncols()
        } else if self.ncols() == 1 {
            self.nrows()
        } else {
            assert!(false); // panic!("Asking for 1d-size of a 2d nalgebra matrix")
            0
        }
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim> Zeros<U2,T> for OMatrix<T,R,C>
     where DefaultAllocator: Allocator<R, C> {
    fn zeros((nrows,ncols):U2) -> Self where T:Zero {
        let r=R::from_usize(nrows);
        let c=C::from_usize(ncols);
        Self::from_element_generic(r,c,T::zero())
    }
}

impl<T : Scalar,
     T2: Scalar,
     R : Dim,
     C : Dim> ChangeT<T2> for OMatrix<T,R,C> where DefaultAllocator : Allocator<R, C> {
    type Output = OMatrix<T2,R,C>;
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorageMut<T,R,C>> ChangeDim for Matrix<T,R,C,S> {
    type Output<const RR:usize,const CC:usize> = nalgebra::SMatrix<T,RR,CC>;
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorageMut<T,R,C>> GetMut<usize, T> for Matrix<T,R,C,S> {
    fn get_mut(& mut self, index:usize) -> Result<& mut T,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        Ok(self.get_mut(index).unwrap())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorageMut<T,R,C>> GetMut<U2, T> for Matrix<T,R,C,S> {
    fn get_mut(& mut self, index:U2) -> Result<& mut T,IndexOutOfBoundsError<U2>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        Ok(self.get_mut(index).unwrap())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> Get<usize, T> for Matrix<T,R,C,S> {
    fn get(&self, index:usize) -> Result<& T,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        Ok(self.get(index).unwrap())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> Get<U2, T> for Matrix<T,R,C,S> {
    fn get(&self, index:U2) -> Result<&T,IndexOutOfBoundsError<U2>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        Ok(self.get(index).unwrap())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> First<T> for Matrix<T,R,C,S> {
    fn first(&self) -> Result<&T,EmptyContainerError> {
        self.get((0,0))
            .ok_or(EmptyContainerError)
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> Last<T> for Matrix<T,R,C,S> {
    fn last(&self) -> Result<&T,EmptyContainerError> {
       let sz:(usize,usize)=self.size();
       if sz.iter().any(|szi|szi == &0) {
           return Err(EmptyContainerError);
       }
       let index=<(usize,usize)>::try_from_iter(sz.into_iterator().map(|szi|szi-1)).unwrap();
       Ok(self.get(index).unwrap())
    }
}

impl<T:Scalar, R:Dim, C:Dim, S : RawStorage<T,R,C>+IsContiguous> AsSlice<T> for Matrix<T,R,C,S>
    where DefaultAllocator : Allocator<R,C> {
    fn as_slice(&self) ->  &[T] {
        self.as_slice()
    }
}

impl<T:Scalar, R:Dim, C:Dim, S : RawStorageMut<T,R,C>+IsContiguous> AsMutSlice<T> for Matrix<T,R,C,S>
    where DefaultAllocator : Allocator<R,C> {
    fn as_mut_slice(& mut self) ->  & mut [T] {
        self.as_mut_slice()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorageMut<T,R,C>> IterMutIndexed<usize, T> for Matrix<T,R,C,S> {
    fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> where T:'a {
        self.iter_mut()
            .enumerate()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorageMut<T,R,C>> IterMutIndexed<U2, T> for Matrix<T,R,C,S> {
    fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(U2,&'a mut T)> where T:'a {
        column_major_index_iterator(self.size())
            .zip(self.iter_mut())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> IterIndexed<usize, T> for Matrix<T,R,C,S> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(usize,&'a T)> where T:'a {
        self.iter()
            .enumerate()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> IterIndexed<U2, T> for Matrix<T,R,C,S> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a T)> where T:'a {
        column_major_index_iterator(self.size())
            .zip(self.iter())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> IntoIterIndexed<usize, T> for Matrix<T,R,C,S> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(usize,T)> {
        // nalgebra does not seem to have an appropriate into_iter method
        let v:Vec<(usize,T)>=
            self.iter()
                .cloned()
                .enumerate()
                .collect();
        v.into_iter()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> IntoIter<T> for Matrix<T,R,C,S> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T> {
        let v:Vec<T>=
            row_major_index_iterator(self.size())
                .map(|(i,j)|self[(i,j)].clone())
                .collect();
        v.into_iter()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> IntoIterIndexed<U2,T> for Matrix<T,R,C,S> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(U2,T)> {
        let v:Vec<T>=
            self.iter()
                .cloned()
                .collect();
        column_major_index_iterator(self.size())
            .zip(v.into_iter())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> IntoVec<T> for Matrix<T,R,C,S> {
    fn into_vec(self) -> Vec<T> {
        self.iter()
            .cloned()
            .collect()
    }
}

impl<T : Scalar,
     E,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> TryAccept<U2,T,E> for Matrix<T,R,C,S> where Self : OCTSize<U2> {
    fn try_accept<'a>(_:U2,_:impl Fn(U2) -> &'a T) -> Result<(),E> where T: 'a {
        Ok(())
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>,
     E : From<LCCE>> TryAccept<usize,T,E> for Matrix<T,R,C,S> where Self : OCTSize<usize> {
    fn try_accept<'a>(size:usize,_:impl Fn(usize) -> &'a T) -> Result<(),E> {
             if R::try_to_usize() == Some(1) { C::try_new(Some(size)).map_err(Into::into)?; Ok(()) }
        else if C::try_to_usize() == Some(1) { R::try_new(Some(size)).map_err(Into::into)?; Ok(()) }
        else { Err(LCCE::from(OtherDimensionMismatchError).into()) } // LCCE::DimensionMismatch(DimensionMismatchError::Other(OtherDimensionMismatchError)).into()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> ItemT for Matrix<T,R,C,S> {
    type T=T;
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorageMut<T,R,C>> IterMut<T> for Matrix<T,R,C,S> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        self.iter_mut()
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> Iter<T> for Matrix<T,R,C,S> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.iter()
    }
}

impl<T : Scalar,
     T2: Scalar,
     R : Dim,
     C : Dim> Map<T,T2> for OMatrix<T,R,C> where DefaultAllocator : Allocator<R, C> {
    type Output = OMatrix<T2,R,C>;
    fn map(self, f:impl Fn(T) -> T2) -> Self::Output {
        OMatrix::map(&self,f)
    }
}


impl<T : Scalar,
     T2: Scalar,
     R : Dim,
     C : Dim,
     E : From<OtherDimensionMismatchError>> TryMap<T,T2,E> for OMatrix<T,R,C> where DefaultAllocator : Allocator<R, C> {
    type Output = OMatrix<T2,R,C>;
    fn try_map(self, f:impl Fn(T) -> T2) -> Result<Self::Output,E> {
        Ok(OMatrix::map(&self,f))
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim> TryMapI<U2, T> for OMatrix<T,R,C> where DefaultAllocator : Allocator<R, C> {
    fn try_map_i(self, index:U2, f:impl FnOnce(& mut T)) -> Result<Self,IndexOutOfBoundsError<U2>> {
        IndexOutOfBoundsError::try_new(&self.size(), &index)?;
        let mut smut=self;
        if let Some(x)=smut.get_mut(index) {
            f(x);
        }
        Ok(smut)
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim> TryMapI<usize, T> for OMatrix<T,R,C> where DefaultAllocator : Allocator<R, C> {
    fn try_map_i(self, index:usize, f:impl FnOnce(& mut T)) -> Result<Self,IndexOutOfBoundsError<usize>> {
        if        self.nrows() == 1 {
            IndexOutOfBoundsError::try_new(&self.ncols(),&index)?;
            Ok(self.try_map_i((0,index),f).unwrap())
        } else if self.ncols() == 1 {
            IndexOutOfBoundsError::try_new(&self.nrows(),&index)?;
            Ok(self.try_map_i((index,0),f).unwrap())
        } else {
            assert!(false);// panic!("TryMapI<usize,...> is only supposed to be used if number of rows or number of cols is 1");
            Ok(self)
        }
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     E : From<OtherDimensionMismatchError>> AnyFromIterator<T,E> for OMatrix<T,R,C>
     where DefaultAllocator              : Allocator<R, C>,
           LenTooSmallError              : Into<E>,
           LenNotEqualToRequiredLenError : Into<E>,
           OtherDimensionMismatchError   : Into<E> {
    fn any_take_away<I:Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,E> {
        let (r,c,v)=match oref {
            Some(tref) => {
                let (r,c)=tref.size();
                let len=r*c;
                let v=
                    utils::iter::next_chunk_dyn(iter, len)
                        .map_err(|e|LenTooSmallError::new(len, e.len()).into())?;
                (r, c, v)
            },
            None => {
                let v:Vec<T>=iter.collect();
                let (r,c)=get_dims_from_len::<R,C>(v.len()).map_err(Into::into)?;
                (r, c, v)
            }
        };
        let r=R::from_usize(r);
        let c=C::from_usize(c);
        Ok(Self::from_iterator_generic(r, c, v))
    }
    crate::any_from_iter_impl!(T, E);
}

impl<T : Scalar+Zero,
     R : Dim,
     C : Dim> TryPutAt<U2,T> for OMatrix<T,R,C>
     where DefaultAllocator : Allocator<R, C> {
    fn try_put_at(size:U2, index:U2, t:T) -> Result<Self,IndexOutOfBoundsError<U2>> {
        IndexOutOfBoundsError::try_new(&size, &index)?;
        let r=R::new(Some(size.0));
        let c=C::new(Some(size.1));
        let mut res=Self::zeros_generic(r, c);
        res[size]=t;
        Ok(res)
    }
}

// TryFromVec is missing...
// we would need to implement it for all but when one dimension is 1 and one dimension is dynamic
// i.e. DVector and RowDVector

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> TryIntoElement<usize,T> for Matrix<T,R,C,S> {
    fn try_into_element(self,index:usize) -> Result<T,IndexOutOfBoundsError<usize>> {
        if        self.nrows() == 1 {
            IndexOutOfBoundsError::try_new(&self.ncols(),&index)?;
            Ok(self[(0,index)].clone())
        } else if self.ncols() == 1 {
            IndexOutOfBoundsError::try_new(&self.nrows(),&index)?;
            Ok(self[(index,0)].clone())
        } else {
            assert!(false);
            Ok(self[(0,0)].clone())
            // panic!("matrix is supposed to have only one row or only one column")
        }
    }
}

impl<T : Scalar,
     R : Dim,
     C : Dim,
     S : RawStorage<T,R,C>> TryIntoElement<U2,T> for Matrix<T,R,C,S> {
    fn try_into_element(self,index:U2) -> Result<T,IndexOutOfBoundsError<U2>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        Ok(self[index].clone())
    }
}