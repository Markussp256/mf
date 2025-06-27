use crate::{matrix::*, Complex, UnitVectorDyn};

use algebra_traits::{ComplexNumber, ConjugateTranspose, Det, Norm, RealNumber, Tolerance, TryMul, TryNormalize, TrySub};

use crate::matrix::matrix_decompositions_dyn::HouseholderDyn;

use utils::IntoThis;

// currently only supported for normal matrices
// so that eigenvectors build orthonormal basis

macro_rules! eig {
    ($name:ident, $fn_name:ident, $stmname:ident, $tr:ident, $init_ew:expr) => {
        pub struct $name<T:$tr> {
            q:$stmname<T>,
            d:DiagonalMatrixDyn<T>
        }
        
        impl<T:$tr> $name<T> {
            pub fn q(&self) -> &$stmname<T> {
                &self.q
            }
        
            pub fn d(&self) -> &DiagonalMatrixDyn<T> {
                &self.d
            }
        
            pub fn into_parts(self) -> ($stmname<T>, DiagonalMatrixDyn<T>) {
                (self.q, self.d)
            }
        
        }

        impl<F:Clone+$tr> $name<F> {

            pub fn new(a:impl NormalMatrix<F>) -> Self {
                let a:SquareMatrixDyn<F>=a.into();
                // let (q,r)=Schur::try_new(a).ok().unwrap().into_parts();
                // let d=DiagonalMatrixDyn::try_from(r.into_square()).ok().unwrap();
                // Self{q,d}
                let n=a.n();
                if n <= 1 {
                    let q=$stmname::identity(n);
                    let d=DiagonalMatrixDyn::<F>::from(a.into_diagonal());
                    return Self{q,d};
                }
                let (ew, ev)=Self::eigen_pair(a.clone()).unwrap();
                let st=HouseholderDyn::froma2b(ev.clone(), UnitVectorDyn::ei(n,0).unwrap()).$fn_name().remove_col(0).0;
                let asubm=st.clone().conjugate_transpose()
                            .try_mul(a.matrix().clone()).unwrap()
                            .try_mul(st.matrix().clone()).unwrap();
        
                let asub=SymmetricMatrixDyn::new_unchecked(asubm);
                let eig_sub=Self::new(asub);
                let m1:MatrixDyn<F>=ev.into_vector().into();
                let m2:MatrixDyn<F>=st.matrix().clone().try_mul(eig_sub.q().matrix().clone()).unwrap();
                let q=$stmname::new_unchecked(MatrixDyn::try_hstack(m1,m2)
                                                          .unwrap());
                let mut d=eig_sub.d().diagonal().clone();
                d.insert(0, ew);
                let d=DiagonalMatrixDyn::from(d);
                Self{q,d}
            }
        
        
            // last element is orthogonal complement to eigenvector
            pub fn eigen_pair(m:SquareMatrixDyn<F>) -> Option<(F, UnitVectorDyn<F>)> {
                // rayleight quotient iteration
                let n=m.n();
                let mut ev=UnitVectorDyn::ei(n, 0).unwrap();
                // choose complex first initial eigenvalue approximation to break symmetry. 
                let mut ew=$init_ew*m.clone().max_norm_of_entries().into_signed();
                let mut iter=0;
                while iter < 10 {
                    let m_shift=m.clone().try_sub(DiagonalMatrixDyn::from_element(n, ew.clone())).unwrap();
                    let qr=SquareQRDyn::from(m_shift);
                    if qr.r().is_singular() {
                        ev=qr.r().try_solve_homogeneous().unwrap();
                        return Some((ew,ev));
                    } else {
                        ev=qr.try_solve(ev.into_vector()).unwrap()
                             .try_normalize().unwrap();
                        if qr.into_matrix()
                             .try_mul(ev.vector().clone()).unwrap()
                             .norm()
                             .is_small() {
                            return Some((ew,ev));
                        }
                    }
        
                    // let osol=qr.try_solve(ev);
                    // h=HouseholderDyn::froma2b(e0.clone(), ev.clone())
                    //                 .map(|hh|hh.orthogonal() )
                    //                 .unwrap_or(OrthogonalOrUnitaryMatrixDyn::<F>::identity(n))
                    //                 .remove_col(0).0;
                    // let ht=h.matrix().clone().conjugate_transpose();
                    // let htm=ht.try_mul(m.matrix().clone()).unwrap();
                    // let hess=SquareMatrixDyn::new_unchecked(htm.clone().try_mul(h.matrix().clone()).unwrap());
                    // let grad=htm.try_mul(ev.vector().clone()).unwrap();
                    // let sol=hess.try_solve(grad).unwrap();
                    // let vnew=VectorDyn::<F>::from(ev).try_add(h.clone().try_mul(sol).unwrap()).unwrap();
                    // ev=vnew.try_normalize().unwrap();
                    ew=Self::rayleight_quotient(&m, ev.clone()).unwrap();
                    iter+=1;
                }
                None
            }
        
        
            pub fn rayleight_quotient(m:&SquareMatrixDyn<F>, v:UnitVectorDyn<F>) -> Option<F> {
                let v=v.vector().clone();
                let selfv=m.clone().try_mul(v.clone())?;
                v.try_scalar_product(selfv)
            }
        }
        
        
        impl<T:$tr> $name<T> {
            pub fn apply_fn(self, f:impl Fn(T) -> T ) -> Self {
                // let q=self.q().clone();
                // let d=self.d()
                //           .clone()
                //           .map_diagonal(f);
                Self{q:self.q,
                     d:self.d.map_diagonal(f)}
            }
        
            pub fn try_apply_fn(self, f:impl Fn(T) -> Option<T>) -> Option<Self> {
                let res=self.d.map_diagonal(f);
                res.vec()
                   .iter()
                   .all(Option::is_some)
                   .then(||
                        Self{q:self.q,
                              d:res.map(Option::unwrap)})
            }
        }
        
        impl<T:$tr+Clone> $name<T> {
            pub fn into_matrix(self) -> MatrixDyn<T> {
                self.q().matrix().clone()
                    .try_mul(self.d().clone().into_this::<MatrixDyn<T>>()).unwrap()
                    .try_mul(self.q().matrix().clone().conjugate_transpose()).unwrap()    
            }
        }
        
        impl<T:$tr+Clone> Det for $name<T> {
            type Output=T;
            fn det(self) -> T {
                self.d()
                    .clone()
                    .det()
            }
        }
    };
}
eig!(EigDyn, unitary, UnitaryMatrixDyn, ComplexNumber, F::i());
eig!(EigRealDyn, orthogonal, OrthogonalMatrixDyn, RealNumber, F::one());



