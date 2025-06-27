use super::{from_dvec, into_dvec, FiniteDifference};

use container_traits::{for_dynamic::Len, AnyParameters, ContainerConstructError, IntoParameters};

use algebra_traits::Scalar;
use algebra::VectorDyn;

use matrix::MatrixDyn;
use matrix_traits::MatrixTryConstruct;

pub fn jacobian_dvec<F:Scalar>(
    f: impl Fn(VectorDyn<F>) -> VectorDyn<F>,
    x0: VectorDyn<F>,
    fin_diff: FiniteDifference<F>,
) -> MatrixDyn<F> {
    let fx0 = f(x0.clone());
    let f=|i:usize,dpi:F|{
        let mut x = x0.clone();
        x[i] += dpi;
        f(x)
    };
    let cols=(0..(x0.len())).map(|i|fin_diff.apply(|dpi:F|f(i,dpi), &fx0).into());
    MatrixDyn::try_from_cols(cols).unwrap()
}

pub fn jacobian<F : Scalar,
                X : Clone+AnyParameters<F,ContainerConstructError<usize>>,
                Y : Clone+IntoParameters<F>>(
    f: impl Fn(X) -> Y,
    x0: X,
    fin_diff: FiniteDifference<F>,
) -> MatrixDyn<F> {
    // construct a corresponding function on VectorDyn
    let f = |dvec: VectorDyn<F>| into_dvec::<F,Y>(f(from_dvec::<F,X>(dvec)));
    let dvec=into_dvec::<F,X>(x0);
    jacobian_dvec(f, dvec, fin_diff)
}

// pub fn uncertainties<
//     F : Scalar,
//     X : Clone+Iter<F>+AnyFromIterator<F,ContainerConstructError<usize>>,
//     Y : Clone+Iter<F>>(
//     f: & impl Fn(&X) -> Y,
//     x0: &X,
//     fin_diff: FiniteDifference<F>,
// ) -> Option<Vec<Nonnegative<F::RealType>>> {
//     uncertainties_from_jacobian(jacobian(f, x0, fin_diff))
// }

// fn uncertainties_from_jacobian<F:Scalar>(m:MatrixDyn<F>) -> Option<Vec<Nonnegative<F::RealType>>> {
//     Some(m.try_pseudo_inverse()?
//           .into_rows()
//           .into_iter()
//           .map(|r|r.norm())
//           .collect())
// }

#[cfg(test)]
use algebra_traits::TryMaxNormOfEntries;

#[cfg(test)]
use matrix_traits::TryMatrixVectorProduct;

#[test]
fn test_jacobian_id() {
    use algebra_traits::TrySub;
    use algebra::Vector3;
    use matrix_traits::MatrixConstruct;
    let f = |x: Vector3<f64>| x.clone();
    let jac = jacobian(f, Vector3::from([1.0, 2.3, 4.5]), FiniteDifference::default());
    assert!(jac.try_sub(MatrixDyn::<f64>::identity(3)).unwrap()
               .try_max_norm_of_entries().unwrap() < 1e-6);
}

#[test]
fn test_jacobian_lin() {
    use algebra::{Vector2,Vector3};
    use matrix::Matrix;
    use matrix_traits::TryFromMatrix;
    let m:Matrix<f64,2,3> = matrix::matrix![1.0,2.0,3.0;4.0,5.0,6.0];
    let f = |x: Vector3<f64>|  m.clone().try_matrix_vector_product::<Vector2<f64>>(x).unwrap();
    let jac=jacobian(f, Vector3::<f64>::from([1.0, 2.3, 4.5]), FiniteDifference::default());
    let jac = Matrix::<f64,2,3>::try_from_matrix(jac).ok().unwrap();
    assert!((jac - m).try_max_norm_of_entries().unwrap() < 1e-6);
}
