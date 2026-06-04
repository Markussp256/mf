use generic_array::{ArrayLength, GenericArray};

pub trait Enumerate {
    type Output;
    fn enumerate(self) -> Self::Output;
}

impl<T> Enumerate for Vec<T> {
    type Output = Vec<(usize,T)>;
    fn enumerate(self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .collect()
    }
}

impl<T,N:ArrayLength> Enumerate for GenericArray<T,N> {
    type Output = GenericArray<(usize,T),N>;
    fn enumerate(self) -> Self::Output {
        utils::iter::next_chunk_gen_arr(
            & mut self.into_iter()
                      .enumerate()).ok()
                                   .unwrap()
    }
}


impl<T,const N:usize> Enumerate for [T;N] {
    type Output = [(usize,T);N];
    fn enumerate(self) -> Self::Output {
        utils::iter::next_chunk(
            & mut self.into_iter()
                      .enumerate()).ok()
                                   .unwrap()
    }
}