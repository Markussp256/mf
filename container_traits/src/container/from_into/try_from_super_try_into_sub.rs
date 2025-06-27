use crate::{IntoIndexedIter, container::{Container, ContainerIndex, ContainerTryConstruct, ContainerConstructError}};

use crate::error::SizeTooSmallError;

pub trait TryFromSuperContainer<Index,C2:Container<Index>,E=ContainerConstructError<Index>> : Container<Index> {
    fn try_from_super(c:C2, start:Index, size:Index) -> Result<Self, E>;
}

impl<Index:ContainerIndex,
     T,
     E  : From<SizeTooSmallError<Index>>,
     C  : ContainerTryConstruct<Index, E, T=T>,
     C2 : Container<Index,T=T>> TryFromSuperContainer<Index,C2,E> for C {
    fn try_from_super(c2:C2, start:Index, size:Index) -> Result<Self, E> {
        let required_size=start.clone().elem_wise_add(size.clone());
        let actual_size=c2.size();
        SizeTooSmallError::try_new_ref(&required_size, &actual_size)?;
        let f=|ind:Index|c2.get(start.clone().elem_wise_add(ind)).unwrap();
        Self::try_accept(size.clone(),f)?;
        Self::any_from_iter(None,
            <C2 as IntoIndexedIter<Index,T>>::into_indexed_iter(c2)
            //c2.into_iterator()
              .filter(|(ind,_)|start.is_elem_wise_smaller_eq(ind) &&
                                         ind.is_elem_wise_strictly_smaller(&required_size))
              .map(|(_,t)|t))
    }
}

pub trait TryIntoSubContainer<Index, C2:Container<Index>, E=ContainerConstructError<Index>> : Container<Index> {
    fn try_into_sub(self, start:Index, size:Index) -> Result<C2,E>;
}

impl<Index,E,C:Container<Index>, C2:TryFromSuperContainer<Index, C, E>> TryIntoSubContainer<Index,C2,E> for C {
    fn try_into_sub(self, start:Index, size:Index) -> Result<C2,E> {
        C2::try_from_super(self, start, size)
    }
}