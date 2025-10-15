
use container_traits::{for_static::TryFromIterator, ItemT, LinearContainerConstructError, Map};
use container::{Concatenated,Cropped};

pub trait Conjugate {
    type Output;
    fn conjugate(&self) -> Self::Output;

    fn are_conjugates(&self,rhs:&Self) -> bool where Self:PartialEq<Self::Output> {
        self == &rhs.conjugate()
    }
}

pub trait IntoConjugate {
    type Output;
    fn into_conjugate(self) -> Self::Output;

    fn are_conjugates(&self,rhs:&Self) -> bool where Self:Clone+PartialEq<Self::Output> {
        self == &rhs.clone().into_conjugate()
    }
}

#[cfg(feature = "num_support")]
impl<T:Clone+std::ops::Neg<Output=T>+PartialEq> Conjugate for num::Complex<T> {
    type Output = Self;
    fn conjugate(&self) -> Self {
        Self{re: self.re.clone(),
             im:-self.im.clone()}
    }
}

#[cfg(feature = "num_support")]
impl<T:std::ops::Neg<Output=T>+PartialEq> IntoConjugate for num::Complex<T> {
    type Output = Self;
    fn into_conjugate(self) -> Self {
        Self{re: self.re,
             im:-self.im}
    }
}

impl<T:Conjugate> Conjugate for Vec<T> {
    type Output = Vec<<T as Conjugate>::Output>;
    fn conjugate(&self) -> Self::Output {
        self.iter()
            .map(Conjugate::conjugate)
            .collect()
    }
}

impl<T:IntoConjugate> IntoConjugate for Vec<T> {
    type Output = Vec<<T as IntoConjugate>::Output>;
    fn into_conjugate(self) -> Self::Output {
        self.into_iter()
            .map(|t|t.into_conjugate())
            .collect()
    }
}

impl<T:Conjugate<Output=T2>,T2,const N:usize> Conjugate for [T;N] {
    type Output = [T2;N];
    fn conjugate(&self) -> [T2;N] {
        <[T2;N] as TryFromIterator<T2, LinearContainerConstructError>>::try_from_iter(
            self.iter()
                .map(Conjugate::conjugate))
                .unwrap()
    }
}


impl<T:IntoConjugate<Output=T2>,T2,const N:usize> IntoConjugate for [T;N] {
    type Output = [T2;N];
    fn into_conjugate(self) -> [T2;N] {
        self.map(IntoConjugate::into_conjugate)
    }
}

impl<A:Conjugate<Output=A2>,A2,
     B:Conjugate<Output=B2>,B2> Conjugate for Concatenated<A,B> {
    type Output = Concatenated<A2,B2>;
    fn conjugate(&self) -> Self::Output {
        Self::Output::new(
            self.a().conjugate(),
            self.b().conjugate())
    }
}

impl<A:IntoConjugate<Output=A2>,A2,
     B:IntoConjugate<Output=B2>,B2> IntoConjugate for Concatenated<A,B> {
    type Output = Concatenated<A2,B2>;
    fn into_conjugate(self) -> Self::Output {
        let (a,b)=self.into_parts();
        Self::Output::new(
            a.into_conjugate(),
            b.into_conjugate())
    }
}

impl<Index : Clone,
     A:Clone+ItemT<T=T>+Map<T,T2,Output=A2>,
     A2,
     T:Conjugate<Output=T2>,
     T2> Conjugate for Cropped<Index,A> {
    type Output = Cropped<Index,A2>;
    fn conjugate(&self) -> Self::Output {
        self.clone()
            .map(|t|t.conjugate())
    }
}

impl<Index,A:ItemT<T=T>+Map<T,T2,Output=A2>,A2,T:IntoConjugate<Output=T2>,T2> IntoConjugate for Cropped<Index,A> {
    type Output = Cropped<Index,A2>;
    fn into_conjugate(self) -> Self::Output {
        self.map(IntoConjugate::into_conjugate)
    }
}