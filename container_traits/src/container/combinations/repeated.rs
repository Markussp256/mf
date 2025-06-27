use crate::*;

// zero elements is not allowed
// one could add it by using Option<value>
pub struct Repeated<Index,C> {
    reps:Index,
    size:Index,
    c:C
}

impl<Index:ContainerIndex,C:Size<Index>> Repeated<Index,C> {
    pub fn try_new(reps:Index,c:C) -> Option<Self> {
        let size=reps.clone().elem_wise_mul(c.size());
        Some(Self{reps,size,c})
    }
}

impl<Index:Clone,C> Size<Index> for Repeated<Index,C> {
    fn size(&self) -> Index {
        self.size
            .clone()
    }
}

impl<Index:ContainerIndex,C:Size<Index>> Repeated<Index,C> {
    fn try_convert_index(&self, index:Index) -> Option<Index> {
        index.is_elem_wise_smaller_eq(&self.size).then(||
            index.elem_wise_mod(self.c.size()))
    }
}

// impl<Index,C> Repeated<Index,C> {
//     pub fn smallest_possible_c_len<'a,T>(iter:impl ExactSizeIterator<Item=&'a T>) -> Option<usize>
//     where T: 'a+PartialEq, C : TryAccept<T>  {
//         let v:Vec<&T>=iter.collect();
//         let len=v.len();
//         if len == 0 { return Some(0); } // if we use 0 repetitions
//         utils::number_theory::get_divisors(len)
//             .into_iter()
//             .filter(|&i|C::try_accept(v.clone().into_iter().take(i)).is_ok())
//             .find(|&i|(0..len-i).all(|j| v[j+i] == v[j]))
//     }
// }


impl<Index:ContainerIndex,C> Repeated<Index,C> {
    fn iter_gen<'a,Out>(&'a self,f:impl Fn(Index) -> Out) -> impl ExactSizeIterator<Item=Out> where Out:'a {
        self.size()
            .index_iterator()
            .map(f)
    }
}

impl<Index:ContainerIndex,T,C:Get<Index,T>+Size<Index>> Iter<T> for Repeated<Index,C> where Self : Get<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.iter_gen(move |ind: Index|self.get(ind).unwrap())
    }
}

impl<Index:ContainerIndex,T,C:Get<Index,T>+Size<Index>> IndexedIter<Index,T> for Repeated<Index,C> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T:'a {
        self.iter_gen(move |ind: Index|(ind.clone(),self.get(ind).unwrap()))
    }
}

impl<T:Clone,C> IntoVec<T> for Repeated<usize,C> where Self : Iter<T> {
    fn into_vec(self) -> Vec<T> {
        self.iter()
            .cloned()
            .collect()
    }
}


impl<Index,C:ItemT> ItemT for Repeated<Index,C> {
    type T=C::T;
}

impl<Index:ContainerIndex,T,C:Get<Index,T>+Size<Index>> Get<Index,T> for Repeated<Index,C> {
    fn get(&self, index:Index) -> Option<&T> {
        self.try_convert_index(index)
            .and_then(|c_index|self.c.get(c_index))
    }
}

impl<Index:ContainerIndex,T,C:TryIntoElement<Index,T>+Size<Index>> TryIntoElement<Index,T> for Repeated<Index,C> {
    fn try_into_element(self, index:Index) -> Option<T> {
        self.try_convert_index(index)
            .and_then(|c_index|self.c.try_into_element(c_index))
    }
}

// impl<Index,
//      T : PartialEq, E,
//      C : TryAccept<Index,T,E>> TryAccept<Index,T,E> for Repeated<Index,C> {
//     fn is_acceptable<'a>(iter:impl ExactSizeIterator<Item=&'a T>) -> Result<(),E> where T: 'a {
//         Self::smallest_possible_c_len(iter)
//            .ok_or(ContainerConstructError::DataInSourceContainerDoesNotSatisfyPropertiesForTargetContainer)
//            .map(|_|())
//     }
// }

// impl<T : PartialEq,
//      C : AnyFromVec<T>+TryAccept<T>> AnyFromVec<T> for Repeated<Index,C> {
//     fn any_from_vec(v:Vec<T>) -> Option<Self> {
//         let c_len=Self::smallest_possible_c_len(v.iter())?;
//         let reps=v.len()/c_len;
//         let c=C::any_from_vec(crate::vec_op::crop(v,c_len)).unwrap();
//         Some(Self::new(c,reps))
//     }
// }

// impl<T : PartialEq,
//      C : AnyFromFn<T>+TryAccept<T>> AnyFromFn<T> for Repeated<Index,C> {
//     fn any_from_fn(len:usize, f:impl Fn(usize) -> T) -> Option<Self> {
//         let v:Vec<T>=(0..len).map(&f).collect();
//         let c_len=Self::smallest_possible_c_len(v.iter())?;
//         let reps=v.len() / c_len;
//         let c=C::any_from_fn(c_len,f).unwrap();
//         Some(Self::new(c,reps))
//     }
// }

impl<Index:ContainerIndex,T,T2,C:Map<T,T2,Output=C2>,C2:Size<Index>> Map<T,T2> for Repeated<Index,C> {
    type Output=Repeated<Index,C2>;

    fn map(self, f:impl Fn(T) -> T2) -> Self::Output {
        Repeated::try_new(self.reps,self.c.map(f)).unwrap()
    }
}