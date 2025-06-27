use num_traits::Zero;

use crate::container::{Container, ContainerConstructError, TryFromSuperContainer};

pub trait TryFromContainer<Index,C2:Container<Index>,E=ContainerConstructError<Index>> : Container<Index> {
    fn try_from_container(c2:C2) -> Result<Self,E>;
}

impl<Index:Zero,
     E,
     C:TryFromSuperContainer<Index,C2,E>,
     C2:Container<Index>> TryFromContainer<Index,C2,E> for C {
    fn try_from_container(c2:C2) -> Result<Self,E> {
        let size=c2.size();
        Self::try_from_super(c2,Index::zero(),size)
    }
}

pub trait TryIntoContainer<Index,C2:Container<Index>,E=ContainerConstructError<Index>> : Container<Index> {
    fn try_into_container(self) -> Result<C2,E>;
}

impl<Index,E,C:Container<Index>, C2:TryFromContainer<Index,C,E>> TryIntoContainer<Index,C2,E> for C {
    fn try_into_container(self) -> Result<C2,E> {
        C2::try_from_container(self)
    }
}