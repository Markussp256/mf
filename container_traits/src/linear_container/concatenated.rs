use crate::*;

use crate::LinearContainerConstructError as LCCE;

use utils::iter::ChainExactSize;

#[derive(Clone, Debug, PartialEq)]
pub struct Concatenated<A,B>(A,B);

pub type Concatenated3<A,B,C>=Concatenated<Concatenated<A,B>,C>;

use crate::for_dynamic::Concat;

impl<A,B> Concatenated<A,B> {
    pub fn new(a:A,b:B) -> Self {
        Self(a,b)
    }

    pub fn into_parts(self) -> (A,B) {
        (self.0, self.1)
    }

    pub fn a(&self) -> &A { &self.0 }
    pub fn b(&self) -> &B { &self.1 }
}


impl<A:Len,B> Concatenated<A,B> {
    fn find_acceptable_splits<'a,T:'a,E>(len:usize,f:impl Fn(usize) -> &'a T) -> impl Iterator<Item=usize>
    where A : TryAccept<usize, T, E>,
          B : TryAccept<usize, T, E> {
            let is_i_acc=move |i:&usize|
                A::try_accept(*i, &f).is_ok() &&
                B::try_accept(len-i,|j| f(i+j)).is_ok();
            (0..len)
                .filter(is_i_acc)
        }
}


impl<T, A : ChangeT<T,Output=A2>, A2,
        B : ChangeT<T,Output=B2>, B2> ChangeT<T> for Concatenated<A,B> {
    type Output=Concatenated<A2,B2>;
}


impl<T,A:Iter<T>,B:Iter<T>> Iter<T> for Concatenated<A,B> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.0.iter().chain_exact_size(
        self.1.iter())
    }
}

impl<T,A,B> IndexedIter<usize,T> for Concatenated<A,B> where Self : Iter<T> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(usize,&'a T)> where T : 'a {
        self.iter()
            .enumerate()
    }
}

impl<T,
     A : IntoIter<T>,
     B : IntoIter<T>> IntoIter<T> for Concatenated<A,B> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T> {
        let (a,b)=self.into_parts();
        ChainExactSize::chain_exact_size(
            a.into_iterator(),
             b.into_iterator())
    }
}

impl<T,
     A : IntoParameters<T>,
     B : IntoParameters<T>> IntoParameters<T> for Concatenated<A,B> {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=T> {
        let (a,b)=self.into_parts();
        ChainExactSize::chain_exact_size(
            a.into_parameters(),
             b.into_parameters())
    }
}

impl<T,
     A : ItemT<T=T>,
     B : ItemT<T=T>> IntoIndexedIter<usize,T> for Concatenated<A, B> where Self : IntoIter<T> {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(usize,T)> {
        self.into_iterator().enumerate()
    }
}

// impl<T,A:IntoIterator<Item=T>,
//        B:IntoIterator<Item=T>> IntoIterator for Concatenated<A,B> {
//     type Item = T;
//     type IntoIter = Chain<<A as IntoIterator>::IntoIter,
//                           <B as IntoIterator>::IntoIter>;
//     fn into_iter(self) -> Self::IntoIter {
//         let (a,b)=self.into_parts();
//         a.into_iter()
//          .chain(b.into_iter())
//     }
// }

impl<T,A:IntoVec<T>,
       B:IntoVec<T>> IntoVec<T> for Concatenated<A,B> {
    fn into_vec(self) -> Vec<T> {
        let (a,b)=self.into_parts();
        a.into_vec().concat(
        b.into_vec())
    }
}

impl<A:Size<usize>,B:Size<usize>> Size<usize> for Concatenated<A,B> {
    fn size(&self) -> usize {
         self.0.size()
        +self.1.size()
    }
}

impl<A : OCTSize<usize>,
     B : OCTSize<usize>> OCTSize<usize> for Concatenated<A,B> {
    const OCTSIZE:Option<usize> = match (A::OCTSIZE, B::OCTSIZE) {
        (Some(a),Some(b)) => Some(a+b),
        _ => None
    };
}

