use algebra_traits::Conjugate;


pub trait Transpose {
    type Output;

    // required

    fn into_transpose(self) -> Self::Output;

    // provided

    fn transpose(&self) -> Self::Output where Self : Clone {
        self.clone()
            .into_transpose()
    }

    fn conjugate_transpose(&self) -> <Self::Output as Conjugate>::Output
    where Self : Clone, <Self as Transpose>::Output : Conjugate {
        self.transpose()
            .into_conjugate()
    }

    fn into_conjugate_transpose(self) -> <Self::Output as Conjugate>::Output
    where Self : Sized,
          Self::Output : Conjugate {
        self.into_transpose()
            .into_conjugate()
    }
}



#[derive(Clone,Debug, PartialEq)]
pub struct Transposed<C>(C);

impl<C> Transposed<C> {
    pub fn new(c:C) -> Self {
        Self(c)
    }
}


impl<C:Clone> Transpose for Transposed<C> {
    type Output=C;
    fn transpose(&self) -> Self::Output {
        self.0
            .clone()
    }

    fn into_transpose(self) -> Self::Output {
        self.0
    }
}