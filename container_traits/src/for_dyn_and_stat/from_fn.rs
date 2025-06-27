pub trait FromFn<Index,T> : Sized {
    fn from_fn(size:Index, f:impl Fn(Index) -> T) -> Self;
}

impl<T> FromFn<usize,T> for Vec<T> {
    fn from_fn(len:usize, f:impl Fn(usize) -> T) -> Self {
        (0..len).map(f).collect()
    }
}

impl<T,const N:usize> FromFn<usize,T> for [T;N] {
    fn from_fn(_:usize, f:impl Fn(usize) -> T) -> Self {
        std::array::from_fn(f)
    }
}

impl<Index:Clone,T,S:FromFn<Index,T>> FromFn<(usize,Index),T> for Vec<S> {
    fn from_fn((size0,size1):(usize,Index), f:impl Fn((usize,Index)) -> T) -> Self {
        (0..size0).map(|i|S::from_fn(size1.clone(),|index|f((i,index))))
                  .collect()
    }
}

impl<Index:Clone,T,S:FromFn<Index,T>,const N:usize> FromFn<(usize,Index),T> for [S;N] {
    fn from_fn((_,size1):(usize,Index), f:impl Fn((usize,Index)) -> T) -> Self {
        std::array::from_fn(|i|S::from_fn(size1.clone(),|index|f((i,index))))
    }
}

// macro_rules! impl_from_fn {
//     ($t:ty $(, const $n:ident :usize )?) => {
//         impl<Index:Clone+PartialEq,T,S:FromFn<Index,T> $(, const $n:usize)?> FromFn<(usize,Index),T> for $t where Self : Size<(usize,Index)> {
//             fn from_fn(id:InstanceStructureDescriptor<Self,(usize,Index)>, f:impl Fn((usize,Index)) -> T) -> Self {
//                 let size=id.size();
//                 let sub=||id.try_sub().unwrap();
//                 (0..(size.0))
//                     .map(|i|S::from_fn(sub(),|index|f((i,index))))
//                     .collect::<Vec<S>>()
//                     .try_into().ok().unwrap()
//             }
//         }
//     };
// }
// impl_from_fn!(Vec<S>);
// impl_from_fn!([S;N], const N:usize);