use nalgebra::{allocator::Allocator, DefaultAllocator, OMatrix, Scalar};
use crate::{FromElement, IndexOutOfBoundsError, LenTooSmallError};

use crate::LinearContainerConstructError as LCCE;

type U2=(usize,usize);
type CCE=ContainerConstructError<U2>;

use num_traits::{Zero,One};

use super::*;
use crate::*;

impl<T : Scalar,
     R : DimExtension,
     C : DimExtension> FromElement<U2,T> for OMatrix<T,R,C>
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


impl<T : Scalar,
     R : Dim,
     C : Dim,
     E : From<DimensionMismatchError<usize>>> TryFromVec<T,E> for OMatrix<T,R,C> where DefaultAllocator : Allocator<R, C> {
    fn try_from_vec(v:Vec<T>) -> Result<Self,E> {
        let len=v.len();
        let (nrows,ncols)=get_dims_from_len::<R,C>(len)
            .map_err(|e|DimensionMismatchError::Other(e))?;
        let r=R::from_usize(nrows);
        let c=C::from_usize(ncols);
        Ok(Self::from_iterator_generic(r, c, v.into_iter()))
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
     R : DimExtension,
     C : DimExtension> TryPutAt<U2,T> for OMatrix<T,R,C>
     where DefaultAllocator : Allocator<R, C> {
    fn try_put_at(size:U2, index:U2, t:T) -> Result<Self,IndexOutOfBoundsError<U2>> {
        IndexOutOfBoundsError::try_new(&size, &index)?;
        let r=R::new(Some(size.0));
        let c=C::new(Some(size.1));
        let mut res=Self::zeros_generic(r, c);
        res[index]=t;
        Ok(res)
    }
}

impl<T : Scalar+Zero+One,
     R : DimExtension,
     C : DimExtension> crate::for_dyn_and_stat::StandardBasis for OMatrix<T,R,C>
     where DefaultAllocator : Allocator<R, C> {
    
    fn try_standard_basis_element(len:usize, index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&len, &index)?;
        let (nrows,ncols)=get_dims_from_len::<R,C>(len).unwrap();
        let i=index % nrows;
        let j=index / nrows;
        Ok(Self::try_put_at((nrows,ncols),(i,j),T::one()).unwrap())
    }
}
