use num_traits::Zero;
use crate::{container::{Container, TryFromSuperContainer},ContainerConstructError};

pub trait AnyFromContainer<Index,C2:Container<Index>,E=ContainerConstructError<Index>> : Container<Index> {
    fn any_from_container(c2:C2) -> Result<Self,E>;
}

impl<Index:Zero, E, C:TryFromSuperContainer<Index,C2, E>, C2:Container<Index>> AnyFromContainer<Index,C2,E> for C {
    fn any_from_container(c2:C2) -> Result<Self,E> {
        let size=c2.size();
        Self::try_from_super(c2,Index::zero(),size)
    }
}
pub trait AnyIntoContainer<Index,C2:Container<Index>,E=ContainerConstructError<Index>> : Container<Index> {
    fn any_into_container(self) -> Result<C2,E>;
}

impl<Index,E,C:Container<Index>, C2:AnyFromContainer<Index,C,E>> AnyIntoContainer<Index,C2,E> for C {
    fn any_into_container(self) -> Result<C2,E> {
        C2::any_from_container(self)
    }
}