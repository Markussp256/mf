// direct view to a container


#[derive(Clone, Debug,
    container_derive::ContainerView
    )]
pub struct View<'a,C>(&'a C);