pub trait Reverse {
    fn reverse(self) -> Self; 
}

impl<T> Reverse for Vec<T> {
    fn reverse(self) -> Self {
        self.into_iter()
        .rev()
        .collect()
        // let mut s=self;
        // s.as_mut_slice().reverse();
        // s
    }
}

impl<T,const N:usize> Reverse for [T;N] {
    fn reverse(self) -> Self {
        self.into_iter()
            .rev()
            .collect::<Vec<T>>()
            .try_into()
            .ok()
            .unwrap()
    }
}