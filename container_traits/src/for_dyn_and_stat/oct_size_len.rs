// optional compile time size
use std::fmt::Debug;

pub trait OCTSize<Index> {
    const OCTSIZE:Option<Index>;

    fn check(sz:&Index) where Index : Debug+PartialEq {
        if let Some(ct_sz)=&Self::OCTSIZE {
            assert_eq!(sz,ct_sz);
        }
    }
}

impl<T> OCTSize<usize> for Vec<T> {
    const OCTSIZE:Option<usize>=None;
}

impl<T,const N:usize> OCTSize<usize> for [T;N] {
    const OCTSIZE:Option<usize> = Some(N);
}

pub trait OCTLen : OCTSize<usize> {
    const OCTLEN:Option<usize>;
}

impl<S:OCTSize<usize>> OCTLen for S {
    const OCTLEN:Option<usize> = <S as OCTSize<usize>>::OCTSIZE;
}

impl<Index,S> OCTSize<(usize,Index)> for Vec<S> {
    const OCTSIZE: Option<(usize, Index)> = None;
}

impl<Index:Copy+PartialEq,S:OCTSize<Index>, const N:usize> OCTSize<(usize,Index)> for [S;N] {
    // note: we can not use map because its not const
    const OCTSIZE: Option<(usize, Index)> =
        match S::OCTSIZE {
            Some(size) => Some((N, size)),
            None => None,
        };
}