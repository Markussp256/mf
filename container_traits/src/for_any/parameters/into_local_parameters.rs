// a type that can be converted to and from an iterator

pub trait IntoLocalParameters<F> {
    fn into_local_parameters(self,rhs:Self) -> impl ExactSizeIterator<Item=F>;
}

#[macro_export]
macro_rules! impl_into_local_parameters_for_multiplicative_group {
    () => {
        fn into_local_parameters(self, rhs: Self) -> impl ExactSizeIterator<Item=F> {
            <Self as container_traits::IntoParameters<F>>::into_parameters(rhs / self)
        }
    };
}


macro_rules! impl_into_local_parameters {
    ($f:ty) => {
        impl IntoLocalParameters<$f> for $f {
            fn into_local_parameters(self,rhs:Self) -> impl ExactSizeIterator<Item=$f> {
                std::iter::once(rhs-self)
            }
        }
    };
}
impl_into_local_parameters!(f64);
impl_into_local_parameters!(f32);
impl_into_local_parameters!(i32);
impl_into_local_parameters!(usize);
