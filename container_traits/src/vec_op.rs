use std::ops::{Div, Mul};

use num_traits::{Zero, One};

pub fn map<T,T2>(a:Vec<T>, f:impl Fn(T) -> T2) -> Vec<T2> {
    a.into_iter()
     .map(f)
     .collect()
}

pub fn from_fn<T>(f:impl Fn(usize) -> T, len:usize) -> Vec<T> {
    (0..len).into_iter()
            .map(f)
            .collect()
}

// if vector has length less than start+len this function
// will just take the remaining ones and not have length len
// use try_into_subvec if you want a vec of length len and
// None otherwise
pub fn into_subvec<T>(v:Vec<T>, start:usize, len:usize) -> Vec<T> {
    v.into_iter()
     .skip(start)
     .take(len)
     .collect()
}

pub fn try_into_subvec<T>(v:Vec<T>, start:usize, len:usize) -> Option<Vec<T>> {
    (start+len <= v.len()).then(||
        into_subvec(v, start, len))
}

pub fn split<T>(v:Vec<T>,index:usize) -> (Vec<T>,Vec<T>) {
    let mut v=v;
    let vb=v.split_off(index);
    (v,vb)
}

pub fn crop<T>(v:Vec<T>, len:usize) -> Vec<T> {
    let mut v=v;
    v.truncate(len);
    v
}

pub fn put_at<T:Zero>(i:usize, len:usize, t:T) -> Vec<T> {
    let z=std::iter::repeat_with(T::zero);
    z.take(i)
     .chain(std::iter::once(t))
     .chain(z.take(len-1-i))
     .collect()
}

pub fn enumerate<T>(a:Vec<T>) -> Vec<(usize, T)> {
    a.into_iter()
     .enumerate()
     .collect()
}

pub fn rev<T>(a:Vec<T>) -> Vec<T> {
    a.into_iter()
     .rev()
     .collect()
}

pub fn pad_zeros<T:Zero>(vs:& mut Vec<T>, len:usize) -> Option<()> {
    (len >= vs.len()).then(||
        vs.extend(std::iter::repeat_with(T::zero)
                                .take(len-vs.len()))
    )
}

pub fn try_binary_operation<T,T2,TR>(a:Vec<T>, b:Vec<T2>, f: impl Fn((T,T2)) -> TR) -> Option<Vec<TR>> {
    (a.len() == b.len()).then(||
        a.into_iter()
         .zip(b.into_iter())
         .map(f)
         .collect())
}

pub fn sum<T:Zero>(a:Vec<T>) -> T {
    a.into_iter()
     .fold(T::zero(), |acc, new|acc+new)
}

pub fn product<T:One>(a:Vec<T>) -> T {
    a.into_iter()
     .fold(T::one(), |acc, new|acc*new)
}

pub fn try_row_col_mul<T:Mul<T2,Output=TR>,T2,TR:Zero>(r:Vec<T>, c:Vec<T2>) -> Option<TR> {
    try_binary_operation(r, c, |(ri,ci)|ri*ci)
            .map(|v|sum(v))
}

pub fn mul<T:Mul<T2,Output=T3>, T2:Clone, T3>(vs:Vec<T>,fac:T2) -> Vec<T3> {
    vs.into_iter()
      .map(|t|t * fac.clone())
      .collect()
}

pub fn div<T:Div<T2,Output=T3>, T2:Clone, T3>(vs:Vec<T>, fac:T2) -> Vec<T3> {
    vs.into_iter()
      .map(|t|t / fac.clone())
      .collect()
}

pub fn mul_pre<T:Clone+Mul<T2,Output=T3>,T2, T3>(fac:T,vs:Vec<T2>) -> Vec<T3> {
    map(vs,|t|fac.clone()*t)
}

pub fn mulf64_pre<T:Mul<Output=T>+From<f64>>(f:f64, vs:Vec<T>) -> Vec<T> {
    map(vs, |vi|T::from(f)*vi)
}

