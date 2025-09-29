use utils::iter::IntoExactSizeIterator;

use container_traits::{ChangeT, Get, GetMut, IndexOutOfBoundsError, IndexedIter, IndexedIterMut, IntoIter, IntoVec, ItemT, Iter, IterMut, Map, Size, TryIntoElement};

use container_traits::container::ContainerIndex;


// we consider only the elements with index elementwise smalelr than max_size elements of C

// currently 

#[derive(Clone, Debug)]
pub struct MultiDSlice<Index,C>{
    c:C,
    lower:Index,  // index in C that correspons to index (0,0...) in MultiDSlice
    upper:Index,
    size:Index,
}

impl<Index:ContainerIndex,C:Size<Index>> MultiDSlice<Index,C> {
    pub fn try_from_lower_size(lower:Index, size:Index, c:C) -> Result<Self,IndexOutOfBoundsError<Index>> {
        let upper=lower.clone().elem_wise_add(size.clone());
        IndexOutOfBoundsError::try_new(&c.size(),&upper)?;
        Ok(Self{c, lower, upper, size})
    }

    pub fn try_from_lower_upper(lower:Index, upper:Index, c:C) -> Result<Self,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&c.size(),&upper)?;
        let size=upper.clone().try_elem_wise_sub(lower.clone()).expect("upper must by elem wise at least lower");
        Ok(Self{c, lower, upper, size})
    }
}

impl<Index:ContainerIndex,C> MultiDSlice<Index,C> {
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


impl<Index:Clone,C> Size<Index> for MultiDSlice<Index,C> {
    fn size(&self) -> Index {
        self.size.clone()
    }
}

impl<Index,F2,C:ChangeT<F2,Output=C2>,C2> ChangeT<F2> for MultiDSlice<Index,C> {
    type Output=MultiDSlice<Index,C2>;
}

impl<Index, T, C> Iter<T> for MultiDSlice<Index,C> where Self : IndexedIter<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.indexed_iter()
            .map(|(_,t)|t)
    }
}

impl<Index : Clone+ContainerIndex, T,
     C : IndexedIter<Index,T>> IndexedIter<Index,T> for MultiDSlice<Index,C> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T:'a {
        self.c
            .indexed_iter()
            .filter(move |(ind,_)| self.valid_c_index(ind))
            .map(move |(ind,t)|(ind.try_elem_wise_sub(self.lower.clone()).unwrap(),t))            
            .into_exact_size_iter(self.size.len())
    }
}

impl<Index : 'static+ContainerIndex, T,
     C : IntoIter<(Index,T)>> IntoIter<(Index,T)> for MultiDSlice<Index,C> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=(Index,T)> {
        let filter_fn=filter_fn(self.lower.clone(), self.upper.clone());
        let lower=self.lower.clone();
        self.c
            .into_iterator()
            .filter(move |(ind,_)| filter_fn(ind))
            .map(move |(ind,t)|(ind.try_elem_wise_sub(lower.clone()).unwrap(),t))            
            .into_exact_size_iter(self.size.len())
    }
}

impl<C:IntoVec<T>,T> IntoVec<T> for MultiDSlice<usize,C> {
    fn into_vec(self) -> Vec<T> {
        self.c
            .into_vec()
            .into_iter()
            .skip(self.lower)
            .take(self.size)
            .collect()
    }
}

impl<Index : ContainerIndex,T,C:TryIntoElement<Index,T>+Size<Index>> TryIntoElement<Index,T> for MultiDSlice<Index,C> {
    fn try_into_element(self,index:Index) -> Result<T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .try_into_element(c_index)
    }
}

impl<Index : ContainerIndex,T,C:Get<Index,T>> Get<Index,T> for MultiDSlice<Index,C> {
    fn get(&self, index:Index) -> Result<&T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .get(c_index)
    }
}

impl<Index,T,C:ItemT<T=T>> ItemT for MultiDSlice<Index,C> {
    type T=T;
}

impl<Index,T,T2,C:Map<T,T2,Output=C2>,C2> Map<T,T2> for MultiDSlice<Index,C> {
    type Output = MultiDSlice<Index,C2>;
    fn map(self, f:impl Fn(T) -> T2) -> Self::Output {
        MultiDSlice{
            c:self.c.map(&f),
            size:self.size,
            lower:self.lower,
            upper:self.upper}
    }
}

impl<Index:ContainerIndex,T,C:GetMut<Index,T>> GetMut<Index,T> for MultiDSlice<Index,C> {
    fn get_mut(&mut self, index:Index) -> Result<&mut T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .get_mut(c_index)
    }
}

impl<Index,T,C> IterMut<T> for MultiDSlice<Index,C> where Self : IndexedIterMut<Index,T> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        self.indexed_iter_mut()
            .map(|(_,t)|t)
    }
}

impl<Index:ContainerIndex,T,C:IndexedIterMut<Index,T>> IndexedIterMut<Index,T> for MultiDSlice<Index,C> {
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