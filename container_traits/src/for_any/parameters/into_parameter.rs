pub trait IntoParameter<F> {
    fn into_parameter(self) -> F; 
}

impl<F> IntoParameter<F> for F {
    fn into_parameter(self) -> F {
        self
    }
}