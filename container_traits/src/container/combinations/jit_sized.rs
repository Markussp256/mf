use utils::iter::IntoExactSizeIterator;

use crate::{Size, ChangeT, Get, GetMut, IndexedIter, IndexedIterMut, IntoIter, IntoVec, ItemT, Iter, IterMut, Map, Pop, TryIntoElement};

use crate::container::ContainerIndex;

// jit-sized means just in time sized
// it means the size is not known at compile time but before the instance is constructed

#[derive(Clone, Debug)]
pub struct JITSize<Index> {
    size:Index
}


#[derive(Clone, Debug)]
pub struct JITSized<Index,C>{
    size:Index,
    c:C
}

impl<Index:ContainerIndex> JITSize<Index> {
    pub fn new(size:Index) -> Self {
        Self{size}
    }

    pub fn try_new_container<C:Size<Index>>(self, c:C) -> Option<JITSized<Index,C>> {
        let size=self.size.clone();
        (c.size() == size).then(||
            JITSized { size, c })
    }
}

impl<Index:Clone,C> Size<Index> for JITSized<Index,C> {
    fn size(&self) -> Index {
        self.size.clone()
    }
}

impl<Index,F2,C:ChangeT<F2,Output=C2>,C2> ChangeT<F2> for JITSized<Index,C> {
    type Output=JITSized<Index,C2>;
}

impl<Index, T, C> Iter<T> for JITSized<Index,C> where Self : IndexedIter<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.indexed_iter()
            .map(|(_,t)|t)
    }
}

impl<Index : Clone+ContainerIndex, T,
     C : IndexedIter<Index,T>> IndexedIter<Index,T> for JITSized<Index,C> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T:'a {
        let size=self.size();
        let len=size.len();
        self.c
            .indexed_iter()
            .filter(move |(ind,_)| ind.is_elem_wise_strictly_smaller(&size))
            .into_exact_size_iter(len)
    }
}

impl<Index : 'static+ContainerIndex, T,
     C : IntoIter<(Index,T)>> IntoIter<(Index,T)> for JITSized<Index,C> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=(Index,T)> {
        let size=self.size();
        let len=size.len();
        self.c
            .into_iterator()
            .filter(move |(ind,_)| ind.is_elem_wise_strictly_smaller(&size))
            .into_exact_size_iter(len)
    }
}

impl<C:IntoVec<T>,T> IntoVec<T> for JITSized<usize,C> {
    fn into_vec(self) -> Vec<T> {
        let len=self.size();
        crate::vec_op::crop(self.c.into_vec(),len)
    }
}

impl<Index : ContainerIndex,T,C:TryIntoElement<Index,T>+Size<Index>> TryIntoElement<Index,T> for JITSized<Index,C> {
    fn try_into_element(self,index:Index) -> Option<T> {
        if index.is_elem_wise_strictly_smaller(&self.size) {
            self.c.try_into_element(index)
        } else {
            None
        }
    }
}

impl<Index : ContainerIndex,T,C:Get<Index,T>> Get<Index,T> for JITSized<Index,C> {
    fn get(&self, index:Index) -> Option<&T> {
        if index.is_elem_wise_strictly_smaller(&self.size) {
            self.c.get(index)
        } else {
            None
        }
    }
}

impl<Index,T,C:ItemT<T=T>> ItemT for JITSized<Index,C> {
    type T=T;
}

impl<Index,T,T2,C:Map<T,T2,Output=C2>,C2> Map<T,T2> for JITSized<Index,C> {
    type Output = JITSized<Index,C2>;
    fn map(self, f:impl Fn(T) -> T2) -> Self::Output {
        JITSized{size:self.size,
                c:self.c.map(&f)}
    }
}

impl<Index:ContainerIndex,T,C:GetMut<Index,T>> GetMut<Index,T> for JITSized<Index,C> {
    fn get_mut(&mut self, index:Index) -> Option<&mut T> {
        if index.is_elem_wise_strictly_smaller(&self.size) {
            self.c.get_mut(index)
        } else {
            None
        }
    }
}

impl<Index,T,C> IterMut<T> for JITSized<Index,C> where Self : IndexedIterMut<Index,T> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        self.indexed_iter_mut()
            .map(|(_,t)|t)
    }
}

impl<Index:ContainerIndex,T,C:IndexedIterMut<Index,T>> IndexedIterMut<Index,T> for JITSized<Index,C> {
    fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T:'a {
        let size=self.size();
        let len=size.len();
        self.c
            .indexed_iter_mut()
            .filter(move |(ind,_)| ind.is_elem_wise_strictly_smaller(&size))            
            .into_exact_size_iter(len)
    }
}

impl<T,C:Pop<T>> Pop<T> for JITSized<usize,C> {
    fn pop(& mut self) -> Option<T> {
        let res=self.c.pop();
        if res.is_some() {
            self.size-=1;
        }
        res
    }
}