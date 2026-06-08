use crate::{LenNotEqualToRequiredLenError, Rebind, container::{Container,ContainerTryConstruct}};


pub trait ConvertContainer<Index> : Container<Index> {
    fn convert<'a,C2:'a+ContainerTryConstruct<Index,E>,E:From<LenNotEqualToRequiredLenError>>(self) -> C2
    where Self : Rebind<E,With<C2::T>=C2>,
          Self::T : Into<C2::T> {
        C2::any_from_iter(None, self.into_iterator().map(|t|t.into())).ok().unwrap()
    }
}