use crate::{DimensionMismatchError, IndexOutOfBoundsError, LowerBoundUpperBoundError};


pub trait TryIterBounded<Index,T> {
    fn try_iter_bounded<'a>(&'a self,lb:Index,ub:Index) -> Result<impl ExactSizeIterator<Item=&'a T>,DimensionMismatchError<Index>>
    where T:'a;
}


macro_rules! try_iter_bounded_impl {
    () => {
        fn try_iter_bounded<'a>(&'a self,lb:usize,ub:usize) -> Result<impl ExactSizeIterator<Item=&'a T>,DimensionMismatchError<usize>> where T:'a {
            LowerBoundUpperBoundError::try_new(&lb,&ub)?;
            IndexOutOfBoundsError::try_new(&self.len(),&ub)?;
            Ok(self[lb..=ub]
                 .iter())
        }
    };
}

impl<T:> TryIterBounded<usize,T> for Vec<T> {
    try_iter_bounded_impl!();
}

impl<T, const N:usize> TryIterBounded<usize,T> for [T;N] {
    try_iter_bounded_impl!();
}

// pub fn impl_try_iter_bounded_from_get
//     <'a,
//      Index : ContainerIndex,
//      T : 'a,
//      G : 'a+Get<Index,T>>(g:&'a G,size:Index,lb:Index,ub:Index) -> Result<impl ExactSizeIterator<Item=&'a T>,DimensionMismatchError<Index>> {
//         IndexOutOfBoundsError::try_new(&size, &ub)?;
//         let cii=ContainerIndexIterator::try_new_exact_size_from_lb_ub(lb,ub)?;
//         Ok(cii.map(move |index:Index|g.get(index).unwrap()))
// }