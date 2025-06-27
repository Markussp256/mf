use std::ops::Deref;

use crate::{CommonLengthError, AnyCommonLength};

pub trait Len {
    fn len(&self) -> usize;
}

impl<T,S:Deref<Target=[T]>> Len for S {
    fn len(&self) -> usize {
        self.as_slice()
            .len()
    }
}

pub trait TryCommonLength : Sized+Len {
    fn try_common_length<'a>(iter:impl ExactSizeIterator<Item=&'a Self>) -> Result<(usize,usize),CommonLengthError> where Self : 'a;
}

impl<A:AnyCommonLength+Len> TryCommonLength for A {
    fn try_common_length<'a>(iter:impl ExactSizeIterator<Item=&'a Self>) -> Result<(usize,usize),CommonLengthError> where Self : 'a {
        <A as AnyCommonLength>::any_common_length(iter)
    }
}