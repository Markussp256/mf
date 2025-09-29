// removes an element and the corresponding coordinate for each dimension
use container_traits::*;
use utils::iter::IntoExactSizeIterator;
pub struct WithoutViewMut<'b,Index,C> {
    c:&'b mut C,
    wo_index:Index, // index that is not used in c
    size:Index,     // of Self
}

impl<'b,Index:ContainerIndex,C : Size<Index>> WithoutViewMut<'b,Index,C> {
    pub fn try_new(c:&'b mut C,wo_index:Index) -> Result<Self,IndexOutOfBoundsError<Index>> {
        let c_size=c.size();
        IndexOutOfBoundsError::try_new(&c_size, &wo_index)?;
        let size=Index::try_from_iter(
            c_size
                .into_iterator()
                .map(|s|s-1)).unwrap();
        Ok(Self{c,wo_index,size})
    }
}

impl<'b,Index:ContainerIndex,C> WithoutViewMut<'b,Index,C> {
    fn try_index_into_c_index(&self,index:Index) -> Result<Index,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&self.size,&index)?;
        Ok(Index::try_from_iter(
            index.into_iterator()
                 .zip(self.wo_index.iter())
                 .map(|(i,j)| if &i < j { i } else { i+1 })).unwrap())
    }
}


fn no_common_coord_fn<Index:ContainerIndex>(wo_index:Index) -> impl Fn(&Index) -> bool {
        move |ind:&Index|
            ind.iter()
               .zip(wo_index.iter())
               .all(|(i,j)|i != j)
}


fn c_index_into_index_fn<Index:ContainerIndex>(wo_index:Index) -> impl Fn(Index) -> Index {
    move |c_index|Index::try_from_iter(
                c_index.into_iterator()
                    .zip(wo_index.iter())
                    .map(|(i,j)|if &i <= j { i } else { i-1 })).unwrap()
}



impl<'b,Index:ContainerIndex, T, C> NumberOfDegreesOfFreedom<T> for WithoutViewMut<'b,Index,C> {
    fn ndofs(&self) -> usize {
        self.size
            .iter()
            .cloned()
            .into_product()
    }
}

impl<'b,Index:Clone, C> Size<Index> for WithoutViewMut<'b,Index,C> {
    fn size(&self) -> Index {
        self.size
            .clone()
    }
}


impl<'b,Index : ContainerIndex, T, C : Get<Index,T>> Get<Index,T> for WithoutViewMut<'b,Index,C> {
    fn get(&self, index:Index) -> Result<&T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c.get(c_index)
    }
}

impl<'b,Index : ContainerIndex, T, C : IndexedIter<Index, T>> IndexedIter<Index,T> for WithoutViewMut<'b,Index, C> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T : 'a {
        let ndofs:usize=<Self as NumberOfDegreesOfFreedom<T>>::ndofs(&self);
        let no_common_coord_fn=no_common_coord_fn(self.wo_index.clone());
        let c_index_into_index_fn=c_index_into_index_fn(self.wo_index.clone());
        self.c
            .indexed_iter()
            .filter(move |(i,_)|no_common_coord_fn(i))
            .map(move |(i,t)|(c_index_into_index_fn(i),t))
            .into_exact_size_iter(ndofs)
    }
}

impl<'b,Index, T, C> Iter<T> for WithoutViewMut<'b,Index, C> where Self : IndexedIter<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.indexed_iter()
            .map(|(_,t)|t)
    }
}

impl<'b,Index, T, C : ItemT<T=T>> ItemT for WithoutViewMut<'b,Index, C> {
    type T = T;
}


impl<'b,Index : ContainerIndex, T, C : IndexedIterMut<Index,T>> IndexedIterMut<Index, T> for WithoutViewMut<'b,Index,C> {
    fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T:'a {
        let ndofs:usize=<Self as NumberOfDegreesOfFreedom<T>>::ndofs(&self);
        let no_common_coord_fn=no_common_coord_fn(self.wo_index.clone());
        let c_index_into_index_fn=c_index_into_index_fn(self.wo_index.clone());
        self.c
            .indexed_iter_mut()
            .filter(move |(i,_)|no_common_coord_fn(i))
            .map(move |(i,t)|(c_index_into_index_fn(i),t))
            .into_exact_size_iter(ndofs)
    }
}

impl<'b,Index, T, C> IterMut<T> for WithoutViewMut<'b,Index, C> where Self : IndexedIterMut<Index, T> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        self.indexed_iter_mut()
            .map(|(_,t)|t)
    }
}

impl<'b,Index : ContainerIndex, T, C : GetMut<Index,T>> GetMut<Index,T> for WithoutViewMut<'b,Index, C> {
    fn get_mut(&mut self, index:Index) -> Result<&mut T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .get_mut(c_index)
    }
}