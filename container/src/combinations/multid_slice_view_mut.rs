use utils::iter::IntoExactSizeIterator;

use container_traits::{Get, GetMut, IndexOutOfBoundsError, IndexedIter, IndexedIterMut, ItemT, Iter, IterMut, Size};

use container_traits::container::ContainerIndex;

// we consider only the elements with index elementwise smalelr than max_size elements of C
// currently 

#[derive(Debug)]
pub struct MultiDSliceViewMut<'a,Index,C>{
    c:&'a mut C,
    lower:Index,  // index in C that correspons to index (0,0...) in MultiDSliceViewMut
    upper:Index,
    size:Index,
}

impl<'a,Index:ContainerIndex,C:Size<Index>> MultiDSliceViewMut<'a,Index,C> {
    pub fn try_from_lower_size(lower:Index, size:Index, c:&'a mut C) -> Result<Self,IndexOutOfBoundsError<Index>> {
        let upper=lower.clone().elem_wise_add(size.clone());
        IndexOutOfBoundsError::try_new(&c.size(),&upper)?;
        Ok(Self{c, lower, upper, size})
    }

    pub fn try_from_lower_upper(lower:Index, upper:Index, c:&'a mut C) -> Result<Self,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&c.size(),&upper)?;
        let size=upper.clone().try_elem_wise_sub(lower.clone()).expect("upper must by elem wise at least lower");
        Ok(Self{c, lower, upper, size})
    }
}

impl<'a,Index:ContainerIndex,C> MultiDSliceViewMut<'a,Index,C> {
    fn valid_c_index(&self,c_index:&Index) -> bool {
        c_index.is_elem_wise_smaller_eq(&self.upper) &&
        c_index.is_elem_wise_larger_eq( &self.lower)
    }

    fn try_index_into_c_index(&self, index:Index) -> Result<Index, IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&self.size, &index)?;
        Ok(index.elem_wise_add(self.lower.clone()))
    }
}



fn filter_fn<Index:ContainerIndex>(lower:Index,upper:Index) -> impl for <'a> Fn(&'a Index) -> bool {
  move |ind|
    ind.is_elem_wise_smaller_eq(&upper) &&
    ind.is_elem_wise_larger_eq( &lower)
}


impl<'a,Index:Clone,C> Size<Index> for MultiDSliceViewMut<'a,Index,C> {
    fn size(&self) -> Index {
        self.size.clone()
    }
}

impl<'b, Index, T, C> Iter<T> for MultiDSliceViewMut<'b,Index,C> where Self : IndexedIter<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.indexed_iter()
            .map(|(_,t)|t)
    }
}

impl<'b,
     Index : Clone+ContainerIndex, T,
     C : IndexedIter<Index,T>> IndexedIter<Index,T> for MultiDSliceViewMut<'b,Index,C> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T:'a {
        self.c
            .indexed_iter()
            .filter(move |(ind,_)| self.valid_c_index(ind))
            .map(move |(ind,t)|(ind.try_elem_wise_sub(self.lower.clone()).unwrap(),t))            
            .into_exact_size_iter(self.size.len())
    }
}


impl<'b,Index : ContainerIndex,T,C:Get<Index,T>> Get<Index,T> for MultiDSliceViewMut<'b,Index,C> {
    fn get(&self, index:Index) -> Result<&T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .get(c_index)
    }
}

impl<'b,Index,T,C:ItemT<T=T>> ItemT for MultiDSliceViewMut<'b,Index,C> {
    type T=T;
}

impl<'b,Index:ContainerIndex,T,C:GetMut<Index,T>> GetMut<Index,T> for MultiDSliceViewMut<'b,Index,C> {
    fn get_mut(&mut self, index:Index) -> Result<&mut T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .get_mut(c_index)
    }
}

impl<'b,Index,T,C> IterMut<T> for MultiDSliceViewMut<'b,Index,C> where Self : IndexedIterMut<Index,T> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        self.indexed_iter_mut()
            .map(|(_,t)|t)
    }
}

impl<'b,Index:ContainerIndex,T,C:IndexedIterMut<Index,T>> IndexedIterMut<Index,T> for MultiDSliceViewMut<'b,Index,C> {
    fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T:'a {
        let filter_fn=filter_fn(self.lower.clone(), self.upper.clone());
        let lower=self.lower.clone();
        self.c
            .indexed_iter_mut()
            .filter(move |(ind,_)| filter_fn(ind))
            .map(move |(ind,t)|(ind.try_elem_wise_sub(lower.clone()).unwrap(),t))            
            .into_exact_size_iter(self.size.len())
    }
}