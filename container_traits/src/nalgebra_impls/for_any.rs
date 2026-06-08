use crate::{container::index_iterator::{column_major_index_iterator,row_major_index_iterator}, container_view::ContainerViewable, container_view_mut::ContainerViewMutable};
use nalgebra::{DefaultAllocator, Dim, IsContiguous, Matrix, RawStorage, RawStorageMut, Scalar, allocator::Allocator};


use crate::*;
use super::*;

use crate::LinearContainerConstructError as LCCE;

type U2=(usize,usize);


impl<T : Scalar,
     R : DimExtension,
     C : DimExtension,
     S : RawStorage<T, R, C>> OCTSize<U2> for Matrix<T, R, C, S>
{
    const OCTSIZE: Option<(usize, usize)> = {
        // Const dims return Some(n), Dyn returns None
        let r = R::VALUE;
        let c = C::VALUE;
        match (r, c) {
            (Some(r), Some(c)) => Some((r, c)),
            _ => None,
        }
    };
}

impl<T : Scalar,
     R : DimExtension,
     C : DimExtension,
     S : RawStorage<T, R, C>> OCTSize<usize> for Matrix<T, R, C, S>
{
    const OCTSIZE: Option<usize> = {
        // Const dims return Some(n), Dyn returns None
        let r = R::VALUE;
        let c = C::VALUE;
        match (r, c) {
            (Some(r), Some(c)) => Some(r*c),
            _ => None,
        }
    };
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
     T2: Scalar,
     R : Dim,
     C : Dim,
     S : ChangeT<T2>> ChangeT<T2> for Matrix<T,R,C,S> {
    type Output<'a> = Matrix<T2,R,C,<S as ChangeT<T2>>::Output<'a>>;
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

impl<T:Scalar, R:Dim, C:Dim, S : RawStorage<T,R,C>+IsContiguous> AsSlice<T> for Matrix<T,R,C,S> {
    fn as_slice(&self) ->  &[T] {
        self.as_slice()
    }
}

impl<T:Scalar, R:Dim, C:Dim, S : RawStorageMut<T,R,C>+IsContiguous> AsMutSlice<T> for Matrix<T,R,C,S> {
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
     R : DimExtension,
     C : DimExtension,
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
     S> ItemT for Matrix<T,R,C,S> {
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


impl<T, R, C> ContainerViewable<U2> for nalgebra::OMatrix<T, R, C>
where
    T: Scalar,
    R: DimExtension,
    C: DimExtension,
    DefaultAllocator: Allocator<R, C>,
{
    type Viewer<'a> =
        nalgebra::Matrix<
            T,
            R,
            C,
            nalgebra::ViewStorage<
                'a,
                T,
                R,
                C,
                <<DefaultAllocator as Allocator<R, C>>::Buffer<T>
                    as nalgebra::RawStorage<T, R, C>>::RStride,
                <<DefaultAllocator as Allocator<R, C>>::Buffer<T>
                    as nalgebra::RawStorage<T, R, C>>::CStride,
            >
        >;

    fn as_view<'a>(&'a self) -> Self::Viewer<'a> {
        self.as_view()
    }
}

impl<T, R, C> ContainerViewMutable<U2> for nalgebra::OMatrix<T, R, C>
where
    T: Scalar,
    R: DimExtension,
    C: DimExtension,
    DefaultAllocator: Allocator<R, C>,
{
    type ViewMuter<'a> =
        nalgebra::Matrix<
            T,
            R,
            C,
            nalgebra::ViewStorageMut<
                'a,
                T,
                R,
                C,
                <<DefaultAllocator as Allocator<R, C>>::Buffer<T>
                    as nalgebra::RawStorage<T, R, C>>::RStride,
                <<DefaultAllocator as Allocator<R, C>>::Buffer<T>
                    as nalgebra::RawStorage<T, R, C>>::CStride,
            >
        >;

    fn as_view_mut<'a>(&'a mut self) -> Self::ViewMuter<'a> {
        self.as_view_mut()
    }
}