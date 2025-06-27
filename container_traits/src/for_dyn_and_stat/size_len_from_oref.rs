
use super::{OCTSize,Size};
use core::fmt::Debug;
pub trait SizeFromORef<Index> {
    fn size_from_oref(oref:Option<&Self>) -> Index;
}

impl<S:OCTSize<Index>+Size<Index>,Index:Debug+PartialEq> SizeFromORef<Index> for S {
    fn size_from_oref(oref:Option<&Self>) -> Index {
        match (oref,<Self as OCTSize<Index>>::OCTSIZE) {
            (Some(r),Some(sz)) => { assert_eq!(r.size(), sz); sz },
            (Some(r),None) => r.size(),
            (None, Some(sz)) => sz,
            (None,None) => { panic!("Neither compile nor runtime size provided") }
        }
    }
}

pub trait LenFromORef : SizeFromORef<usize> {
    fn len_from_oref(oref:Option<&Self>) -> usize {
        Self::size_from_oref(oref)
    }
}

impl<S:SizeFromORef<usize>> LenFromORef for S {}