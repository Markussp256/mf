
use crate::ContainerConstructError;

pub trait TryAccept<Index,T,Error=ContainerConstructError<Index>> {
    fn try_accept<'a>(f:impl Fn(Index) -> &'a T) -> Result<(),Error> where T: 'a;
}

// the error is infallible but we use LinearContainerConstructError to have the same error type
// as the other traits used in containertryconstruct

impl<T,const N:usize> TryAccept<usize,T> for [T;N] {
    fn try_accept<'a>(_:impl Fn(usize) -> &'a T) -> Result<(),ContainerConstructError<usize>> where T:'a {
        Ok(())
    }
}