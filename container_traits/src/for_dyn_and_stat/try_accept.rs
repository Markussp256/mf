
use crate::ContainerConstructError;

pub trait TryAccept<Index,T,Error=ContainerConstructError<Index>> : Sized {
    fn try_accept<'a>(size:Index,f:impl Fn(Index) -> &'a T) -> Result<(),Error> where T: 'a;
}

// the error is infallible but we use LinearContainerConstructError to have the same error type
// as the other traits used in containertryconstruct

impl<T> TryAccept<usize,T> for Vec<T> {
    fn try_accept<'a>(_:usize,_: impl Fn(usize) -> &'a T) -> Result<(),ContainerConstructError<usize>> where T:'a {
        Ok(())
    }
}

impl<T,const N:usize> TryAccept<usize,T> for [T;N] {
    fn try_accept<'a>(_:usize,_:impl Fn(usize) -> &'a T) -> Result<(),ContainerConstructError<usize>> where T: 'a {
        Ok(())
    }
}