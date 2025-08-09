type U2=(usize,usize);


pub trait MatrixConstruct
    : super::MatrixTryConstruct
     +container_traits::ContainerConstruct<U2,crate::MatrixConstructError>
{
    // we can not simply use ClosedMap trait because we would get conflicting implementation for nalgebra's 1x1 SMatrix 
    fn map_rows(self, f:impl Fn(Self::Row) -> Self::Row) -> Self {
        Self::try_from_rows(self.into_rows().map(f)).unwrap()
    }

    fn map_cols(self, f:impl Fn(Self::Col) -> Self::Col) -> Self {
        Self::try_from_cols(self.into_cols().map(f)).unwrap()
    }

}