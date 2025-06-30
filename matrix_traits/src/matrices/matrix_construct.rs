type U2=(usize,usize);

pub trait MatrixConstruct : super::MatrixTryConstruct
                           +container_traits::ContainerConstruct<U2,crate::MatrixConstructError>
                           +container_traits::ClosedMap<Self::Row>
                           +container_traits::ClosedMap<Self::Col>
                            {}