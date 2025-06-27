// could be implemented from IntoIterator and Len

use super::NumberOfDegreesOfFreedom;

pub trait TryIntoArray<T> : Sized + NumberOfDegreesOfFreedom<T> + IntoIterator<Item=T> {
    fn try_into_array<const N:usize>(self) -> Result<[T;N],Self> {
        if self.ndofs() == N {
            Ok(utils::iter::next_chunk(& mut self.into_iter()).ok().unwrap())
        } else {
            Err(self)
        }
    }
}

impl<T,S:Sized+NumberOfDegreesOfFreedom<T>+IntoIterator<Item=T>> TryIntoArray<T> for S {}


// impl<T,const M:usize> TryIntoArray for [T;M] {
//     impl_try_into_array!(|_|M);
// }