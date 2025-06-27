use crate::{ChangeT, container::{Container,ContainerTryConstruct}};


pub trait ConvertContainer<Index> : Container<Index> {
    fn convert<C2:ContainerTryConstruct<Index,E>,E>(self) -> C2
    where Self:ChangeT<C2::T,Output=C2>, Self::T : Into<C2::T> {
        C2::any_from_iter(None, self.into_iterator().map(|t|t.into())).ok().unwrap()
    }
}