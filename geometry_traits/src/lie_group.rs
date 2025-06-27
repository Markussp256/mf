
use algebra_traits::group::MultiplicativeGroup;
use super::Manifold;

pub trait LieGroup<F,E> : MultiplicativeGroup + Manifold<F,E> {}