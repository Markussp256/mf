use num_traits::Zero;
use crate::container::{Container, ContainerConstructError, TryFromSuperContainer};

pub trait FromContainer<Index,C2:Container<Index>,E=ContainerConstructError<Index>> : Container<Index> {
    fn from_container(c2:C2) -> Self;
}

impl<Index:Zero,E,C:TryFromSuperContainer<Index,C2,E>, C2:Container<Index>> FromContainer<Index,C2,E> for C {
    fn from_container(c2:C2) -> Self {
        let size=c2.size();
        Self::try_from_super(c2,Index::zero(),size).ok().unwrap()
    }
}

pub trait IntoContainer<Index,C2:Container<Index>,E=ContainerConstructError<Index>> : Container<Index> {
    fn into_container(self) -> C2;
}

impl<Index, E, C:Container<Index>,C2:FromContainer<Index,C,E>> IntoContainer<Index,C2,E> for C {
    fn into_container(self) -> C2 {
        C2::from_container(self)
    }
}