
use container_traits::{ClosedMap, Concatenated, Cropped, ItemT, Map};

pub trait Conjugate {
    fn conjugate(self) -> Self;

    fn are_conjugates(&self,rhs:&Self) -> bool where Self:Clone+PartialEq {
        self == &rhs.clone().conjugate()
    }
}

#[cfg(feature = "num_support")]
impl<T:std::ops::Neg<Output=T>+PartialEq> Conjugate for num::Complex<T> {
    fn conjugate(self) -> Self {
        Self{re:self.re, im:-self.im}
    }
}

impl<T:Conjugate> Conjugate for Vec<T> {
    fn conjugate(self) -> Self {
        self.into_iter()
            .map(Conjugate::conjugate)
            .collect()
    }
}

impl<T:Conjugate,const N:usize> Conjugate for [T;N] {
    fn conjugate(self) -> Self {
        self.map(Conjugate::conjugate)
    }
}

impl<A:Conjugate,
     B:Conjugate> Conjugate for Concatenated<A,B> {
    fn conjugate(self) -> Self {
        let (a,b)=self.into_parts();
        Self::new(a.conjugate(),
                  b.conjugate())
    }
}

impl<Index,A:ItemT<T=T>+ClosedMap<T>,T:Conjugate> Conjugate for Cropped<Index,A> {
    fn conjugate(self) -> Self {
        self.map(Conjugate::conjugate)
    }
}