impl<A : NumberOfDegreesOfFreedom<T>,
     B : NumberOfDegreesOfFreedom<T>,T> NumberOfDegreesOfFreedom<T> for Concatenated<A,B> {
    fn ndofs(&self) -> usize {
         self.0.ndofs()
        +self.1.ndofs()
    }
}

impl<A : for_static::NumberOfDegreesOfFreedom<T>,
     B : for_static::NumberOfDegreesOfFreedom<T>,T> for_static::NumberOfDegreesOfFreedom<T> for Concatenated<A,B> {
    const NDOFS: usize = A::NDOFS + B::NDOFS;
}

impl<T,A:TryIntoElement<usize,T>+Len,B:TryIntoElement<usize,T>> TryIntoElement<usize,T> for Concatenated<A,B> {
    fn try_into_element(self,index:usize) -> Option<T> {
        let a_len=self.0.len();
        if index < a_len {
            self.0.try_into_element(index)
        } else {
            self.1.try_into_element(index-a_len)
        }
    }
}

impl<T,
     A : Get<usize, T>+Len,
     B : Get<usize, T>> Get<usize,T> for Concatenated<A,B> {
    fn get(&self, index:usize) -> Option<&T> {
        let a_len=self.0.len();
        if index < a_len {
            self.0.get(index)
        } else {
            self.1.get(index-a_len)
        }
    }
}

impl<T, A : First<T>, B> First<T> for Concatenated<A,B> {
    fn first(&self) -> Option<&T> {
        self.0
            .first()
    }
}

impl<T, A , B : Last<T>> Last<T> for Concatenated<A,B> {
    fn last(&self) -> Option<&T> {
        self.1
            .last()
    }
}

impl<T,
     A : TryAccept<usize,T>+Len,
     B : TryAccept<usize,T>> TryAccept<usize,T> for Concatenated<A,B> {
    fn try_accept<'a>(size:usize,f:impl Fn(usize) -> &'a T) -> Result<(),LCCE> where T: 'a {
        // not clear which error to throw we tried all splits and it didnt work
        Self::find_acceptable_splits(size,f)
            .next()
            .map(|_|())
            .ok_or(LCCE::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer)
    }
}


impl<T,A:ItemT<T=T>,B:ItemT<T=T>> ItemT for Concatenated<A,B> {
    type T=T;
}

impl<T, A : AnyFromVec<T,LCCE>+TryAccept<usize,T,LCCE>+Len,
        B : AnyFromVec<T,LCCE>+TryAccept<usize,T,LCCE>>
        AnyFromVec<T,LCCE> for Concatenated<A,B> {
    fn any_from_vec(v:Vec<T>) -> Result<Self,LCCE> {
        let index=
            Self::find_acceptable_splits(v.len(),|i|&v[i])
                .next()
                .ok_or(LCCE::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer)?;
        let (va,vb)=crate::vec_op::split(v,index);
        let a=A::any_from_vec(va)?;
        let b=B::any_from_vec(vb)?;
        Ok(Self::new(a,b))
    }
}

impl<T, A : FromFn<usize,T> + Len,
        B : FromFn<usize,T>> FromFn<usize,T> for Concatenated<A,B> {
        fn from_fn(len:usize, f:impl Fn(usize) -> T) -> Self {
            let a=A::from_fn(0,&f);
            let b=B::from_fn(len,f);
            // if many function evaluations happen it may be faster to store all values and provide refs
            Self::new(a,b)
        }
}

impl<T, A : TryFromFn<usize,T,LCCE> + Len,
        B : TryFromFn<usize,T,LCCE>> TryFromFn<usize,T,LCCE> for Concatenated<A,B> {
        fn try_from_fn(len:usize, f:impl Fn(usize) -> T) -> Result<Self,LCCE> {
            
            for size_a in 0..len {
                let ra=A::try_from_fn(size_a,&f);
                let rb=B::try_from_fn(len-size_a,|i|f(i+size_a));
                match (ra,rb) {
                    (Ok(a),Ok(b)) => { return Ok(Self::new(a,b)); }
                    _ => {}
                };
            }
            // if many function evaluations happen it may be faster to store all values and provide refs
            Err(ContainerConstructError::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer)
        }
}

