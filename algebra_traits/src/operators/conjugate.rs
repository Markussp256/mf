
use container_traits::{ItemT, Iter, Map, Size};
use container::{Concatenated,Cropped};

pub trait Conjugate {
    type Output;

    // required
    fn into_conjugate(self) -> Self::Output;

    fn are_conjugates(&self, rhs:&Self::Output) -> bool;

    // provided
    fn conjugate(&self) -> Self::Output where Self : Clone {
        self.clone()
            .into_conjugate()
    }
}


#[cfg(feature = "num_support")]
impl<T:Clone+std::ops::Neg<Output=T>+PartialEq> Conjugate for num::Complex<T> {
    type Output = Self;
    fn conjugate(&self) -> Self {
        Self{re: self.re.clone(),
             im:-self.im.clone()}
    }

    fn into_conjugate(self) -> Self {
        Self{re: self.re,
             im:-self.im}
    }

    fn are_conjugates(&self,rhs:&Self) -> bool {
        self.re ==  rhs.re &&
        self.im == -rhs.im.clone()
    }
}

impl<T:Conjugate<Output=S>,S> Conjugate for Vec<T> {
    type Output = Vec<S>;

    fn into_conjugate(self) -> Self::Output {
        self.into_iter()
            .map(|t|t.into_conjugate())
            .collect()
    }

    fn are_conjugates(&self,rhs:&Vec<S>) -> bool {
        self.len() == rhs.len() &&
        self.iter()
            .zip(rhs.iter())
            .all(|(l,r)|l.are_conjugates(r))
    }
}

impl<T:Conjugate<Output=T2>,T2,const N:usize> Conjugate for [T;N] {
    type Output = [T2;N];

    fn into_conjugate(self) -> [T2;N] {
        self.map(Conjugate::into_conjugate)
    }

    fn are_conjugates(&self,rhs:&[T2;N]) -> bool {
        self.iter()
            .zip(rhs.iter())
            .all(|(l,r)|l.are_conjugates(r))
    }
}

impl<A:Conjugate<Output=A2>,A2,
     B:Conjugate<Output=B2>,B2> Conjugate for Concatenated<A,B> {
    type Output = Concatenated<A2,B2>;

    fn into_conjugate(self) -> Self::Output {
        let (a,b)=self.into_parts();
        Self::Output::new(
            a.into_conjugate(),
            b.into_conjugate())
    }

    fn are_conjugates(&self, rhs:&Self::Output) -> bool {
        self.a().are_conjugates(rhs.a()) &&
        self.b().are_conjugates(rhs.b())
    }
}

impl<Index : PartialEq,
     A:Clone+ItemT<T=T>+Map<T,T2,Output=A2>,
     A2,
     T:Conjugate<Output=T2>,
     T2> Conjugate for Cropped<Index,A>
        where Self            : Size<Index>+Iter<T>,
            Cropped<Index,A2> : Size<Index>+Iter<T2>   {
    type Output = Cropped<Index,A2>;

    fn into_conjugate(self) -> Self::Output {
        self.map(Conjugate::into_conjugate)
    }

    fn are_conjugates(&self, rhs:&Self::Output) -> bool {
        self.size() == rhs.size()  &&
        self.iter()
            .zip(rhs.iter())
            .all(|(l,r)|l.are_conjugates(r))
    }
}