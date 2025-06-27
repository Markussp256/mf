use std::ops::Mul;

use crate::{Vector, VectorDyn};
use algebra_traits::{ConstNonZero, Norm, RealNumber, Tolerance, TryDiv, TryNormalize};

#[derive(Clone, Debug, PartialEq,
         algebra_derive::Conjugate,
         algebra_derive::Neg,
         algebra_derive::Scalarproduct,
         container_derive::IntoInner,
         container_derive::Inner,
         container_derive::AsRef,
         container_derive::Get,
         container_derive::StandardBasis,
         container_derive::IntoIterator,
         container_derive::Size,
         container_derive::XYZ,
         derive_more::Index)]
pub struct Unit<C>(C);

pub type UnitVector<T,const N:usize>=Unit<Vector<T,N>>;
pub type UnitVectorDyn<T>=Unit<VectorDyn<T>>;

impl<C:Clone+Norm> Unit<C> where C::NormT : RealNumber {
    pub fn try_new(c:C) -> Result<Self, C> {
        let is_ok=
            c.clone()
             .norm()
             .is_close_to_one();
        if is_ok {
            Ok(Self(c))
        } else {
            Err(c)
        }
    }
}

// get direction from a vector
impl<C : TryNormalize<NormT=NT>,
     NT: Clone+RealNumber> Unit<C> {
    pub fn try_dir<
    V : ConstNonZero + Mul<NT,Output=V>,
    CV: TryDiv<V,Output=C>>(cv:CV) -> Option<(V,Self)> {
        let nz=V::NONZERO;
        V::div_nz_gen(cv)
        .try_divide_by_norm()
        .ok()
        .and_then(|(n,v)|Self::try_new(v).ok().map(|uv|(nz*n,uv)))
    }
}


impl<C:algebra_traits::ScalarMul<F>, F:'static> std::ops::Mul<F> for Unit<C> {
    type Output=C;
    fn mul(self, rhs: F) -> C {
        <C as algebra_traits::ScalarMul<F>>::scalar_mul(self.0, &rhs)
    }
}

// impl<C     : Clone,
//      NormT : num_traits::One+algebra_traits::Tolerance+algebra_traits::Distance<DistT=DistT>,
//      DistT : PartialOrd> TryFrom<C> for Unit<C>
//     where C : algebra_traits::Norm<NormT=NormT> {
//     type Error=C;
//     fn try_from(c: C) -> Result<Self, C> {
//         let norm:NormT=<C as algebra_traits::Norm>::norm(c.clone()).into_signed();
//         if <NormT as algebra_traits::Tolerance>::is_close_to_one(norm) {
//             Ok(Self(c))
//         } else {
//             Err(c)
//         }
//     }
// }

// if its a unitvector it will also be a unitvector when transformed to another type
// we can not implement From because that would also mean to implement From<Self> which is not possible
impl<C:'static> Unit<C> {
    pub fn from_unchecked<C2:'static+Into<C>>(value:Unit<C2>) -> Self {
        Self(value.0.into())
    }

    pub fn into_unchecked<C2:'static>(self) -> Unit<C2> where C:Into<C2> {
        Unit::from_unchecked(self)
    }

    pub fn try_from_unchecked<C2:'static>(value:Unit<C2>) -> Result<Self, Unit<C2>> where C2 : TryInto<C,Error=C2>{
        value.0
             .try_into()
             .map(|c|Unit(c))
             .map_err(|c2|Unit(c2))
    }

    pub fn try_into_unchecked<C2:'static>(self) -> Result<Unit<C2>, Self> where C:TryInto<C2,Error=C> {
        Unit::try_from_unchecked(self)
    }
}


// impl<T:'static,C:[<$ct TryConstruct>] <T=T>+container_traits::IntoVec<T>> TryFrom<Vec<T>> for Unit<C>
//     where Self : TryFrom<C,Error=C> {
//     type Error=Vec<T>;
//     fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
//         let c=<C as [<$ct TryConstruct>]>::[<$ct_lc _try_from_iter>](value)?;
//         let v=[<$name Generic>]::from(c);
//         v.try_into()
//          .map_err(|e:C| <C as container_traits::IntoVec<T>>::into_vec(e))
//     }
// }

// impl<F, const N:usize> TryFrom<[F;N]> for [<Unit $name>]<F,N> where Self : TryFrom<$name<F,N>,Error=$name<F,N>> {
//     type Error=[F;N];
//     fn try_from(value: [F;N]) -> Result<Self, Self::Error> {
//         let v:$name<F,N>=value.into();
//         v.try_into()
//          .map_err(|v:$name<F,N>|v.into())
//     }
// }
// impl<F, const N:usize> TryFrom<[<$name Dyn>]<F>> for [<Unit $name>]<F,N> where Self : TryFrom<$name<F,N>, Error=$name<F,N>> {
//     type Error = [<$name Dyn>]<F>;
//     fn try_from(value: [<$name Dyn>]<F>) -> Result<Self, Self::Error> {
//         let v:$name<F,N>=value.try_into()?;
//         v.try_into()
//          .map_err(|v:$name<F,N>|v.into())
//     }
// }
// impl<F, const N:usize> TryFrom<[<Unit $name Dyn>]<F>> for [<Unit $name>]<F, N> {
//     type Error=[<Unit $name Dyn>]<F>;
//     fn try_from(vs: [<Unit $name Dyn>]<F>) -> Result<Self, Self::Error> {
//         vs.try_into_unchecked()
//     }
// }
// impl<F, const N:usize> From<[<Unit $name>]<F, N>> for [<Unit $name Dyn>]<F>
//     where Self:TryFrom<[<$name Dyn>]<F>> {
//     fn from(value: [<Unit $name>]<F, N>) -> Self {
//         value.into_unchecked()
//     }
// }

