// only use this if its guaranteed that data lives in some multidimensional box

use crate::EmptyVecError;

pub trait Size<Index> {
    fn size(&self) -> Index;
}

impl<T> Size<usize> for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T, const N:usize> Size<usize> for [T;N] {
    fn size(&self) -> usize {
        N
    }
}

pub trait Len : Size<usize> {
    fn len(&self) -> usize {
        <Self as Size<usize>>::size(&self)
    }
}
impl<S:Size<usize>> Len for S {}



#[derive(Clone, Debug, thiserror::Error)]
pub enum CommonSizeError<Index> {
    #[error(transparent)]
    EmptyVec(#[from] EmptyVecError),
    #[error("not all elements have the same size, the sizes are {0}")]
    NotAllHaveSameSize(Vec<Index>)
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum CommonLengthError {
    #[error(transparent)]
    EmptyVec(#[from] EmptyVecError),
    #[error("not all elements have the same length, the sizes are {0:?}")]
    NotAllHaveSameLength(Vec<usize>)
}

impl From<CommonLengthError> for CommonSizeError<usize> {
    fn from(value: CommonLengthError) -> Self {
        match value {
            CommonLengthError::EmptyVec(e)
            => CommonSizeError::EmptyVec(e),
            CommonLengthError::NotAllHaveSameLength(vec) 
            => CommonSizeError::NotAllHaveSameSize(vec)
        }
    }
}

impl From<CommonSizeError<usize>> for CommonLengthError {
    fn from(value: CommonSizeError<usize>) -> Self {
        match value {
            CommonSizeError::EmptyVec(e)
             => CommonLengthError::EmptyVec(e),
            CommonSizeError::NotAllHaveSameSize(vec)
             => CommonLengthError::NotAllHaveSameLength(vec)
        }
    }
}

pub trait TryCommonSize<Index> : Sized+Size<Index> {
    // In Ok result, first number means length of iterator, second common size
    fn try_common_size<'a>(iter:impl ExactSizeIterator<Item=&'a Self>) -> Result<(usize,Index),CommonSizeError<Index>> where Self : 'a;
}


impl<Index:Clone+PartialEq,S:Size<Index>> TryCommonSize<Index> for S {
    fn try_common_size<'a>(iter:impl ExactSizeIterator<Item=&'a Self>) -> Result<(usize,Index),CommonSizeError<Index>> where Self : 'a {
        let mut iter=
            iter.map(Size::size);
        let size0=
            iter.next()
                .ok_or(EmptyVecError)?;
        let mut count=1;
        let mut faults=Vec::new();
        for s in iter {
            if s != size0 {
                faults.push((count,s));
            }
            count+=1;
        }
        if faults.is_empty() {
            return Ok((count,size0));
        }
        // reconstruct the lengths for the error
        let mut sizes=Vec::new();
        let mut curr=0;
        let sz0iter=std::iter::repeat(size0);
        for (i,size) in faults {
            sizes.extend(sz0iter.clone().take(i-curr));
            sizes.push(size);
            curr=i;
        }
        Err(CommonSizeError::NotAllHaveSameSize(sizes))
    }
}


pub trait TryCommonLength : TryCommonSize<usize> {
    // In Ok result, first number means length of iterator, second common length
    fn try_common_length<'a>(iter:impl ExactSizeIterator<Item=&'a Self>) -> Result<(usize,usize),CommonLengthError> where Self : 'a {
        Self::try_common_size(iter)
            .map_err(|e|e.into())
    }
}
impl<A:Len> TryCommonLength for A {}