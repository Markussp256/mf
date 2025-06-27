use std::{iter::{Chain, Flatten, Once, Repeat, Take}, num::NonZeroUsize};

#[derive(Clone, Debug)]
pub struct VarStep<I:Iterator, ISteps:Iterator<Item=NonZeroUsize>> {
    iter:I,
    steps:ISteps
}

impl<T,I:Iterator<Item=T>, ISteps:Iterator<Item=NonZeroUsize>> Iterator for VarStep<I,ISteps> {
    type Item=T;
    fn next(&mut self) -> Option<Self::Item> {
        let nsteps=self.steps.next()?;
        let mut res=None;
        for _ in 0..nsteps.into() {
            res=self.iter.next();
        }
        res
    }
}

impl<I:Iterator, ISteps:Iterator<Item=NonZeroUsize>> VarStep<I,ISteps> {
    pub fn new(iter:I, steps:ISteps) -> Self {
        Self{iter, steps}
    }
}

impl<T,I:Iterator<Item=T>> VarStep<I,Flatten<Repeat<Chain<Take<Repeat<NonZeroUsize>>,Once<NonZeroUsize>>>>> {
    pub fn take_a_skip_b(iter:I, a:usize, b:usize) -> Self {
        let steps_a=std::iter::repeat(NonZeroUsize::new(1usize).unwrap()).take(a);
        let steps=steps_a.chain(std::iter::once(NonZeroUsize::new(b+1).unwrap()));
        let steps=std::iter::repeat(steps).flatten();
        Self::new(iter, steps)
    }
} 