pub mod matrix_col_dyn;
pub mod matrix_col_stat;

pub mod matrix_row_dyn;
pub mod matrix_row_stat;

mod impl_row_col;
mod impl_display;

use algebra::{Unit, VectorGeneric};

use container_traits::{FromInner, IntoInner, LinearContainer};
use matrix_traits::{ColVector, RowVector};

algebra::gen_vector!(MatrixColGeneric, MatrixColDyn, MatrixCol);
algebra::gen_vector!(MatrixRowGeneric, MatrixRowDyn, MatrixRow);

impl<C:LinearContainer> RowVector for MatrixRowGeneric<C> {}
impl<C:LinearContainer> ColVector for MatrixColGeneric<C> {}

pub type UnitMatrixColGeneric<C> = Unit<MatrixColGeneric<C>>;
pub type UnitMatrixRowGeneric<C> = Unit<MatrixRowGeneric<C>>;

pub type UnitMatrixCol<T,const N:usize>=Unit<MatrixCol<T,N>>;
pub type UnitMatrixRow<T,const N:usize>=Unit<MatrixRow<T,N>>;

pub type UnitMatrixColDyn<T>=Unit<MatrixColDyn<T>>;
pub type UnitMatrixRowDyn<T>=Unit<MatrixRowDyn<T>>;

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

// impl<F   : Clone+ComplexNumber,
//      C   : LinearContainerTryConstruct<T=F>+ChangeT<F,Output=C>+ChangeT<MatrixRowGeneric<C>,Output=C2>,
//      C2  : 'static+Clone+Conjugate+PartialEq+LinearContainerTryConstruct<T=MatrixRowGeneric<C>>+ChangeT<MatrixRowGeneric<C>,Output=C2>> HermitianOuterProduct for MatrixColGeneric<C> {
//     type Output=Hermitian<Square<MatrixGeneric<MatrixRowGeneric<C>,MatrixColGeneric<C>>>>;
// }

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