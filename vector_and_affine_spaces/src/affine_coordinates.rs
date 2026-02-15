use algebra::{EnhancedArray,EnhancedVec};
use algebra_traits::Scalar;

use container_traits::{for_dynamic, for_static, TryAccept, AnyFromIterator, AnyFromParameters, AnyLenFromORef, AnyStandardBasis, ContainerConstructError, ContainerIndex, IndexOutOfBoundsError, IntoIter, IntoParameters, IntoSum, LenTooSmallError, LinearContainerAnyConstruct, TryFromVec, TryMap};
use container_traits::LinearContainerConstructError as LCCE;
use container_traits::ContainerConstructError as CCE;
use num_traits::{Zero,One};
use utils::iter::ChainExactSize;


#[derive(Clone,
         Debug,
         container_derive::JustContainer,
         container_derive::IntoInner,
         container_derive::Inner,
         container_derive::StandardBasis
         // derive_more::Into
)]
pub struct AffineCoordinatesGen<C>(C);

pub type AffineCoordinates<F,const N:usize> = AffineCoordinatesGen<EnhancedArray<F,N>>;
pub type AffineCoordinatesDyn<F>            = AffineCoordinatesGen<EnhancedVec<F>>;


impl<C> AffineCoordinatesGen<C> {
   pub fn try_new<F : Scalar, I : ContainerIndex>(c:C) -> Result<Self,CCE<I>> where C : Clone+IntoIter<F> {
      if c.clone().into_iterator().into_sum().is_close_to_one() {
         Ok(Self(c))
      } else {
         Err(CCE::<I>::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer)
      }
   }
}

impl<F,C : IntoIter<F>> IntoParameters<F> for AffineCoordinatesGen<C> {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=F> {
        self.0
            .into_iterator()
            .skip(1)
    }
}

impl<F : Clone+Scalar,
     C : Clone+AnyLenFromORef+AnyFromIterator<F,LCCE>+IntoIter<F>> AnyFromParameters<F,LCCE> for AffineCoordinatesGen<C> {
   fn any_take_away<I : Iterator<Item=F>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,LCCE> {
      let oc=oref.map(|s|&s.0);
      let len=AnyLenFromORef::any_len_from_oref(oc);
      assert!(len > 0);
      let vs=utils::iter::next_chunk_dyn(iter, len-1)
            .map_err(|v|LenTooSmallError::new(len-1,v.len()))?;
      let i0=F::one()-vs.clone().into_sum();
      let c=<C as AnyFromIterator<F,LCCE>>::any_from_iter(oc,std::iter::once(i0).chain_exact_size(vs.into_iter()))?;
      Self::try_new(c)
   }

   container_traits::any_from_parameters_impl!(F);
}

impl<F : Clone+Scalar,
     C : Clone+for_static::Len+for_static::TryFromIterator<F,LCCE>+IntoIter<F>> for_static::TryFromParameters<F,LCCE> for AffineCoordinatesGen<C> {
    fn try_take_away<I : Iterator<Item=F>>(iter:& mut I) -> Result<Self,LCCE> {
      let len=<Self as for_static::Len>::LEN;
      assert!(len > 0);
      let vs=utils::iter::next_chunk_dyn(iter, len-1)
            .map_err(|v|LenTooSmallError::new(len-1,v.len()))?;
      let i0=F::one()-vs.clone().into_sum();
      let c=<C as for_static::TryFromIterator<F,LCCE>>::try_from_iter(std::iter::once(i0).chain_exact_size(vs.into_iter()))?;
      Self::try_new(c)
    }

   container_traits::try_from_parameters_impl!(F);
}


impl<C : AnyStandardBasis> AffineCoordinatesGen<C> {

}

impl<F:Zero+One, const N:usize> AffineCoordinates<F,N> {
   pub fn try_ei(index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
      <Self as container_traits::for_static::StandardBasis>::try_standard_basis_element(index)
   }
}

impl<F:Zero+One> AffineCoordinatesDyn<F> {
   pub fn try_ei(len:usize, index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
      <Self as container_traits::for_dynamic::StandardBasis>::try_standard_basis_element(len,index)
   }
}

impl<F : Clone+Scalar,
     C : Clone+for_dynamic::TryFromFn<I, F>+IntoIter<F>,
     I : ContainerIndex> for_dynamic::TryFromFn<I, F> for AffineCoordinatesGen<C> {
   fn try_from_fn(size:I, f:impl Fn(I) -> F) -> Result<Self,ContainerConstructError<I>> {
      let c=<C as for_dynamic::TryFromFn<I,F>>::try_from_fn(size, f)?;
      Self::try_new(c)
   }
}

impl<F : Clone+Scalar,
     C : TryAccept<I,F>,
     I : ContainerIndex> TryAccept<I, F> for AffineCoordinatesGen<C> {
   fn try_accept<'a>(size:I,f:impl Fn(I) -> &'a F) -> Result<(),ContainerConstructError<I>> where F: 'a {
      if size.clone()
             .index_iterator()
             .map(&f)
             .cloned()
             .into_sum()
             .is_close_to_one() {
          <C as TryAccept<I,F>>::try_accept(size, f)
      } else {
          Err(ContainerConstructError::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer)
      }
   }
}

impl<F : Clone+Scalar,
     C : Clone+TryFromVec<F,LCCE>+IntoIter<F>> TryFromVec<F,LCCE> for AffineCoordinatesGen<C> {
   fn try_from_vec(vs:Vec<F>) -> Result<Self, LCCE> {
      let c=<C as TryFromVec<F,LCCE>>::try_from_vec(vs)?;
      Self::try_new(c)
   }
}

impl<F   : Clone+Scalar,
     C   : Clone+AnyFromIterator<F,LCCE>+IntoIter<F>> AnyFromIterator<F,LCCE> for AffineCoordinatesGen<C> {
   fn any_take_away<I:Iterator<Item=F>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,LCCE> {
      let c=<C as AnyFromIterator<F,LCCE>>::any_take_away(oref.map(|r|&r.0), iter)?;
      Self::try_new(c)
   }
   container_traits::any_from_iter_impl!(F);
}

impl<F   : Clone+Scalar,
     C   : Clone+for_static::TryFromIterator<F,LCCE>+IntoIter<F>> for_static::TryFromIterator<F,LCCE> for AffineCoordinatesGen<C> {
   fn try_take_away<I:Iterator<Item=F>>(iter:& mut I) -> Result<Self,LCCE> {
      let c=<C as for_static::TryFromIterator<F,LCCE>>::try_take_away(iter)?;
      Self::try_new(c)
   }
   container_traits::try_from_iter_impl!(F);
}

impl<F  : Clone+Scalar,
     F2 : Clone+Scalar,
     C  : TryMap<F,F2,LCCE,Output=C2>,
     C2 : Clone+IntoIter<F2>> TryMap<F,F2,LCCE> for AffineCoordinatesGen<C> {
   type Output=AffineCoordinatesGen<C2>;
   fn try_map(self, f:impl Fn(F) -> F2) -> Result<Self::Output,LCCE> {
      let c2=self.0
                     .try_map(f)?;
      Self::Output::try_new(c2)
   }
}

impl<F : Clone+Scalar,
     C : Clone+LinearContainerAnyConstruct<T=F>+AnyStandardBasis> algebra_traits::AffineCoordinates<usize> for AffineCoordinatesGen<C> {
   fn any_ei(len:usize, index:usize) -> Result<Self,IndexOutOfBoundsError<usize>> {
      <Self as container_traits::AnyStandardBasis>::any_standard_basis_element(len,index)
   }
}