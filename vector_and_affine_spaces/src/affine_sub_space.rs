use std::{marker::PhantomData, ops::Sub};

use algebra::{EnhancedArray,EnhancedVec};
use algebra_traits::{ConstElement, InnerProductSpace, Scalar, Tolerance, Torsor
};
use container_traits::{AnyLen, AnyParameters, ChangeT, Empty, IntoParameter, LinearContainer, Parameter, Push};
use container_traits::{LinearContainerConstructError as LCCE, LinearContainerDynamic};
use num_traits::Zero;

use crate::{AffineCoordinatesDyn, AffineCoordinatesGen};

use super::AffineCoordinates;

#[derive(Clone, Debug)]
pub struct AffineSubSpaceGen<F, C>(C, PhantomData<F>);

impl<F, C> AffineSubSpaceGen<F, C> {
    fn private_new(vs:C) -> Self {
        Self(vs,PhantomData::<F>)
    }

    pub fn dimension(&self) -> usize where C : AnyLen {
        self.0
            .any_len()
    }

    pub fn basis(&self) -> &C {
        &self.0
    }

    pub fn into_basis(self) -> C {
        self.0
    }
}

impl<F, C:LinearContainer+Push<C::T>> AffineSubSpaceGen<F,C>  {
    fn private_push(&mut self, v:C::T) {
        self.0
            .push(v);
    }
}

impl<F,C:Empty> Empty for AffineSubSpaceGen<F,C> {
    fn empty() -> Self {
        Self::private_new(C::empty())
    }

    fn is_empty(&self) -> bool {
        self.0
            .is_empty()
    }
}

pub type AffineSubSpace   <F,A,const N:usize> = AffineSubSpaceGen<F,EnhancedArray<A,N>>;
pub type AffineSubSpaceDyn<F,A>               = AffineSubSpaceGen<F,EnhancedVec<A>>;

impl<F,A,const N:usize> From<AffineSubSpace<F,A,N>> for AffineSubSpaceDyn<F,A> {
    fn from(value: AffineSubSpace<F,A,N>) -> Self {
        Self::private_new(
            value.0
                .into())
    }
}

impl<F,V,const N:usize> TryFrom<AffineSubSpaceDyn<F,V>> for AffineSubSpace<F,V,N> {
    type Error=AffineSubSpaceDyn<F,V>;
    fn try_from(value: AffineSubSpaceDyn<F,V>) -> Result<Self, Self::Error> {
        if value.dimension() == N {
            Ok(Self::private_new(
            value.0
                 .try_into().ok().unwrap()))
        } else {
            Err(value)
        }
    }
}

impl<F : Scalar,
     V : Clone+InnerProductSpace<F>,
     A : Clone+Sub<Output=V>+Torsor+ConstElement+Tolerance,
     C : Clone+LinearContainer<T=A>+ChangeT<F,Output=CF>,
     CF: Clone+LinearContainer<T=F>> AffineSubSpaceGen<F, C>
    where V::ScProdT               : Clone+Zero+Parameter<F>,
          A::DistT                 : PartialOrd,
          Self                     : Clone,
          AffineCoordinatesGen<CF> : algebra_traits::AffineCoordinates<T=F> {

    pub fn any_find_coordinates_of_projection(&self, v:A) -> AffineCoordinatesGen<CF> {
        // try to find linear combination that adds up to v
        let f=|ws:AffineCoordinatesGen<CF>|{
            let v_appr=self.clone()
                              .any_affine_combination(ws)
                              .unwrap();
            let diff=v_appr-v.clone();
            self.clone()
                .into_v_basis()
                .into_iter()
                .map(|e|e.scalar_product(diff.clone()).into_parameter())
                .collect::<Vec<F>>()
        };

        let ac0=<AffineCoordinatesGen::<CF> as algebra_traits::AffineCoordinates>::any_ei(self.dimension(), 0).unwrap();
        optimization::fsolve(f, ac0, None)
                        .ok().unwrap()
    }

    pub fn project(&self, v:A) -> A {
        let res=self.any_find_coordinates_of_projection(v);
        self.clone()
            .any_affine_combination(res)
            .unwrap()
    }

    pub fn contains(&self, v:A) -> bool {
         self.project(v.clone())
             .is_close_to(v)
            //  .distance(v.clone())
            //  .is_small()
    }

    pub fn any_affine_combination(self, ac:AffineCoordinatesGen<CF>) -> Option<A> {
        A::try_affine_combination(ac, self.into_basis())
    }

    pub fn into_v_basis(self) -> Vec<V> {
        let basis=self.into_basis();
        let len=basis.any_len();
        basis.into_iterator()
             .map(|a|a-A::ELEMENT)
             .filter(|v|!v.is_zero())
             .take(len-1)
             .collect()
    }

}


impl<F  : Clone+Scalar,
     V  : Clone+InnerProductSpace<F>,
     A  : Clone+Sub<Output=V>+Torsor+ConstElement+Tolerance,
     const N:usize> AffineSubSpace<F,A,N>
     where V::ScProdT               : Clone+Zero+Parameter<F>,
           A::DistT                 : PartialOrd {
    pub fn find_coordinates_of_projection(&self, a:A) -> AffineCoordinates<F,N> {
        self.any_find_coordinates_of_projection(a)
    }

    pub fn affine_combination(self, ac:AffineCoordinates<F,N>) -> A {
        self.any_affine_combination(ac).unwrap()
    }
}


impl<F  : Clone+Scalar,
     V  : Clone+InnerProductSpace<F>,
     A  : Clone+Sub<Output=V>+Torsor+ConstElement+Tolerance> AffineSubSpaceDyn<F,A>
     where V::ScProdT               : Clone+Zero+Parameter<F>,
           A::DistT                 : PartialOrd {
    pub fn find_coordinates_of_projection(&self, a:A) -> AffineCoordinatesDyn<F> {
        self.any_find_coordinates_of_projection(a)
    }

    pub fn try_affine_combination(self, ws:AffineCoordinatesDyn<F>) -> Option<A> {
        self.any_affine_combination(ws)
    }
}

impl<F  : Clone+Scalar,
     V  : Clone+InnerProductSpace<F>,
     A  : Clone+Sub<Output=V>+Torsor+ConstElement+Tolerance,
     C  : Clone+LinearContainerDynamic<T=A>
         +ChangeT<usize,Output = CU>
         +ChangeT<F,    Output = CF>,
     CF : Clone+LinearContainerDynamic<T=F>+AnyParameters<F,LCCE>,
     CU : Clone+LinearContainerDynamic<T=usize>> AffineSubSpaceGen<F, C>
    where V::ScProdT               : Zero+Clone+Parameter<F>,
          A::DistT                 : PartialOrd,
          Self                     : Clone,
          AffineCoordinatesGen<CF> : algebra_traits::AffineCoordinates<T=F> {
    pub fn new(mut vs:Vec<A>) -> (Self,CU) {
        let _marker=PhantomData;
        let vs_len=vs.len();
        match vs_len {
            0 => { panic!("this function should not be called with empty Vec"); },
            1 => { return (Self(C::from_vec(vs),_marker), CU::from_vec(vec![0])); },
            _ => {}
        }

        let last=match vs.pop() {
            Some(last) => last,
            None => { return (Self::empty(),CU::empty()); }
        };
        let ind_last=vs.len();
        let (mut s1, mut inds)=Self::new(vs);
        if !s1.contains(last.clone()) {
            s1.private_push(last);
            inds.push(ind_last);
        }
        (s1, inds)
    }

   
}