#[test]
fn test_eig() {
    use crate::matrix::{MatrixDyn,SkewSymmetricMatrixDyn};
    use crate::c64;
    use algebra_traits::Tolerance;
    let v:f64=1.0;
    let m:MatrixDyn<f64>=crate::matrix![0.0, v;-v, 0.0].into();
    let skew=SkewSymmetricMatrixDyn::<f64>::try_from(m).unwrap();
    let eig=EigDyn::<c64>::new(skew);
   
    let (_q, d)= eig.into_parts();
    let es=d.diagonal().clone();
    let s=es[0].imag().signum()*v.signum();
    assert!(es[0].is_close_to(c64::new(0.0, s*v)));
    assert!(es[1].is_close_to(c64::new(0.0,-s*v)));
}


impl<R:RealNumber, M:Into<OrthogonalMatrixDyn<R>>> From<M> for EigDyn<Complex<R>> {
    fn from(m: M) -> Self {
        let m=UnitaryMatrixDyn::from_orthogonal(m.into());
        Self::new(m)
    }
}


// impl<C:ComplexNumber> EigDyn<C> {
//     pub fn try_into_real(self) -> Option<EigDyn<C::RealType>> {
//         let (q,d)=self.into_parts();
//         let qm=q.into_matrix();
//         let qimag=qm.clone().map(|z|z.into_imag());
//         if !qimag.max_norm_of_entries().is_small() {
//             return None;
//         }
//         let q: SpecialUnitaryMatrixDyn<C::RealType>=SpecialUnitaryMatrixDyn::new_unchecked(qm.map(|z| z.into_real()));
//         // d
//         let dm:MatrixDyn<C>=d.into();
//         let dimag=dm.clone().map(|z|z.into_imag());
//         if !dimag.max_norm_of_entries().is_small() {
//             return None;
//         }
//         let d: DiagonalMatrixDyn<C::RealType>=DiagonalMatrixDyn::from(dm.map(|z|z.into_real()).into_diagonal());
//         Some(EigDyn{q,d})
//     }
// }

impl<F:ComplexNumber> From<SkewSymmetricMatrixDyn<F::RealType>> for EigDyn<F> {
    fn from(m: SkewSymmetricMatrixDyn<F::RealType>) -> Self {
        Self::new(m)
    }
}

// impl<F:ComplexNumber> From<OrthogonalMatrixDyn<F::RealType>> for EigDyn<F> {
//     fn from(m: OrthogonalMatrixDyn<F::RealType>) -> Self {
//         Self::new(m)
//     }
// }
// utils::from_via!(impl<F:Scalar> From<SpecialOrthogonalMatrixDyn<F::RealType>> for EigDyn<F>, via OrthogonalMatrixDyn<F::RealType>);


// utils::from_via!(impl<F> From<SpecialUnitaryMatrixDyn<F>> for EigDyn<F>, via UnitaryMatrixDyn<F>);

// fn wilkinson_shift<F:Clone+ComplexNumber>(m:&SquareMatrixDyn<F>) -> F {   
//     let n=m.n();
//     assert!(n >= 2);
//     // restrict now to 2x2 submatrix
//     let s=m.matrix().try_submatrix(n-2, 2, n-2, 2).unwrap();
//     let trace=s[(0,0)].clone()+s[(1,1)].clone();
//     let det=s[(0,0)].clone()*s[(1,1)].clone()
//               -s[(0,1)].clone()*s[(1,0)].clone();
//     let sqrt_discrs:[F;2]=(trace.clone().pow2()-det.muli(4)).nth_roots(2).try_into().ok().unwrap();
//     let lambdas=sqrt_discrs.map(|sqrt_discr|(trace.clone()+sqrt_discr).div2());
//     let d=s[(1,1)].clone();
//     let ds=lambdas.clone().map(|lda|lda.distance(d.clone()).into_signed());
//     if ds[0] < ds[1] {
//         lambdas[0].clone()
//     } else {
//         lambdas[1].clone()
//     }
// }

