use algebra_traits::IntoConjugate;

pub trait Transpose {
    type Output;
    fn transpose(&self) -> Self::Output;
}

pub trait IntoTranspose {
    type Output;
    fn into_transpose(self) -> Self::Output;
}

// we can not bound trait with Conjugate because we can not implement for foreign type

pub trait IntoConjugateTranspose : Sized+IntoConjugate+IntoTranspose {
    fn into_conjugate_transpose(self) -> <<Self as IntoTranspose>::Output as IntoConjugate>::Output
    where <Self as IntoTranspose>::Output : IntoConjugate {
        self.into_transpose()
            .into_conjugate()
    }
}
impl<X:IntoConjugate+IntoTranspose> IntoConjugateTranspose for X {}

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
}

impl<C> IntoTranspose for Transposed<C> {
    type Output = C;
    fn into_transpose(self) -> Self::Output {
        self.0
    }
}

// pub trait RowTranspose : RowVector + Transpose where <Self as Transpose>::Output : ColVector<F=Self::F> {}
// pub trait ColTranspose : ColVector + Transpose where <Self as Transpose>::Output : RowVector<F=Self::F> {}

// pub trait MatrixTranspose : Matrix + Transpose where <Self as Transpose>::Output : Matrix<F=Self::F,
//                                                                                           Row=<Self::Col as Transpose>::Output,
//                                                                                           Col=<Self::Row as Transpose>::Output> {}