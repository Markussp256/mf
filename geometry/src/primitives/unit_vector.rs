use crate::Vector;

// #[derive(Clone, Copy, Debug,
//          derive_more::Index,
//          algebra_derive::Neg)]
// pub struct UnitVector<T, const N:usize>(Vector<T,N>);

pub type UnitVector<T,const N:usize>=algebra::Unit<Vector<T,N>>;
pub type UnitVector2<T>=UnitVector<T,2>;
pub type UnitVector3<T>=UnitVector<T,3>;

// impl<V:Clone, NormT:One+Tolerance, const N:usize> TryFrom<Vector<V,N>> for UnitVector<V, N> 
// where Vector<V,N> : Norm<NormT=NormT>,
//        NormT::DistT : PartialOrd {
//     type Error=Vector<V,N>;
//     fn try_from(value: Vector<V,N>) -> Result<Self, Self::Error> {
//         if value.clone().norm().is_close_to_one() {
//             Ok(Self(value))
//         } else {
//             Err(value)
//         }
//     }
// }

// utils::try_from_via!(impl<T, const N:usize> TryFrom<VectorDyn<T>> for UnitVector<T, N>, via Vector<T,N> where UnitVector<T, N> : TryFrom<Vector<T, N>>);

// impl<V, const N:usize> From<algebra::UnitVector<V, N>> for UnitVector<V, N> {
//     fn from(value: algebra::UnitVector<V, N>) -> Self {
//         let vector:algebra::Vector<V,N>=value.into();
//         Self(vector.into())
//     }
// }

// utils::from_via!(impl<T, const N:usize> From<UnitVector<T, N>> for algebra::UnitVectorDyn<T>, via algebra::UnitVector<T, N>);