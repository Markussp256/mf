use std::ops::Mul;
use num_traits::Zero;

use algebra::{Unit, Vector, VectorGeneric};

use container_traits::{FromInner, IntoInner, LinearContainer};
use matrix_traits::{
    try_vector_vector_product_impl,
    try_into_vector_vector_product_impl,
    ColVectorView,
    IntoTranspose,
    IntoVectorVectorProduct,
    RowVectorView,
    Transpose,
    TryVectorVectorProduct,
    TryIntoVectorVectorProduct,
    VectorVectorProduct};

algebra::gen_vector_view!(MatrixColViewGeneric, MatrixColViewDyn, MatrixColView);
algebra::gen_vector_view!(MatrixRowViewGeneric, MatrixRowViewDyn, MatrixRowView);

algebra::gen_vector!(MatrixColGeneric, MatrixColDyn, MatrixCol);
algebra::gen_vector!(MatrixRowGeneric, MatrixRowDyn, MatrixRow);

impl<C:LinearContainer> RowVectorView for MatrixRowViewGeneric<C> {}
impl<C:LinearContainer> ColVectorView for MatrixColViewGeneric<C> {}

impl<C:LinearContainer> RowVectorView for MatrixRowGeneric<C> {}
impl<C:LinearContainer> ColVectorView for MatrixColGeneric<C> {}

pub type UnitMatrixColGeneric<C> = Unit<MatrixColGeneric<C>>;
pub type UnitMatrixRowGeneric<C> = Unit<MatrixRowGeneric<C>>;

pub type UnitMatrixCol<T,const N:usize>=Unit<MatrixCol<T,N>>;
pub type UnitMatrixRow<T,const N:usize>=Unit<MatrixRow<T,N>>;

pub type UnitMatrixColDyn<T>=Unit<MatrixColDyn<T>>;
pub type UnitMatrixRowDyn<T>=Unit<MatrixRowDyn<T>>;

macro_rules! vec_vec_impl {
    ($lhs : ident, $rhs : ident) => {
        impl<F1 : Mul<F2,Output=F3>+Clone, F2:Clone, F3 : Zero,
             C1 : LinearContainer<T=F1>,
             C2 : LinearContainer<T=F2>> TryVectorVectorProduct<$rhs<C2>> for $lhs<C1> {
            type Output=F3;
            fn try_vector_vector_product(&self, rhs:&$rhs<C2>) -> Option<F3> {
                try_vector_vector_product_impl(self,rhs)
            }
        }
        impl<F1 : Mul<F2,Output=F3>, F2, F3 : Zero,
             C1 : LinearContainer<T=F1>,
             C2 : LinearContainer<T=F2>> TryIntoVectorVectorProduct<$rhs<C2>> for $lhs<C1> {
            type Output=F3;
            fn try_into_vector_vector_product(self, rhs:$rhs<C2>) -> Option<F3> {
                try_into_vector_vector_product_impl(self,rhs)
            }
        }
    }
}


vec_vec_impl!(MatrixRowGeneric,MatrixColGeneric);

impl<F1 : Mul<F2,Output=F3>+Clone, F2:Clone, F3 : Zero,
     C1 : LinearContainer<T=F1>,
     C2 : LinearContainer<T=F2>> TryVectorVectorProduct<VectorGeneric<C2>> for MatrixRowGeneric<C1> {
    type Output=F3;
    fn try_vector_vector_product(&self, rhs:&VectorGeneric<C2>) -> Option<F3> {
        try_vector_vector_product_impl(self,rhs)
    }
}

impl<F1 : Mul<F2,Output=F3>+Clone,F2:Clone,F3:Zero, const N:usize> VectorVectorProduct<MatrixCol<F2,N>> for MatrixRow<F1,N> {
    type Output = F3;
    fn vector_vector_product(&self, rhs:&MatrixCol<F2,N>) -> F3 {
        try_vector_vector_product_impl(self,rhs).unwrap()
    }
}

