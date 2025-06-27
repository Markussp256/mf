use super::VarStep;


pub trait TakeASkipB : Sized+Iterator {
    fn take_a_skip_b(self, a:usize, b:usize) -> impl Iterator<Item=Self::Item> {
        VarStep::take_a_skip_b(self, a, b)
    }
}
impl<I:Iterator+Sized> TakeASkipB for I {}