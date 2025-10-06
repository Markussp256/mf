use std::ops::Sub;
use crate::{ConstElement, ScalarMul, Torsor};
use super::{Basis, LinearAffineCoordinates, Vectorspace};
use utils::iter::ChainExactSize;

pub trait AffineBasis<F> : Sized+ConstElement+Torsor
    where <Self as Sub>::Output : Basis<F> {
    fn affine_basis() -> impl ExactSizeIterator<Item=Self> {
        std::iter::once(Self::ELEMENT)
            .chain_exact_size(
                <Self as Sub>::Output::basis()
                    .map(|v|Self::ELEMENT+v)
            )
    }
}

pub trait TryAffineCombination : ConstElement+Torsor {
    fn try_affine_combination<F>(
        ac   : impl LinearAffineCoordinates<T=F>,
        iter : impl IntoIterator<Item=Self>) -> Option<Self> where <Self as Sub>::Output : Vectorspace<F> {
        let vs:Vec<Self>=iter.into_iter().collect();
        (ac.len() == vs.len()).then(||
        // note len can not be zero because AffineCoordinates can not have len 0
        vs.into_iter()
          .zip(ac.into_iterator())
          .map(|(v,f)|(v.sub(Self::ELEMENT)).scalar_mul(&f))
          .fold(Self::ELEMENT,|acc,new|acc.add(new)))
    }
}
impl<A:ConstElement+Torsor> TryAffineCombination for A {}

impl<F,
     V : Basis<F>,
     A : ConstElement+Torsor+Sub<Output=V>> AffineBasis<F> for A {}