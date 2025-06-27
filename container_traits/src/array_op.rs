use utils::iter::next_chunk;

use num_traits::{Zero, One};

use std::ops::{Neg, Add, Sub, Mul, Div};

// we could implement own traits for the array, however we can not implements foreign traits for array (orphan rule)
// we make free functions for all traits instead  


pub fn try_into_subarray<T, const M: usize, const N: usize>(
    arr: [T; M],
    start: usize,
) -> Option<[T; N]> {
    arr.into_iter()
       .skip(start)
       .collect::<Vec<T>>()
       .try_into()
       .ok()
}

pub fn try_concat<T, const M: usize, const N: usize, const K: usize>(
    a: [T; M],
    b: [T; N],
) -> Option<[T; K]> {
        a.into_iter()
         .chain(b.into_iter())
         .collect::<Vec<T>>()
         .try_into()
         .ok()
}

pub fn without<T, const N:usize, const N1:usize>(a:[T;N], i:usize) -> Option<[T;N1]> {
    if i >= N {
        return None;
    }
    let mut a:Vec<T>=a.into_iter().collect();
    a.remove(i);
    let mut aiter=a.into_iter();
    next_chunk(& mut aiter).ok()
}

pub fn try_into_element<T, const N:usize>(a:[T;N], i:usize, rep:T) -> T {
    let mut a_mut=a;
    std::mem::replace(&mut a_mut[i], rep)
}

pub fn put_at<T:Zero, const N:usize>(i:usize, t:T) -> [T;N] {
    let z=std::iter::repeat_with(T::zero);
    let mut iter=z.take(i)
                                           .chain(std::iter::once(t))
                                           .chain(z.take(N-1-i));
    next_chunk(&mut iter).ok().unwrap()
}

pub fn enumerate<T, const N:usize>(a:[T;N]) -> [(usize, T);N] {
    let mut iter=a.into_iter()
                                                    .enumerate();
    next_chunk(& mut iter).ok().unwrap()
}

pub fn neg<T:Neg<Output=TR>,TR, const N:usize>(a:[T;N]) -> [TR;N] {
    a.map(|ai|-ai)
}

pub fn apply_binary_op<T,T2,TR, const N:usize>(a:[T;N], b:[T2;N], f:impl Fn((T,T2)) -> TR) -> [TR;N] {
    next_chunk(& mut a.into_iter()
                            .zip(b.into_iter())
                            .map(f)).ok().unwrap()
}

pub fn add<T:Add<T2,Output=TR>,T2,TR, const N:usize>(a:[T;N], b:[T2;N]) -> [TR;N] {
    apply_binary_op(a, b, |(ai,bi)|ai+bi)
}

pub fn sub<T:Sub<T2,Output=TR>,T2,TR, const N:usize>(a:[T;N], b:[T2;N]) -> [TR;N] {
    apply_binary_op(a, b, |(ai,bi)|ai-bi)
}

pub fn mul<T:Mul<T2,Output=TR>, T2:Clone, TR, const N:usize>(a:[T;N], b:T2) -> [TR;N] {
    a.map(|t|t * b.clone())
}

pub fn div<T:Div<T2,Output=TR>, T2:Clone, TR, const N:usize>(a:[T;N], b:T2) -> [TR;N] {
    a.map(|t: T|t / b.clone())
}

pub fn add3<T:Add<Output=T>,const N:usize>(a0:[T;N], a1:[T;N], a2:[T;N]) -> [T;N] {
    add(add(a0,a1),a2)
}

pub fn mul_pre<T:Mul<T2,Output=T2>+Clone, T2, const N:usize>(b:T, a:[T2;N]) -> [T2;N] {
    a.map(|t|b.clone() * t)
}

pub fn sum<T:Zero, const N:usize>(a:[T;N]) -> T {
    a.into_iter()
     .fold(T::zero(), |acc, new|acc+new)
}

pub fn product<T:One, const N:usize>(a:[T;N]) -> T {
    a.into_iter()
     .fold(T::one(), |acc, new|acc*new)
}

pub fn row_col_mul<T:Mul<T2,Output=TR>,T2,TR:Zero, const N:usize>(r:[T;N], c:[T2;N]) -> TR {
    sum(apply_binary_op(r, c, |(ri,ci)|ri*ci))
}

pub fn cross_product<
    T:Clone+Mul<T2, Output=T3>,
    T2:Clone,
    T3:Sub<Output=TR>,
    TR
>(
    a: impl Into<[T; 3]>,
    b: impl Into<[T2;3]>,
) -> [TR;3] {
    let a:[T;3]=a.into();
    let b:[T2;3]=b.into();
    std::array::from_fn(|i|{
        let i1=(i+1) % 3;
        let i2=(i+2) % 3;
        a[i1].clone()*b[i2].clone()
       -a[i2].clone()*b[i1].clone()})
}