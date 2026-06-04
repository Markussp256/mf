use either::Either;
use container_traits::{IntoIter, Iter};
use generic_array::{ArrayLength,GenericArray};
use crate::{Max, Nonnegative, TryDiv};

pub trait Norm {
    type NormT;
    fn norm(&self) -> Nonnegative<Self::NormT>;
    fn into_norm(self) -> Nonnegative<Self::NormT>;
}


pub trait TryNormalize: Clone + Norm + TryDiv<Self::NormT> where Self::NormT : Clone {

    fn try_divide_by_norm(self) -> Result<(Self::NormT,<Self as TryDiv<Self::NormT>>::Output),<Self as TryDiv<Self::NormT>>::Error> {
        let norm=
            self.norm()
                .into_signed();
        <Self as TryDiv<Self::NormT>>::try_div(self, norm.clone())
            .map(|r|(norm,r))
    }

    fn try_normalize<R>(self) -> Result<(Self::NormT,R),
                                         Either<<Self as TryDiv<Self::NormT>>::Error,
                                               <<Self as TryDiv<Self::NormT>>::Output as TryInto<R>>::Error>>
    where <Self as TryDiv<Self::NormT>>::Output : TryInto<R> {
        let (n,v)=self.try_divide_by_norm()
            .map_err(|e|Either::Left(e))?;
        let r=v.try_into().map_err(|e|Either::Right(e))?;
        Ok((n,r))
    }
}
impl<T : TryDiv>               TryNormalize for Vec<T> where Self : Clone+Norm+TryDiv<Self::NormT,Output=Vec<<T as TryDiv>::Output>>, Self::NormT : Clone {}
impl<T : TryDiv,const N:usize> TryNormalize for [T;N]  where Self : Clone+Norm+TryDiv<Self::NormT,Output=[<T as TryDiv>::Output;N]>,  Self::NormT : Clone {}


#[macro_export]
macro_rules! impl_norm_from_squared_norm_generic {
    (for $t:ty where $($tb:tt)*) => {
        impl<$($tb)*, T2, TR> // :num_traits::Zero+$crate::Max
                $crate::Norm for $t
        where Self : $crate::NormSquared<Norm2T=T2>,
        $crate::Nonnegative<T2> : $crate::Sqrt<Output=$crate::Nonnegative<TR>> {
            type NormT=TR;
            fn norm(&self) -> $crate::Nonnegative<Self::NormT> {
                <$crate::Nonnegative<T2> as $crate::Sqrt>::sqrt(
                  <Self as $crate::NormSquared>::norm_squared(self))
            }
            fn into_norm(self) -> $crate::Nonnegative<Self::NormT> {
                <$crate::Nonnegative<T2> as $crate::Sqrt>::sqrt(
                  <Self as $crate::NormSquared>::into_norm_squared(self))
            }
        }
    }
}

#[macro_export]
macro_rules! impl_norm_from_squared_norm {
    ($name:ident <$t:ident $(,$d:ident)*>) => {
        $crate::impl_norm_from_squared_norm_generic!(for $name<$t $(,$d)*> where $t $(,const $d:usize)*);
        $crate::impl_try_normalize!($name<$t $(,$d)*>);
    }
}

#[macro_export]
macro_rules! impl_norm_for_vector_without_normalize {
    ($name:ident <$t:ident  $(,$d:ident)*>) => {
    impl<$t : $crate::TryDiv<Output=F>
    +$crate::ConstNonZero
    +$crate::Norm<NormT=NT>,
    NT:num_traits::Zero
    +$crate::ScalarMul<F::RealType>
    +std::cmp::PartialOrd,
 F : $crate::Scalar
    $(,const $d: usize)*> $crate::Norm for $name<$t $(,$d)*> 
    where  Self             : container_traits::Map<$t,F,Output= $name<F $(,$d)*>>,
       $name<F $(,$d)*> : $crate::NormSquared<Norm2T=F::RealType>,
    // TR::DistT:PartialOrd
{
   type NormT=NT;
   fn norm(&self) -> $crate::Nonnegative<Self::NormT> {
       // dividing vector would require clone therefore we use map
       let scaled=<Self as container_traits::Map<$t,F>>::map(self,<$t as $crate::ConstNonZero>::div_nz);
       let f=<$crate::Nonnegative<F::RealType> as $crate::Sqrt>::sqrt(
           <$name<F $(,$d)*> as $crate::NormSquared>::norm_squared(&scaled));
       let nt=<$t as $crate::ConstNonZero>::NONZERO.norm();
       $crate::Nonnegative::try_new(<NT as $crate::ScalarMul<F::RealType>>::scalar_mul(nt.into_signed(), &f.into_signed())).unwrap()
   }

   fn into_norm(self) -> $crate::Nonnegative<Self::NormT> {
       // dividing vector would require clone therefore we use map
       let scaled=<Self as container_traits::Map<$t,F>>::map(self,<$t as $crate::ConstNonZero>::div_nz);
       let f=<$crate::Nonnegative<F::RealType> as $crate::Sqrt>::sqrt(
           <$name<F $(,$d)*> as $crate::NormSquared>::into_norm_squared(scaled));
       let nt=<$t as $crate::ConstNonZero>::NONZERO.into_norm();
       $crate::Nonnegative::try_new(<NT as $crate::ScalarMul<F::RealType>>::scalar_mul(nt.into_signed(), &f.into_signed())).unwrap()
   }
}



}
}

#[macro_export]
macro_rules! impl_norm_for_vector {
    ($name:ident <$t:ident  $(,$d:ident)*>) => {
        $crate::impl_norm_for_vector_without_normalize!($name <$t $(,$d)*>);
        $crate::impl_try_normalize!($name <$t $(,$d)*>);
    }
}

crate::impl_norm_from_squared_norm_generic!(for [T;N]  where T, const N:usize);
crate::impl_norm_from_squared_norm_generic!(for GenericArray<T,N>  where T, N:ArrayLength);
crate::impl_norm_from_squared_norm_generic!(for Vec<T> where T);

// unfortunately we can not define norm for rowvector and colvector because then the 1x1 vector is defined twice
// hence we define (frobenius) norm generally for matrix which boils down to usual norm for row colvectors



#[test]
fn test_norm() {
    let v=vec![0.8,0.6];
    assert_eq!(v.into_norm(), 1.0)
}

pub trait TryMaxNormOfEntries<T : Norm>
    where <T as Norm>::NormT : Max {
    fn try_max_norm_of_entries(&self) -> Option<Nonnegative<<T as Norm>::NormT>>
        where Self : Iter<T>{
        self.iter()
            .map(|v|v.norm())
            .reduce(Nonnegative::into_max)
    }

    fn try_into_max_norm_of_entries(self) -> Option<Nonnegative<<T as Norm>::NormT>>
        where Self : Sized+IntoIter<T> {
        self.into_iterator()
            .map(|v|v.into_norm())
            .reduce(Nonnegative::into_max)
    }
}

impl<I,
     T     : Norm<NormT=NormT>,
     NormT : Max> TryMaxNormOfEntries<T> for I {}


// #[derive(Clone)]
// struct Foo;


// impl<T:Norm<P=f64,S=f64>> Norm for Vec<T> {

// }
