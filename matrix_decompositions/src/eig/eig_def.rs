use container_traits::{ChangeT, ClosedMap, Inner, IntoInner, Map};
use matrix_traits::*;
use algebra_traits::{ComplexNumber, Conjugate, RealNumber, TryDiv, RealAndImag};
use algebra::Complex;
use matrix_wrappers::{Orthogonal, SpecialOrthogonal, SpecialUnitary, Symmetric, Unitary};
use num_traits::One;
use std::ops::Mul;

use crate::eig::eig_impl::EigImpl;

pub trait EigBaseConditions
    : Clone
     +MatrixSquare
     +Transpose<Output=Self>
     +Conjugate<Output=Self>
     +TryMatrixMatrixProduct<Output=Self> {}

impl<EBC
    : Clone
     +MatrixSquare
     +Transpose<Output=EBC>
     +Conjugate<Output=EBC>
     +TryMatrixMatrixProduct<Output=EBC>> EigBaseConditions for EBC {}

macro_rules! def_impl_eig {
    ($eig:ident, $r_or_c:ident, $o_or_u:ident) => {

    pub struct $eig<M:MatrixSquare> where M::T : $r_or_c {
        q:$o_or_u<M>,
        d:DiagonalMatrixGeneric<M::Row>
    }

    impl<F   : $r_or_c,
        Row  : RowVector<T=F>,
        M    : EigBaseConditions<T=F,Row=Row>> $eig<M>
        where DiagonalMatrixGeneric<Row> : TryMatrixMatrixProduct<M,Output=M> {

           pub fn new(q:$o_or_u<M>, d:DiagonalMatrixGeneric<Row>) -> Self { Self{q,d} }
           pub fn q(&self) -> & $o_or_u<M> { &self.q }
           pub fn d(&self) -> &DiagonalMatrixGeneric<Row> { &self.d }
           pub fn into_parts(self) -> ($o_or_u<M>,DiagonalMatrixGeneric<Row>) { (self.q,self.d) }
           pub fn into_matrix(self) -> M {
               let (q,d)=(self.q,self.d);
               let dqh=d.try_matrix_matrix_product(&q.inner().conjugate_transpose()).unwrap();
               q.into_inner().try_matrix_matrix_product(&dqh).unwrap()
            }

            pub fn apply_fn(self, f:impl Fn(F)->F) -> Self
            where Row : RowVectorTryConstruct<T=F>+ClosedMap<F> {
                Self::new(self.q,self.d.map_diagonal(f))
            }

            pub fn try_solve
                <Rhs : ColVectorTryConstruct+ChangeT<Out::T,Output=Out>,
                 Out : ColVectorTryConstruct>(self, rhs:Rhs) -> Option<Out>
                where   F : Mul<Rhs::T,Output=Rhs::T>
                           +Mul<Out::T,Output=Out::T>,
                        Rhs::T : TryDiv<F,Output=Out::T>,
                        M : TryMatrixVectorProduct<Rhs,Output=Rhs>
                           +TryMatrixVectorProduct<Out,Output=Out>
            {
                let qt=self.q.conjugate_transpose();
                let qtr=qt.try_matrix_vector_product(&rhs).ok()?;
                let dinvqtr=Out::try_from_vec(
                    qtr.into_iterator()
                       .zip(self.d.into_diagonal().into_iter())
                       .map(|(rhsi,dii)|rhsi.try_div(dii))
                       .collect::<Result<Vec<Out::T>,_>>().ok()?).ok()?;
                self.q.try_matrix_vector_product(&dinvqtr).ok()
            }
    }
    };
}
def_impl_eig!(EigStructReal, RealNumber,    SpecialOrthogonal);
def_impl_eig!(EigStruct    , ComplexNumber, SpecialUnitary);


impl<R  : RealNumber,
     M  : MatrixTryConstruct<T=R,Row=Row>
         +EigBaseConditions
         +ChangeT<R,Output=M>
         +EigImpl,
     Row : RowVector<T=R>
          +Mul<R,Output=Row>> From<Symmetric<M>> for EigStructReal<M> {
    fn from(value: Symmetric<M>) -> Self {
        let (q,d)=<M as EigImpl>::eig(value.into_inner(),R::zero());
        let q=SpecialOrthogonal::try_new(Orthogonal::from_stiefel(q),R::one()).ok().unwrap();
        Self::new(q,d)
    }
}

impl<R     : RealNumber,
     M     : MatrixNormal<T=R>+IntoBaseMatrix<Output=MBase>,
     MBase : MatrixViewSquare<T=R>+Map<R,Complex<R>,Output=MC>,
     MC    : MatrixTryConstruct<T=Complex<R>,Row=MCRow>
            +ChangeT<Complex<R>,Output=MC>
            +EigBaseConditions
            +EigImpl,
     MCRow : RowVector<T=Complex<R>>
            +Mul<Complex<R>,Output=MCRow>> From<M> for EigStruct<MC> {
    fn from(value: M) -> Self {
        let mc:MC=value.into_base_matrix()
                       .map(|r|Complex::new(r,R::zero()));
        // choose nonreal initial eigenvalue to break symmetry and actuall find complex eigenvalues
        let init_ew=Complex::new(R::zero(),R::one());
        let (q,d)=mc.eig(init_ew);
        let q=SpecialUnitary::try_new(Unitary::from_stiefel(q),Complex::<R>::one()).ok().unwrap();
        Self::new(q,d)
    }
}

// pub trait Eig : MatrixNormal
//     where Self::T : Zero+TryDiv,
//     <Self::T as TryDiv>::Output : Mul<Self::T,Output=Self::T> {
//     type Q : AlgebraMatrix<T=<Self::T as TryDiv>::Output>;//  + AnyMatrixMatrixProduct<Self::R,Output=Self>
//     fn eig(self) -> (Self::Q, DiagonalMatrixGeneric<Self::Row>);

//     fn try_solve
//     <Rhs : ColVectorTryConstruct+ChangeT<Out::T,Output=Out>,
//      Out : ColVectorTryConstruct>(self, rhs:Rhs) -> Option<Out>
//     where   <Self::T as TryDiv>::Output : Mul<Rhs::T,Output=Rhs::T>+Mul<Out::T,Output=Out::T>,
//             Rhs::T : TryDiv<Self::T,Output=Out::T>,
//             Self::Q : Clone+Transpose<Output = Self::Q> + TryMatrixVectorProduct<Rhs,Output=Rhs>+TryMatrixVectorProduct<Out,Output=Out>
//             {
//         let (q,d)=self.eig();
//         let qt=q.clone().conjugate_transpose();
//         let qtr=qt.try_matrix_vector_product(rhs)?;
//         let dinvqtr=Out::try_from_vec(
//             qtr.into_iterator()
//                .zip(d. into_diagonal().into_iter())
//                .map(|(rhsi,dii)|rhsi.try_div(dii))
//                .collect::<Result<Vec<Out::T>,_>>().ok()?).ok()?;
//         q.try_matrix_vector_product(dinvqtr)
//     }
// }