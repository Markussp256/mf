// only use this if its guaranteed that data lives in some multidimensional box

pub trait Size<Index> {
    const SIZE:Index;
}

impl<T,const N:usize> Size<usize> for [T;N] {
    const SIZE:usize=N;
}

pub trait Len : Size<usize> {
    const LEN:usize;
}

impl<S:Size<usize>> Len for S {
    const LEN:usize=<S as Size<usize>>::SIZE;
}