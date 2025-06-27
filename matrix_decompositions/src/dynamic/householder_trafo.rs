use std::ops::Mul;

use algebra_traits::{AdditiveGroup, ComplexNumber, Conjugate, ConjugateTranspose, MulI, RealNumber, Scalar, ScalarMul, TryDiv, TryMul, TryNormalize, TrySub};

use crate::{UnitVectorDyn, VectorDyn};
use crate::matrix::{MatrixColDyn, MatrixDyn, MatrixRowDyn, OrthogonalMatrixDyn, StiefelMatrixDyn, SymmetricMatrixDyn, UnitaryMatrixDyn};

#[derive(Clone, Debug)]
pub struct HouseholderDyn<T>(UnitVectorDyn<T>);

impl<T> HouseholderDyn<T> {
    pub fn new(uv:UnitVectorDyn<T>) -> Self {
        Self(uv)
    }

    pub fn into_inner(self) -> UnitVectorDyn<T> {
        self.0
    }

    pub fn into_col_vector(self) -> MatrixColDyn<T> {
        self.0
            .into_vector()
            .into()
    }

    pub fn into_row_vector(self) -> MatrixRowDyn<T> where T:Conjugate {
        self.into_col_vector()
            .conjugate_transpose()
    }
}

/// finds householder trafo that transforms a to b up to a scalar factor
impl<T:Clone+Scalar> HouseholderDyn<T> {
    pub fn froma2b(a:UnitVectorDyn<T>, b:UnitVectorDyn<T>) -> Self {
        // scalar product must be real for householder formula to work
        let sc_prod=a.vector().clone().try_scalar_product(b.vector().clone()).unwrap();
        let fac=if sc_prod.clone().is_small() {
            T::one()
        } else {
            -sc_prod.try_normalize::<T>().unwrap()
        };
        Self(b.into_vector()
              .try_div(fac).unwrap()
              .try_sub(a.into_vector()).unwrap()
              .try_normalize().unwrap())
    }
}

macro_rules! orthogonal_or_unitary {
    ($name:ident, $tr:ident, $fn_name:ident) => {
        impl<T:$tr+Clone> HouseholderDyn<T> {
            pub fn $fn_name(&self) -> $name<T> {
                let n=self.0.len();
                let vvh=MatrixDyn::from(self.0.vector().clone()).hermite_form();
                let m=SymmetricMatrixDyn::<T>::identity(n).try_sub(vvh.scalar_mul(&T::one().muli(2))).unwrap();
                $name::<T>::from_square_unchecked(m.into_square())
            }
        }
    };
}
orthogonal_or_unitary!(OrthogonalMatrixDyn, RealNumber, orthogonal);
orthogonal_or_unitary!(UnitaryMatrixDyn, ComplexNumber, unitary);


/// builds the first n columns of the matrix
impl<T:Scalar+Clone> HouseholderDyn<T> {
    pub fn stiefel(&self, n:usize) -> StiefelMatrixDyn<T> {
        let m=MatrixDyn::from_fn(|i,j|
            if i==j { T::one() } else { T::zero() }
            -self.0[i].clone()*self.0[j].clone().conjugate().muli(2), (self.0.len(), n));
        StiefelMatrixDyn::new_unchecked(m)
    }
}

#[test]
fn test_froma2b() {
    let e0=UnitVectorDyn::ei(2,0).unwrap();
    let b=UnitVectorDyn::try_from(VectorDyn::from(vec![0.8,0.6])).unwrap();
    let hh=HouseholderDyn::<f64>::froma2b(e0, b);
    println!("{}", hh.orthogonal().matrix());

    let d:MatrixDyn<f64>=hh.orthogonal().into_matrix().try_sub(crate::matrix![-0.8, -0.6;-0.6, 0.8].into()).unwrap();
    assert!(d.max_norm_of_entries() < 1e-8);
}

#[cfg(test)]
use algebra_traits::{Norm, Tolerance};

#[cfg(test)]
fn are_collinear<F:Scalar>(a:VectorDyn<F>, b:VectorDyn<F>) -> bool
where VectorDyn<F> : Norm<NormT=F::RealType> {
    let an=a.clone().norm();
    let bn=b.clone().norm();
    let sp=a.try_scalar_product(b).unwrap().norm();
    Tolerance::is_close_to(sp, an*bn)
}


#[test]
fn test_froma2b_complex() {
    use crate::c64;
    let e0=UnitVectorDyn::<c64>::ei(2,0).unwrap();
    let onedivsqrt2=1.0/2_f64.sqrt();
    let bs=vec![
        UnitVectorDyn::try_from(VectorDyn::from(vec![c64::from(0.8), c64::new(0.0, 0.6)])).unwrap(),
        UnitVectorDyn::try_from(VectorDyn::from(vec![c64::i(), c64::from(0.0)])).unwrap(),
        UnitVectorDyn::try_from(VectorDyn::from(vec![c64::new(0.0, onedivsqrt2), c64::from(onedivsqrt2)])).unwrap(),
    ];
    for b in bs {
        let hh=HouseholderDyn::<c64>::froma2b(e0.clone(), b.clone());
        let bv=b.clone().into_vector();
        let e0v=e0.clone().into_vector();
        let hb=hh.clone().try_mul(bv.clone()).unwrap();
        let he0=hh.clone().try_mul(e0v.clone()).unwrap();
        assert!(are_collinear(hb, e0v));
        assert!(are_collinear(he0, bv));
    }
}


impl<F:Clone+Scalar+Mul<V,Output=V>,
     V:Clone+AdditiveGroup> TryMul<VectorDyn<V>> for HouseholderDyn<F> {
    type Output=VectorDyn<V>;
    fn try_mul(self, rhs:VectorDyn<V>) -> Option<VectorDyn<V>> {
        let sp:V=self.clone().into_row_vector().try_mul(rhs.clone())?;
        let sp2=sp.muli(2);
        rhs.try_sub(self.into_col_vector().map(|v|v*sp2.clone()).into())
    }
}

impl<F, V> TryMul<MatrixColDyn<V>> for HouseholderDyn<F>
where HouseholderDyn<F> : TryMul<VectorDyn<V>, Output=VectorDyn<V>> {
    type Output=MatrixColDyn<V>;
    fn try_mul(self, rhs:MatrixColDyn<V>) -> Option<MatrixColDyn<V>> {
        let rhs:VectorDyn<V>=rhs.into();
        self.try_mul(rhs)
            .map(|v|v.into())
    }
}


impl<F:Clone+Scalar> TryMul<MatrixDyn<F>> for HouseholderDyn<F> {
    type Output=MatrixDyn<F>;
    fn try_mul(self, rhs:MatrixDyn<F>) -> Option<MatrixDyn<F>> {
        (self.0.len() == rhs.nrows()).then(||
            MatrixDyn::try_from_cols(
                rhs.into_cols()
                       .into_iter()
                       .map(|c|self.clone().try_mul(c).unwrap())
                       .collect()).unwrap())
    }
}
