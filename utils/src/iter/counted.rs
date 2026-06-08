

pub struct Counted<I> {
    iter: I,
    taken: usize,
}

impl<I> Counted<I> {
    pub fn new(iter:I) -> Counted<I> {
        Self{iter,taken:0}
    }

    pub fn taken(&self) -> usize {
        self.taken
    }

    pub fn into_parts(self) -> (I,usize) {
        (self.iter,self.taken)
    }
}

impl<I> Iterator for Counted<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        self.taken += 1;
        Some(item)
    }
}

pub struct CountedMut<'a,I> {
    iter: &'a mut I,
    taken: usize,
}

impl<'a,I> CountedMut<'a,I> {
    pub fn new(iter:&'a mut I) -> Self {
        Self{iter,taken:0}
    }

    pub fn taken(&self) -> usize {
        self.taken
    }

    pub fn into_parts(self) -> (&'a mut I,usize) {
        (self.iter,self.taken)
    }
}

impl<'a,I> Iterator for CountedMut<'a,I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        self.taken += 1;
        Some(item)
    }
}