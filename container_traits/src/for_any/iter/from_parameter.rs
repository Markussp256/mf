pub trait FromParameter<F> {
    fn from_parameter(f:F) -> Self;
}

impl<F> FromParameter<F> for F {
    fn from_parameter(f:F) -> Self {
        f
    }
}