impl<T,T2,A:Map<T,T2,Output=A2>,A2,B:Map<T,T2,Output=B2>,B2> Map<T,T2> for Concatenated<A,B> {
    type Output = Concatenated<A2,B2>;
    fn map(self, f:impl Fn(T) -> T2) -> Self::Output {
        let (a,b)=self.into_parts();
        Self::Output::new(a.map(&f), b.map(&f))
    }
}

impl<T,T2,A:TryMap<T,T2,LCCE, Output=A2>,A2,B:TryMap<T,T2,LCCE,Output=B2>,B2> TryMap<T,T2,LCCE> for Concatenated<A,B> {
    type Output = Concatenated<A2,B2>;
    fn try_map(self, f:impl Fn(T) -> T2) -> Result<Concatenated<A2,B2>, LCCE> {
        let (a,b)=self.into_parts();
        Ok(Self::Output::new(a.try_map(&f)?, b.try_map(&f)?))
    }
}

impl<T,
     A : GetMut<usize,T>+Len,
     B : GetMut<usize,T>> GetMut<usize,T> for Concatenated<A,B> {
    fn get_mut(&mut self, index:usize) -> Option<&mut T> {
        let a_len=self.0.len();
        if index < a_len {
            self.0.get_mut(index)
        } else {
            self.1.get_mut(index-a_len)
        }
    }
}

impl<T,A:IterMut<T>,B:IterMut<T>> IterMut<T> for Concatenated<A,B> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        self.0.iter_mut().chain_exact_size(
        self.1.iter_mut())
    }
}

impl<T,A,B> IndexedIterMut<usize,T> for Concatenated<A,B> where Self : IterMut<T> {
    fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(usize,&'a mut T)> where T:'a {
        self.iter_mut()
            .enumerate()
    }
}

impl<T,A,B:Extend<T>> Extend<T> for Concatenated<A,B> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.1.extend(iter)
    }
}

impl<A:Empty,B:Empty> Empty for Concatenated<A,B> {
    fn empty() -> Self {
        Self::new(A::empty(),B::empty())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
     && self.1.is_empty()
    }
}

impl<T,
     A : IntoIter<T>+AnyFromIterator<T,LCCE>,
     B : IntoIter<T>+AnyFromIterator<T,LCCE>> AnyFromIterator<T,LCCE> for Concatenated<A,B> {
    fn any_take_away<I: Iterator<Item = T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,LCCE> {
        let a=A::any_take_away(oref.map(|r|&r.0),iter)?;
        let b=B::any_take_away(oref.map(|r|&r.1),iter)?;
        Ok(Self::new(a,b))
    }

    crate::any_from_iter_impl!(T);
}

impl<T,
     A : AnyParameters<T,LCCE>,
     B : AnyParameters<T,LCCE>> AnyFromParameters<T,LCCE> for Concatenated<A,B> {
    fn any_take_away<I: Iterator<Item = T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,LCCE> {
        let a=A::any_take_away(oref.map(|r|&r.0),iter)?;
        let b=B::any_take_away(oref.map(|r|&r.1),iter)?;
        Ok(Self::new(a,b))
    }

    crate::any_from_parameters_impl!(T);
}

impl<T,A:Pop<T>,B:Pop<T>+Empty> Pop<T> for Concatenated<A,B> {
    fn pop(& mut self) -> Option<T> {
        if self.1.is_empty() {
            self.0.pop()
        } else {
            self.1.pop()
        }
    }
}

impl<T,A,B:Push<T>> Push<T> for Concatenated<A,B> {
    fn push(& mut self,t:T) {
        self.1.push(t)
    }
}