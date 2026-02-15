use std::marker::PhantomData;

use algebra::{EnhancedArray, EnhancedVec};
use num_traits::Zero;

use algebra_traits::{InnerProductSpace, Scalar, Tolerance};

use container_traits::{AnyLen, AnyParameters, ChangeT, Empty, IntoParameter, LinearContainer, LinearContainerAnyConstruct, LinearContainerDynamic, Parameter, Push};

use container_traits::LinearContainerConstructError as LCCE;

#[derive(Clone, Debug)]
pub struct SubSpaceGen<F,C>(C, PhantomData<F>);

impl<F, C> SubSpaceGen<F,C> {
    pub fn dimension(&self) -> usize where C : AnyLen {
        self.0
            .any_len()
    }

    pub fn basis(&self) -> &C {
        &self.0
    }

    fn private_new(vs:C) -> Self {
        Self(vs,PhantomData::<F>)
    }
}

impl<F, C:LinearContainer+Push<C::T>> SubSpaceGen<F,C>  {
    fn private_push(&mut self, v:C::T) {
        self.0
            .push(v);
    }
}

impl<F,C:Empty> Empty for SubSpaceGen<F,C> {
    fn empty() -> Self {
        Self::private_new(C::empty())
    }

    fn is_empty(&self) -> bool {
        self.0
            .is_empty()
    }
}


pub type SubSpace   <F,V,const N:usize> = SubSpaceGen<F,EnhancedArray<V,N>>;
pub type SubSpaceDyn<F,V>               = SubSpaceGen<F,EnhancedVec<V>>;

impl<F,V,const N:usize> From<SubSpace<F,V,N>> for SubSpaceDyn<F,V> {
    fn from(value: SubSpace<F,V,N>) -> Self {
        Self::private_new(
        value.0
                .into())
    }
}

impl<F,V,const N:usize> TryFrom<SubSpaceDyn<F,V>> for SubSpace<F,V,N> {
    type Error=SubSpaceDyn<F,V>;
    fn try_from(value: SubSpaceDyn<F,V>) -> Result<Self, Self::Error> {
        if value.dimension() == N {
            Ok(Self::private_new(
            value.0
                 .try_into().ok().unwrap()))
        } else {
            Err(value)
        }
    }
}

impl<F  : Clone+Scalar,
     V  : Clone+InnerProductSpace<F>+Tolerance,
     C  : Clone+LinearContainer<T=V>+ChangeT<F,Output=CF>,
     CF : Clone+LinearContainerAnyConstruct<T=F>+AnyParameters<F,LCCE>> SubSpaceGen<F,C>
    where V::ScProdT : Clone+Zero+Parameter<F>,
          V::DistT   : PartialOrd {

    fn any_find_coordinates_of_projection(self, v:V) -> CF {
        // try to find linear combination that adds up to v

        let f=|ws:CF|{
            let v_appr=self.clone().any_linear_combination(ws).unwrap();
            let diff=v_appr-v.clone();
            self.basis()
                .clone()
                .into_iterator()
                .map(|e|e.scalar_product(diff.clone()).into_parameter())
                .collect::<Vec<F>>()
        };
        let first_guess: CF=CF::any_from_vec(vec![F::zero();self.dimension()]).ok().unwrap();
        // let target:Vec<V::ScProdT>=vec![V::ScProdT::zero();self.dimension()];
        optimization::fsolve(&f, first_guess,None)
            .ok()
            .unwrap()
    }

    pub fn project(self, v:V) -> V {
        let res=self.clone().any_find_coordinates_of_projection(v);
        self.any_linear_combination(res)
            .unwrap()
    }

    pub fn contains(self, v:V) -> bool {
         self.project(v.clone())
             .is_close_to(v.clone())
    }
    
    fn any_linear_combination(self, ws:CF) -> Option<V> {
        (ws.any_len() == self.dimension()).then(||
        V::linear_combination(ws.into_iterator()
                                    .zip(self.0
                                             .clone()
                                             .into_iterator())))
    }
}

impl<F  : Clone+Scalar,
     V  : Clone+InnerProductSpace<F>+Tolerance,
     const N:usize> SubSpace<F,V,N>
    where V::ScProdT : Clone+Zero+Parameter<F>,
          V::DistT   : PartialOrd {
    pub fn find_coordinates_of_projection(self, v:V) -> [F;N] {
        self.any_find_coordinates_of_projection(v).into()
    }

    pub fn linear_combination(self, ws:[F;N]) -> V {
        self.any_linear_combination(ws.into()).unwrap()
    }
}


impl<F  : Clone+Scalar,
     V  : Clone+InnerProductSpace<F>+Tolerance> SubSpaceDyn<F,V>
    where V::ScProdT : Clone+Zero+Parameter<F>,
          V::DistT   : PartialOrd {
    pub fn find_coordinates_of_projection(self, v:V) -> Vec<F> {
        self.any_find_coordinates_of_projection(v).into()
    }

    pub fn try_linear_combination(self, ws:Vec<F>) -> Option<V> {
        self.any_linear_combination(ws.into())
    }
}

impl<F  : Clone+Scalar,
     V  : Clone+InnerProductSpace<F>+Tolerance,
     C  : Clone+LinearContainerDynamic<T=V>
         +ChangeT<usize,Output = CU>
         +ChangeT<F,    Output = CF>,
     CF : Clone+LinearContainerDynamic<T=F>+AnyParameters<F,LCCE>,
     CU : Clone+LinearContainerDynamic<T=usize>> SubSpaceGen<F,C> 
    where V::ScProdT : Clone+Zero+Parameter<F>,
          V::DistT   : PartialOrd {
    pub fn new(mut vs:Vec<V>) -> (Self, CU) {
        let last=match vs.pop() {
            Some(last) => last,
            None => { return (Self::empty(),CU::empty()); }
        };
        let ind_last=vs.len();
        let (mut s1, mut inds)=Self::new(vs);
        if !s1.clone().contains(last.clone()) {
            s1.private_push(last);
            inds.push(ind_last);
        }
        (s1, inds)
    }
}