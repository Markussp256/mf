use algebra_traits::Conjugate;

pub trait Transpose {
    type Output;
    fn transpose(self) -> Self::Output;
}

// we can not bound trait with Conjugate because we can not implement for foreign type

pub trait ConjugateTranspose : Sized+Conjugate+Transpose {
    fn conjugate_transpose(self) -> <Self as Transpose>::Output {
        self.conjugate()
            .transpose()
    }
}
impl<X:Conjugate+Transpose> ConjugateTranspose for X {}

#[derive(Clone,Debug, PartialEq)]
pub struct Transposed<C>(C);

impl<C> Transposed<C> {
    pub fn new(c:C) -> Self {
        Self(c)
    }
}


impl<C:Transpose<Output=Transposed<C>>> Transpose for Transposed<C> {
    type Output=C;

    fn transpose(self) -> Self::Output {
        self.0
    }
}

// pub trait RowTranspose : RowVector + Transpose where <Self as Transpose>::Output : ColVector<F=Self::F> {}
// pub trait ColTranspose : ColVector + Transpose where <Self as Transpose>::Output : RowVector<F=Self::F> {}

// pub trait MatrixTranspose : Matrix + Transpose where <Self as Transpose>::Output : Matrix<F=Self::F,
//                                                                                           Row=<Self::Col as Transpose>::Output,
//                                                                                           Col=<Self::Row as Transpose>::Output> {}