impl<F1 : Mul<F2,Output=F3>+Clone,F2:Clone,F3:Zero, const N:usize> VectorVectorProduct<Vector<F2,N>> for MatrixRow<F1,N> {
    type Output = F3;
    fn vector_vector_product(&self, rhs:&Vector<F2,N>) -> F3 {
        try_vector_vector_product_impl(self,rhs).unwrap()
    }
}

// algebra::gen_unit_types!(MatrixCol, matrixcol, ColVector, colvector);
// algebra::gen_unit_types!(MatrixRow, matrixrow, RowVector, rowvector);

// impl<F,
//      C  : ContainerTryConstruct<T=F>,
//      C2 : ContainerTryConstruct<T=F>,
//      C3 : ContainerTryConstruct<T=MatrixRowGeneric<C>>>
//          BuildMatrix<MatrixColGeneric<C2>> for MatrixRowGeneric<C>
//     where MatrixColGeneric<C3> : ColVectorTryConstruct<T=MatrixRowGeneric<C>> + ChangeT<F, Output=MatrixColGeneric<C2>>,
//           MatrixColGeneric<C2> : ColVectorTryConstruct<T=F> + ChangeT<MatrixRowGeneric<C>, Output=MatrixColGeneric<C3>>,
//           MatrixRowGeneric<C>  : RowVectorTryConstruct<T=F> {
//     type Matrix=MatrixGeneric<MatrixColGeneric<C3>>;
// }


impl<C:'static> From<VectorGeneric<C>> for MatrixColGeneric<C> {
    fn from(value: VectorGeneric<C>) -> Self {
        Self(value.into_inner())
    }
}

impl<C:'static> Into<VectorGeneric<C>> for MatrixColGeneric<C> {
    fn into(self) -> VectorGeneric<C> {
        VectorGeneric::from_inner(self.0)
    }
}


fn test_is_rvtc<F, const N:usize>(a:MatrixRow<F, N>) -> impl matrix_traits::RowVectorTryConstruct {
    a
}

macro_rules! impl_row_col_vector {
    ($name:ident, $lc_name:ident, $other:ident) => {
        paste::paste!(
            impl<C:LinearContainer> Transpose for [<Matrix $name Generic>]<C> {
                type Output=[<Matrix $other Generic>]<C>;
                fn transpose(&self) -> Self::Output where Self : Clone {
                    self.clone()
                        .0
                        .into()
                }

                fn into_transpose(self) -> Self::Output {
                    self.0
                        .into()
                }
            }
        );
    };
}
impl_row_col_vector!(Row,row,Col);
impl_row_col_vector!(Col,col,Row);


#[cfg(test)]
use container_traits::FromVec;

#[test]
fn test_try_sub() {
    use algebra_traits::TrySub;

    let a=MatrixColDyn::<f64>::from_vec(vec![1.0,2.0,3.0]);
    let b=MatrixColDyn::<f64>::from_vec(vec![1.0,2.0,3.0]);
    assert_eq!(a.try_sub(b),Ok(MatrixColDyn::<f64>::from_vec(vec![0.0,0.0,0.0])))
}

#[test]
fn test_try_sub_colvector_of_rowvectors() {
    use algebra_traits::TrySub;
    let a=vec![
     MatrixRowDyn::<f64>::from_vec(vec![1.0,2.0]),
     MatrixRowDyn::<f64>::from_vec(vec![1.0])
    ];
    let b=vec![
     MatrixRowDyn::<f64>::from_vec(vec![1.0,2.0,3.0]),
     MatrixRowDyn::<f64>::from_vec(vec![1.0])
    ];
    let a=MatrixColDyn::from_vec(a);
    let b=MatrixColDyn::from_vec(b);
    assert!(a.try_sub(b).is_err());
}




#[test]
fn test_change_t() {
    use container_traits::ChangeT;
    assert_eq!(std::any::TypeId::of::<<MatrixColDyn<Vec<f64>> as ChangeT<f64>>::Output>(),
               std::any::TypeId::of::<MatrixColDyn<f64>>());
}