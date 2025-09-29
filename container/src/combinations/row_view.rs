use container_traits::*;


#[derive(Clone,Debug)]
pub struct RowView<'b,Index0,Index1,C> {
    c:&'b C,
    i_row:Index0,
    size:Index1
}

impl<'b,Index0 : ContainerIndex, Index1, C : Size<(Index0,Index1)>> RowView<'b,Index0,Index1,C> {
    pub fn try_new(c:&'b C,i_row:Index0) -> Result<Self,IndexOutOfBoundsError<Index0>> {
        let (c_rows,size)=c.size();
        IndexOutOfBoundsError::try_new(&c_rows, &i_row)?;
        Ok(Self{c,i_row,size})
    }
}

impl<'b,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C> RowView<'b,Index0,Index1,C> {
    fn try_index_into_c_index(&self,index:Index1) -> Result<(Index0,Index1),IndexOutOfBoundsError<Index1>> {
        IndexOutOfBoundsError::try_new(&self.size,&index)?;
        Ok((self.i_row.clone(),index))
    }
}


impl<'b,
     T,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C : ItemT<T=T>> NumberOfDegreesOfFreedom<T> for RowView<'b,Index0,Index1,C> {
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
     C> Size<Index1> for RowView<'b,Index0,Index1,C> {
    fn size(&self) -> Index1 {
        self.size
            .clone()
    }
}


impl<'b,
     T,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C : ItemT<T=T>+Get<(Index0,Index1),T>> Get<Index1,T> for RowView<'b,Index0,Index1,C> {
    fn get(&self, index:Index1) -> Result<&T,IndexOutOfBoundsError<Index1>> {
        let c_index=self.try_index_into_c_index(index)?;
        Ok(self.c.get(c_index).unwrap())
    }
}

impl<'b,
     T,
     Index0 : ContainerIndex,
     Index1 : ContainerIndex,
     C : ItemT<T=T>+Get<(Index0,Index1),T>> IndexedIter<Index1,T> for RowView<'b,Index0, Index1, C> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index1,&'a T)> where T : 'a {
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
     C : ItemT<T=T>+Get<(Index0,Index1),T>> Iter<T> for RowView<'b,Index0,Index1, C> {
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
     C : ItemT<T=T>> ItemT for RowView<'b,Index0, Index1, C> {
    type T = T;
}