// basis of a finite-dimensional Vectorspace

use algebra::EnhancedArray;
use algebra_traits::{FiniteDimensionalInnerProductSpace, Scalar, Tolerance};

use container_traits::{Inner, Parameter};

use num_traits::Zero;
use super::{SubSpace,SubSpaceDyn};

#[derive(Clone, Debug)]
pub struct Basis<F, V, const DIM:usize>(SubSpace<F, V, DIM>);

impl<F, V:FiniteDimensionalInnerProductSpace<F,DIM>, const DIM:usize> Basis<F, V, DIM> {
    pub fn basis(&self) -> &[V;DIM] {
        self.0
            .basis()
            .inner()
    }
}

impl<F, V:FiniteDimensionalInnerProductSpace<F,DIM>, const DIM:usize, S:Into<SubSpace<F,V,DIM>>> From<S> for Basis<F, V, DIM> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl<F, V:FiniteDimensionalInnerProductSpace<F,DIM>, const DIM:usize> TryFrom<SubSpaceDyn<F,V>> for Basis<F, V, DIM> {
    type Error = SubSpaceDyn<F,V>;
    fn try_from(value: SubSpaceDyn<F,V>) -> Result<Self,SubSpaceDyn<F,V>> {
        value.try_into()
             .map(|s|Self(s))
    }
}

impl<F:Clone+Scalar,
     const DIM:usize,
    V:Clone+FiniteDimensionalInnerProductSpace<F,DIM>+Tolerance> Basis<F, V, DIM>
    where V::ScProdT : Clone+Zero+Parameter<F>,
          V::DistT : PartialOrd { // +Scalarproduct<Output=SP>+Clone+Distance<SignedOutput = DS>, SP:Zero+Parameters1<F>
    pub fn try_new(vs:Vec<V>) -> Option<(Self,EnhancedArray<usize,DIM>)> {
        let (ss,ind)=SubSpaceDyn::new(vs);
        let ss=ss.try_into().ok()?;
        let ind=ind.try_into().ok()?;
        Some((ss,ind))
    }

    pub fn linear_combination(self, ws:[F;DIM]) -> V {
        self.0
            .linear_combination(ws)
    }

    pub fn find_coordinates(self, v:V) -> [F;DIM] {
        self.0
            .find_coordinates_of_projection(v)
    }
}