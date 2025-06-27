use super::{FromParameter,IntoParameter};

pub trait Parameter<F> : IntoParameter<F>
                        +FromParameter<F> {}

impl<F,
     T : IntoParameter<F>
        +FromParameter<F>> Parameter<F> for T {}

pub fn check_x<F,X:Parameter<F>+Clone+PartialEq>(x:X) {
    if x.clone() != X::from_parameter(x.into_parameter()) {
        panic!("IntoParameter is not inverse of FromParameter");
    }
}

pub fn check_f<F:Clone+PartialEq,X:Parameter<F>>(f:F) {
    if f.clone() != X::from_parameter(f).into_parameter() {
        panic!("IntoParameter is not inverse of FromParameter");
    }
}