use algebra_traits::{ComplexNumber, ConjugateTranspose, MulI, Scalar, TryMul, TrySub};

use crate::matrix::{DiagonalMatrixDyn, SpecialStiefelMatrixDyn, SquareMatrixDyn, SquareRightTriangularMatrixDyn};

use super::SquareQRDyn;

#[derive(Clone, Debug)]
pub struct Schur<F:Scalar> {
    q:SpecialStiefelMatrixDyn<F>,
    r:SquareRightTriangularMatrixDyn<F>
}

impl<F:Scalar> Schur<F> {

    pub fn q(&self) -> &SpecialStiefelMatrixDyn<F> {
        &self.q
    }

    pub fn r(&self) -> &SquareRightTriangularMatrixDyn<F> {
        &self.r
    }

    pub fn into_parts(self) -> (SpecialStiefelMatrixDyn<F>, SquareRightTriangularMatrixDyn<F>) {
        (self.q, self.r)
    }
}

fn wilkinson_shift<F:Clone+ComplexNumber>(m:&SquareMatrixDyn<F>) -> F {
    
    let n=m.n();
    assert!(n >= 2);
    // restrict now to 2x2 submatrix
    let s=m.matrix().try_submatrix(n-2, 2, n-2, 2).unwrap();
    let trace=s[(0,0)].clone()+s[(1,1)].clone();
    let det=s[(0,0)].clone()*s[(1,1)].clone()
              -s[(0,1)].clone()*s[(1,0)].clone();
    let sqrt_discrs:[F;2]=(trace.clone().pow2()-det.muli(4)).nth_roots(2).try_into().ok().unwrap();
    let lambdas=sqrt_discrs.map(|sqrt_discr|(trace.clone()+sqrt_discr).div2());
    let d=s[(1,1)].clone();
    let ds=lambdas.clone().map(|lda|lda.distance(d.clone()).into_signed());
    if ds[0] < ds[1] {
        lambdas[0].clone()
    } else {
        lambdas[1].clone()
    }
}


impl<F:Clone+ComplexNumber> Schur<F> {

    pub fn into_matrix(self) -> SquareMatrixDyn<F> {
        self.q
            .square().clone()
            .try_mul(self.r.square().clone()).unwrap()
            .try_mul(self.q.square().clone().conjugate_transpose()).unwrap()
    }

    fn improve_factor(r:SquareMatrixDyn<F>) -> (SpecialStiefelMatrixDyn<F>, F) {
        let shift=wilkinson_shift(&r);
        let d=DiagonalMatrixDyn::from_element(r.n(), shift.clone());
        let r=r.try_sub(d).unwrap();
        (SquareQRDyn::from(r).into_parts().0, shift)
    }

    fn r_from_mq(m:SquareMatrixDyn<F>,
                 q:SpecialStiefelMatrixDyn<F>) -> SquareMatrixDyn<F> {
        q.square().clone()
         .conjugate_transpose()
         .try_mul(m).unwrap()
         .try_mul(q.square().clone()).unwrap()
    }

    fn try_new_with_q(m:SquareMatrixDyn<F>,
                      q:SpecialStiefelMatrixDyn<F>) -> Result<Self, SquareMatrixDyn<F>> {
        let r=Self::r_from_mq(m, q.clone());
        SquareRightTriangularMatrixDyn::try_from(r)
                .map(|r|Self{q,r})
    }


    pub fn try_new(m:SquareMatrixDyn<F>) -> Result<Self, Vec<(SpecialStiefelMatrixDyn<F>, SquareMatrixDyn<F>, F)>> {
        let n=m.n();
        let mut q=SpecialStiefelMatrixDyn::identity(n);
        let mut errs=Vec::new();
        for _ in 0..100 {
            let r=utils::ret_or_err!(Self::try_new_with_q(m.clone(), q.clone()));
            let (imp_fac, shift)=Self::improve_factor(r.clone());
            errs.push((q.clone(), r.clone(), shift));
            q=q.try_mul(imp_fac).unwrap();
        }
        Err(errs)
    }
}


#[test]
fn test_schur_with_skew() {
    use crate::{matrix::{Matrix3, SkewSymmetricMatrix}, c64};
    use algebra_traits::{Parameters, Tolerance};
    let m=SkewSymmetricMatrix::try_from_parameters(vec![0.1, -0.3, 0.1]).unwrap().into_matrix();
    let m:Matrix3<c64>=m.map(c64::from);
    let m_dyn:SquareMatrixDyn<c64>=m.clone().try_into().unwrap();
    match Schur::try_new(m_dyn) {
        Ok(schur) => {
            assert!(DiagonalMatrixDyn::try_from(schur.r().clone().into_square()).is_ok());
            let m_re:Matrix3<c64>=schur.into_matrix().try_into().unwrap();
            let diff=m_re-m;
            assert!(diff.max_norm_of_entries().into_signed().is_small());
        },
        Err(vs) => {
            for (st, r, shift) in vs {
                println!("st: {}", st.matrix());
                println!("r: {}", r.matrix());
                println!("wilkinson shift: {}", shift);
            }
            assert!(false);
        }
    }
}



#[test]
fn test_schur() {
    use crate::{matrix::Matrix2, c64};
    use algebra_traits::Tolerance;
    let m=crate::matrix![0.8,0.6;-0.6,0.8];
    let m:Matrix2<c64>=m.map(c64::from);
    let m_dyn:SquareMatrixDyn<c64>=m.clone().try_into().unwrap();
    match Schur::try_new(m_dyn) {
        Ok(schur) => {
            assert!(DiagonalMatrixDyn::try_from(schur.r().clone().into_square()).is_ok());
            let m_re:Matrix2<c64>=schur.into_matrix().try_into().unwrap();
            let diff=m_re-m;
            assert!(diff.max_norm_of_entries().into_signed().is_small());
        },
        Err(vs) => {
            for (st, r,_) in vs {
                println!("st: {:?}", st);
                println!("r: {:?}", r);
            }
            assert!(false);
        }
    }
}