// #[macro_export]
// macro_rules! gen_unit_types {
//     ($name:ident, $name_lc:ident, $ct:ident, $ct_lc:ident) => {
//         paste::paste!(

//             #[derive(Clone, Debug, PartialEq,
//                      algebra_derive::Conjugate,
//                      algebra_derive::Neg,
//                      algebra_derive::Scalarproduct,
//                      algebra_derive::TryScalarproduct,
//                      algebra_derive::TryScalarproduct,
//                      container_derive::IntoInner,
//                      container_derive::Get,
//                      container_derive::StandardBasis,
//                      container_derive::IntoIterator,
//                      container_derive::Len,
//                      derive_more::Index)]
//             pub struct Unit<C:'static>(C);

//             impl<C:algebra_traits::ScalarMul<F>, F:'static> std::ops::Mul<F> for Unit<C> {
//                 type Output=C;
//                 fn mul(self, rhs: F) -> C {
//                     <C as algebra_traits::ScalarMul<F>>::scalar_mul(self.0, &rhs)
//                 }
//             }

//             impl<C     : Clone,
//                  NormT : num_traits::One+algebra_traits::Tolerance+algebra_traits::Distance<DistT=DistT>,
//                  DistT : PartialOrd> TryFrom<C> for Unit<C>
//                 where C : algebra_traits::Norm<NormT=NormT> {
//                 type Error=C;
//                 fn try_from(c: C) -> Result<Self, C> {
//                     let norm:NormT=<C as algebra_traits::Norm>::norm(c.clone()).into_signed();
//                     if <NormT as algebra_traits::Tolerance>::is_close_to_one(norm) {
//                         Ok(Self(c))
//                     } else {
//                         Err(c)
//                     }
//                 }
//             }

//             // if its a unitvector it will also be a unitvector when transformed to another type
//             impl<C:'static> Unit<C> {
//                 pub fn from_unchecked<C2:'static+Into<C>>(value:Unit<C2>) -> Self {
//                     Self(value.0.into_unchecked())
//                 }

//                 pub fn into_unchecked<C2:'static>(self) -> Unit<C2> where C:Into<C2> {
//                     Unit::from_unchecked(self)
//                 }

//                 pub fn try_from_unchecked<C2:'static>(value:Unit<C2>) -> Result<Self, Unit<C2>> where C2 : TryInto<C,Error=C2>{
//                     value.0
//                          .try_into_unchecked()
//                          .map(|c|Unit(c))
//                          .map_err(|c2|Unit(c2))
//                 }

//                 pub fn try_into_unchecked<C2:'static>(self) -> Result<Unit<C2>, Self> where C:TryInto<C2,Error=C> {
//                     Unit::try_from_unchecked(self)
//                 }
//             }

//             impl<C> Unit<C> {
//                 pub fn $name_lc(&self) -> &C {
//                     &self.0
//                 }

//                 pub fn [<into_ $name_lc>](self) -> C {
//                     self.0
//                 }
//             }

         
//             pub type [<Unit $name>]<F,const N:usize> = Unit<$crate::EnhancedArray<F,N>>;
//             pub type [<Unit $name Dyn>]<F> =           Unit<$crate::EnhancedVec<F>>;

//             impl<T:'static,C:[<$ct TryConstruct>] <T=T>+container_traits::IntoVec<T>> TryFrom<Vec<T>> for Unit<C>
//                 where Self : TryFrom<C,Error=C> {
//                 type Error=Vec<T>;
//                 fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
//                     let c=<C as [<$ct TryConstruct>]>::[<$ct_lc _try_from_iter>](value)?;
//                     let v=[<$name Generic>]::from(c);
//                     v.try_into()
//                      .map_err(|e:C| <C as container_traits::IntoVec<T>>::into_vec(e))
//                 }
//             }


//             impl<F, const N:usize> TryFrom<[F;N]> for [<Unit $name>]<F,N> where Self : TryFrom<$name<F,N>,Error=$name<F,N>> {
//                 type Error=[F;N];

//                 fn try_from(value: [F;N]) -> Result<Self, Self::Error> {
//                     let v:$name<F,N>=value.into();
//                     v.try_into()
//                      .map_err(|v:$name<F,N>|v.into())
//                 }
//             }
//             impl<F, const N:usize> TryFrom<[<$name Dyn>]<F>> for [<Unit $name>]<F,N> where Self : TryFrom<$name<F,N>, Error=$name<F,N>> {
//                 type Error = [<$name Dyn>]<F>;
//                 fn try_from(value: [<$name Dyn>]<F>) -> Result<Self, Self::Error> {
//                     let v:$name<F,N>=value.try_into()?;
//                     v.try_into()
//                      .map_err(|v:$name<F,N>|v.into())
//                 }
//             }
//             impl<F, const N:usize> TryFrom<[<Unit $name Dyn>]<F>> for [<Unit $name>]<F, N> {
//                 type Error=[<Unit $name Dyn>]<F>;
//                 fn try_from(vs: [<Unit $name Dyn>]<F>) -> Result<Self, Self::Error> {
//                     vs.try_into_unchecked()
//                 }
//             }
//             impl<F, const N:usize> From<[<Unit $name>]<F, N>> for [<Unit $name Dyn>]<F>
//                 where Self:TryFrom<[<$name Dyn>]<F>> {
//                 fn from(value: [<Unit $name>]<F, N>) -> Self {
//                     value.into_unchecked()
//                 }
//             }
//         );
//     };
// }
// gen_unit_types!(Vector, vector, Container, container);






