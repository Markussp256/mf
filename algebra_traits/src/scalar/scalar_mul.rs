use generic_array::{ArrayLength,GenericArray};

macro_rules! impl_arr_vec {
    ($tr:ident, $fn:ident) => {
        impl<T:$tr<F>, F, const N:usize> $tr<F> for [T;N] {
            fn $fn(self, f:&F) -> Self {
                self.map(|vi|vi.$fn(f))
            }
        }

        impl<T:$tr<F>, F, N:ArrayLength> $tr<F> for GenericArray<T,N> {
            fn $fn(self, f:&F) -> Self {
                Self::try_from_iter(
                    self.into_iter()
                        .map(|vi|vi.$fn(f))
                ).unwrap()
            }
        }

        impl<T:$tr<F>, F> $tr<F> for Vec<T> {
            fn $fn(self, f:&F) -> Self {
                self.into_iter()
                    .map(|vi|vi.$fn(f))
                    .collect()
            }
        }
    };
}

pub trait ScalarMul<Field> {
    fn scalar_mul(self, f:&Field) -> Self;
}
impl_arr_vec!(ScalarMul,scalar_mul);


// impl<T:TryDiv<Output=F>+Mul<F,Output=T>, F> ScalarMul for T {
//     type Field=F;
// }

pub trait ScalarDiv<Field> {
    fn scalar_div(self, f:&Field) -> Self;
}
impl_arr_vec!(ScalarDiv,scalar_div);

// impl <T:Div<Output=F>+ Div<F, Output=T>, F> ScalarDiv for T {
//     type Field=F;
// }


pub trait TryScalarDiv<Field> : Sized {
    type Error;
    fn is_scalar_divable_by(&self, f:&Field) -> Result<(),Self::Error>;
    fn try_scalar_div(self, f:&Field) -> Result<Self,Self::Error>;
}

impl<T:TryScalarDiv<F,Error=E>, E, F:Clone, N:ArrayLength> TryScalarDiv<F> for GenericArray<T,N>  {
    type Error=E;

    fn is_scalar_divable_by(&self, f:&F) -> Result<(),E> {
        for t in self {
            t.is_scalar_divable_by(f)?
        }
        Ok(())
    }

    fn try_scalar_div(self, f:&F) -> Result<Self,E> {
        self.is_scalar_divable_by(f)?;
        Ok(Self::try_from_iter(
            self.into_iter()
                .map(|t|t.try_scalar_div(f).ok().unwrap())).unwrap())
    }
}

impl<T:TryScalarDiv<F,Error=E>, E, F:Clone, const N:usize> TryScalarDiv<F> for [T;N]  {
    type Error=E;

    fn is_scalar_divable_by(&self, f:&F) -> Result<(),E> {
        for t in self {
            t.is_scalar_divable_by(f)?
        }
        Ok(())
    }

    fn try_scalar_div(self, f:&F) -> Result<Self,E> {
        self.is_scalar_divable_by(f)?;
        Ok(self.map(|t|t.try_scalar_div(f).ok().unwrap()))
    }
}


impl<T:TryScalarDiv<F,Error=E>, E, F:Clone> TryScalarDiv<F> for Vec<T> {
    type Error=E;

    fn is_scalar_divable_by(&self, f:&F) -> Result<(),E> {
        for t in self {
            t.is_scalar_divable_by(f)?
        }
        Ok(())
    }

    fn try_scalar_div(self, f:&F) -> Result<Self,E> {
        self.into_iter()
            .map(|t|t.try_scalar_div(f))
            .collect()
    }
}


// impl<T:TryDiv<Output=F>+TryDiv<F,Output=T>, F> TryScalarDiv for T {
//     type Field=F;
// }

// use crate::MapScalars;
// use std::ops::Mul;
// pub trait ScalarMul : Sized+MapScalars
// where Self::F : Clone+ Mul<Output=Self::F> {
//     fn scalar_mul(self, f:Self::F) -> Self {
//         self.map_scalars(|v|v * f.clone())
//     }

//     fn pre_scalar_mul(f:Self::F, rhs:Self) -> Self {
//         rhs.map_scalars(|v|f.clone() * v)
//     }
// }