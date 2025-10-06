// removes an element and the corresponding coordinate for each dimension
use container_traits::*;
use utils::iter::IntoExactSizeIterator;
pub struct Without<Index,C> {
    c:C,
    wo_index:Index, // index that is not used in c
    size:Index,     // of Self
}

impl<Index:ContainerIndex,C : Size<Index>> Without<Index,C> {
    pub fn try_new(c:C,wo_index:Index) -> Result<Self,IndexOutOfBoundsError<Index>> {
        let c_size=c.size();
        IndexOutOfBoundsError::try_new(&c_size, &wo_index)?;
        let size=Index::try_from_iter(
            c_size
                .into_iterator()
                .map(|s|s-1)).unwrap();
        Ok(Self{c,wo_index,size})
    }
}

impl<Index:ContainerIndex,C> Without<Index,C> {
    pub fn into_parts(self) -> (C,Index) { (self.c,self.wo_index) }

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



impl<Index:ContainerIndex, T, C> NumberOfDegreesOfFreedom<T> for Without<Index,C> {
    fn ndofs(&self) -> usize {
        self.size
            .iter()
            .cloned()
            .into_product()
    }
}

impl<Index:Clone, C> Size<Index> for Without<Index,C> {
    fn size(&self) -> Index {
        self.size
            .clone()
    }
}

impl<Index:Iter<usize>,C> IsEmpty for Without<Index,C> {
    fn is_empty(&self) -> bool {
        self.size
            .iter()
            .any(|szi|szi == &0)
    }
}

impl<Index : ContainerIndex, T, C : Get<Index,T>> Get<Index,T> for Without<Index,C> {
    fn get(&self, index:Index) -> Result<&T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c.get(c_index)
    }
}

impl<Index : ContainerIndex, T, C : IterIndexed<Index, T>> IterIndexed<Index,T> for Without<Index, C> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T : 'a {
        let ndofs:usize=<Self as NumberOfDegreesOfFreedom<T>>::ndofs(&self);
        let no_common_coord_fn=no_common_coord_fn(self.wo_index.clone());
        let c_index_into_index_fn=c_index_into_index_fn(self.wo_index.clone());
        self.c
            .iter_indexed()
            .filter(move |(i,_)|no_common_coord_fn(i))
            .map(move |(i,t)|(c_index_into_index_fn(i),t))
            .into_exact_size_iter(ndofs)
    }
}

impl<Index, T, C> Iter<T> for Without<Index, C> where Self : IterIndexed<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.iter_indexed()
            .map(|(_,t)|t)
    }
}

impl<Index, T, C : ItemT<T=T>> ItemT for Without<Index, C> {
    type T = T;
}

impl<Index : 'static+ContainerIndex, T, C : 'static+IntoIterIndexed<Index,T>> IntoIterIndexed<Index, T> for Without<Index,C> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(Index,T)> {
        let ndofs:usize=<Self as NumberOfDegreesOfFreedom<T>>::ndofs(&self);
        let no_common_coord_fn=no_common_coord_fn(self.wo_index.clone());
        let c_index_into_index_fn=c_index_into_index_fn(self.wo_index.clone());
        self.c
            .into_iter_indexed()
            .filter(move |(i,_)|no_common_coord_fn(i))
            .map(move |(i,t)|(c_index_into_index_fn(i),t))
            .into_exact_size_iter(ndofs)
    }
}

impl<Index, T, C> IntoIter<T> for Without<Index, C> where Self : IntoIterIndexed<Index, T> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T> {
        self.into_iter_indexed()
            .map(|(_,t)|t)
    }
}

impl<Index : ContainerIndex, T, C : TryIntoElement<Index, T>> TryIntoElement<Index, T> for Without<Index, C> {
    fn try_into_element(self,index:Index) -> Result<T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .try_into_element(c_index)
    }
}


impl<Index : ContainerIndex, T, C : IterMutIndexed<Index,T>> IterMutIndexed<Index, T> for Without<Index,C> {
    fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T:'a {
        let ndofs:usize=<Self as NumberOfDegreesOfFreedom<T>>::ndofs(&self);
        let no_common_coord_fn=no_common_coord_fn(self.wo_index.clone());
        let c_index_into_index_fn=c_index_into_index_fn(self.wo_index.clone());
        self.c
            .iter_mut_indexed()
            .filter(move |(i,_)|no_common_coord_fn(i))
            .map(move |(i,t)|(c_index_into_index_fn(i),t))
            .into_exact_size_iter(ndofs)
    }
}

impl<Index, T, C> IterMut<T> for Without<Index, C> where Self : IterMutIndexed<Index, T> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        self.iter_mut_indexed()
            .map(|(_,t)|t)
    }
}

impl<Index : ContainerIndex, T, C : GetMut<Index,T>> GetMut<Index,T> for Without<Index, C> {
    fn get_mut(&mut self, index:Index) -> Result<&mut T,IndexOutOfBoundsError<Index>> {
        let c_index=self.try_index_into_c_index(index)?;
        self.c
            .get_mut(c_index)
    }
}