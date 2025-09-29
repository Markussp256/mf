use std::iter::{Chain, Flatten, Once, Repeat, Take};

// iter contains the main Iterator
// skips contains info how many elem are skipped

#[derive(Clone, Debug)]
pub struct VarStep<I:Iterator, ISkips:Iterator<Item=usize>> {
    iter:I,
    skips:ISkips
}

impl<I:Iterator, ISkips:Iterator<Item=usize>> VarStep<I,ISkips> {
    pub fn new(iter:I, skips:ISkips) -> Self {
        Self{iter, skips}
    }
}

impl<T,I:Iterator<Item=T>, ISkips:Iterator<Item=usize>> Iterator for VarStep<I,ISkips> {
    type Item=T;
    fn next(&mut self) -> Option<Self::Item> {
        let nskips=self.skips.next()?;
        for _ in 0..nskips {
            self.iter.next();
        }
        self.iter.next()
    }
}



impl<T,I:Iterator<Item=T>> VarStep<I,Flatten<Repeat<Chain<Take<Repeat<usize>>,Once<usize>>>>> {
    pub fn take_a_skip_b(iter:I, a:usize, b:usize) -> Self {
        let skips_a=std::iter::repeat(0).take(a);
        let skips=skips_a.chain(std::iter::once(b));
        let skips=std::iter::repeat(skips).flatten();
        Self::new(iter, skips)
    }
} 