use container_traits::*;


#[derive(Clone,Debug)]
pub struct ColView<'b,Index0,Index1,C> {
    c:&'b C,
    j_col:Index1,
    size:Index0
}

impl<'b,Index0 , Index1: ContainerIndex, C : Size<(Index0,Index1)>> ColView<'b,Index0,Index1,C> {
    pub fn try_new(c:&'b C,j_col:Index1) -> Result<Self,IndexOutOfBoundsError<Index1>> {
        let (size,c_cols)=c.size();
        IndexOutOfBoundsError::try_new(&c_cols, &j_col)?;
        Ok(Self{c,j_col,size})
    }
}

impl<'b,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C> ColView<'b,Index0,Index1,C> {
    fn try_index_into_c_index(&self,index:Index0) -> Result<(Index0,Index1),IndexOutOfBoundsError<Index0>> {
        IndexOutOfBoundsError::try_new(&self.size,&index)?;
        Ok((index, self.j_col.clone()))
    }
}


impl<'b,
     T,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C : ItemT<T=T>> NumberOfDegreesOfFreedom<T> for ColView<'b,Index0,Index1,C> {
    fn ndofs(&self) -> usize {
        self.size
            .iter()
            .cloned()
            .into_product()
    }
}

impl<'b,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C> Size<Index0> for ColView<'b,Index0,Index1,C> {
    fn size(&self) -> Index0 {
        self.size
            .clone()
    }
}


impl<'b,
     T,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C : ItemT<T=T>+Get<(Index0,Index1),T>> Get<Index0,T> for ColView<'b,Index0,Index1,C> {
    fn get(&self, index:Index0) -> Result<&T,IndexOutOfBoundsError<Index0>> {
        let c_index=self.try_index_into_c_index(index)?;
        Ok(self.c.get(c_index).unwrap())
    }
}

impl<'b,
     T,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C : ItemT<T=T>+Get<(Index0,Index1),T>> IndexedIter<Index0,T> for ColView<'b,Index0, Index1, C> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index0,&'a T)> where T : 'a {
        self.size
            .clone()
            .index_iterator()
            .map(|j|(j.clone(), self.get(j).unwrap()))
    }
}

impl<'b,
     T,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C : ItemT<T=T>+Get<(Index0,Index1),T>> Iter<T> for ColView<'b,Index0,Index1, C> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.size
            .clone()
            .index_iterator()
            .map(|j| self.get(j).unwrap())
    }
}

impl<'b,
     T,
     Index0,
     Index1,
     C : ItemT<T=T>> ItemT for ColView<'b,Index0, Index1, C> {
    type T = T;
}