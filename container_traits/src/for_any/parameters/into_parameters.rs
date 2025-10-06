// a type that can be converted to and from an iterator

pub trait IntoParameters<F> {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=F>;
}

impl<F> IntoParameters<F> for Vec<F> {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=F> {
        self.into_iter()
    }
}

impl<F, const N:usize> IntoParameters<F> for [F;N] {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=F> {
        self.into_iter()
    }
}

macro_rules! impl_into_parameters {
    ($f:ty) => {
        impl IntoParameters<$f> for $f {
            fn into_parameters(self) -> impl ExactSizeIterator<Item=$f> {
                std::iter::once(self)
            }
        }
    };
}
impl_into_parameters!(f64);
impl_into_parameters!(f32);
impl_into_parameters!(i32);
impl_into_parameters!(usize);
