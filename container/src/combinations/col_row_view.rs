use super::{MultiDSliceView, MultiDSliceViewMut};
use container_traits::{AsMutSlice, AsSlice, ContainerIndex, ContainerSize, Get, GetMut, IndexOutOfBoundsError, ItemT, IterIndexed, IterMutIndexed, OCTSize, Size, SizeFromORef};


#[derive(
        container_derive::NumberOfDegreesOfFreedom,
        container_derive::IsEmpty,
        container_derive::Iter,
        container_derive::First,
        container_derive::Last,
        container_derive::ItemT)]
pub struct RowView<'a,Index0,Index1,C>(
    MultiDSliceView<'a,(Index0,Index1),C>
);

impl<'a,Index0,Index1,C:Size<(Index0,Index1)>> Size<Index1> for RowView<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn size(&self) -> Index1 {
        self.0
            .size()
            .1
    }
}


impl<'a,Index0 : Clone,Index1 : ContainerIndex,T,C:Size<(Index0,Index1)>+Get<(Index0,Index1),T>+ItemT<T=T>> Get<Index1,T> for RowView<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn get(&self, index:Index1) -> Result<&T,IndexOutOfBoundsError<Index1>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        let row_i=self.0.lb().0.clone();
        Ok(self.0
               .get((row_i,index))
               .unwrap())
    }
}

impl<'b,Index0,Index1 : ContainerIndex,T,C:AsSlice<T>+Size<(Index0,Index1)>+ItemT<T=T>> IterIndexed<Index1,T> for RowView<'b,Index0,Index1,C>
    where (Index0,Index1) : ContainerSize {
        fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index1,&'a T)> where T : 'a {
            self.0
                .iter_indexed()
                .map(|(ind,t)|(ind.1,t))
        }
}

impl<'a,
     Index0 : ContainerSize, 
     Index1 : ContainerSize,
     C : Size<(Index0,Index1)>> RowView<'a, Index0,Index1,C>
     where (Index0,Index1) : ContainerSize {
    pub fn try_new(c:&'a C, i_row:Index0) -> Result<Self, IndexOutOfBoundsError<Index0>> {
        let (n_rows,sz1)=c.size();
        IndexOutOfBoundsError::try_new(&n_rows,&i_row)?;
        let lb1=Index1::try_from_iter(std::iter::repeat(0).take(Index0::DIM)).unwrap();
        let ub1=Index1::try_from_iter(sz1.iter().map(|sz1i|if sz1i == &0 { 0 } else {sz1i-1})).unwrap();
        let sz0=Index0::try_from_iter(std::iter::repeat(1).take(Index1::DIM)).unwrap();
        Ok(Self(MultiDSliceView::try_new((i_row.clone(),lb1),(i_row.clone(),ub1),(sz0,sz1),c).unwrap()))
    }
}

impl<'a,Index0,Index1,C> OCTSize<Index1> for RowView<'a,Index0,Index1,C> {
    const OCTSIZE:Option<Index1>=None;
}

#[derive(
    container_derive::NumberOfDegreesOfFreedom,
    container_derive::IsEmpty,
    container_derive::Iter,
    container_derive::First,
    container_derive::Last,
    container_derive::ItemT)]
pub struct ColView<'a,Index0,Index1,C>(
    MultiDSliceView<'a,(Index0,Index1),C>
);

impl<'a,
     Index0 : ContainerSize, 
     Index1 : ContainerSize,
     C : Size<(Index0,Index1)>> ColView<'a, Index0,Index1,C>
     where (Index0,Index1) : ContainerSize {
    pub fn try_new(c:&'a  C, j_col:Index1) -> Result<Self, IndexOutOfBoundsError<Index1>> {
        let (sz0,n_cols)=c.size();
        IndexOutOfBoundsError::try_new(&n_cols,&j_col)?;
        let lb0=Index0::try_from_iter(std::iter::repeat(0).take(Index0::DIM)).unwrap();
        let ub0=Index0::try_from_iter(sz0.iter().map(|sz0i|if sz0i == &0 { 0 } else {sz0i-1})).unwrap();
        let sz1=Index1::try_from_iter(std::iter::repeat(1).take(Index1::DIM)).unwrap();
        Ok(Self(MultiDSliceView::try_new((lb0,j_col.clone()),(ub0,j_col),(sz0,sz1),c).unwrap()))
    }
}

impl<'a,Index0,Index1,C:Size<(Index0,Index1)>> Size<Index0> for ColView<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn size(&self) -> Index0 {
        self.0
            .size()
            .0
    }
}


impl<'a,Index0 : ContainerIndex,Index1 : Clone,T,C:Size<(Index0,Index1)>+Get<(Index0,Index1),T>+ItemT<T=T>> Get<Index0,T> for ColView<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn get(&self, index:Index0) -> Result<&T,IndexOutOfBoundsError<Index0>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        let col_j=self.0.lb().1.clone();
        Ok(self.0
               .get((index,col_j))
               .unwrap())
    }
}

impl<'b,Index0 : ContainerIndex,Index1,T,C:AsSlice<T>+Size<(Index0,Index1)>+ItemT<T=T>> IterIndexed<Index0,T> for ColView<'b,Index0,Index1,C>
    where (Index0,Index1) : ContainerSize {
        fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index0,&'a T)> where T : 'a {
            self.0
                .iter_indexed()
                .map(|(ind,t)|(ind.0,t))
        }
}

impl<'a,Index0,Index1,C> OCTSize<Index0> for ColView<'a,Index0,Index1,C> {
    const OCTSIZE:Option<Index0>=None;
}

#[derive(
    container_derive::NumberOfDegreesOfFreedom,
    container_derive::IsEmpty,
    container_derive::Iter,
    container_derive::First,
    container_derive::Last,
    container_derive::ItemT,
    container_derive::IterMut)]
pub struct RowViewMut<'a,Index0,Index1,C>(
    MultiDSliceViewMut<'a,(Index0,Index1),C>
);

impl<'a,
     Index0 : ContainerSize, 
     Index1 : ContainerSize,
     C : Size<(Index0,Index1)>> RowViewMut<'a,Index0,Index1,C>
     where (Index0,Index1) : ContainerSize {
    pub fn try_new(c:&'a mut C, i_row:Index0) -> Result<Self, IndexOutOfBoundsError<Index0>> {
        let (n_rows,sz1)=c.size();
        IndexOutOfBoundsError::try_new(&n_rows,&i_row)?;
        let lb1=Index1::try_from_iter(std::iter::repeat(0).take(Index0::DIM)).unwrap();
        let ub1=Index1::try_from_iter(sz1.iter().map(|sz1i|if sz1i == &0 { 0 } else {sz1i-1})).unwrap();
        let sz0=Index0::try_from_iter(std::iter::repeat(1).take(Index1::DIM)).unwrap();
        Ok(Self(MultiDSliceViewMut::try_new((i_row.clone(),lb1),(i_row.clone(),ub1),(sz0,sz1),c).unwrap()))
    }
}

impl<'a,Index0,Index1,C:Size<(Index0,Index1)>> Size<Index1> for RowViewMut<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn size(&self) -> Index1 {
        self.0
            .size()
            .1
    }
}


impl<'a,Index0 : Clone,Index1 : ContainerIndex,T,C:Size<(Index0,Index1)>+Get<(Index0,Index1),T>+ItemT<T=T>> Get<Index1,T> for RowViewMut<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn get(&self, index:Index1) -> Result<&T,IndexOutOfBoundsError<Index1>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        let row_i=self.0.lb().0.clone();
        Ok(self.0
               .get((row_i,index))
               .unwrap())
    }
}

impl<'a,Index0 : Clone,Index1 : ContainerIndex,T,C:Size<(Index0,Index1)>+GetMut<(Index0,Index1),T>+ItemT<T=T>> GetMut<Index1,T> for RowViewMut<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn get_mut(& mut self, index:Index1) -> Result<& mut T,IndexOutOfBoundsError<Index1>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        let row_i=self.0.lb().0.clone();
        Ok(self.0
               .get_mut((row_i,index))
               .unwrap())
    }
}

impl<'b,Index0,Index1 : ContainerIndex,T,C:AsSlice<T>+Size<(Index0,Index1)>+ItemT<T=T>> IterIndexed<Index1,T> for RowViewMut<'b,Index0,Index1,C>
    where (Index0,Index1) : ContainerSize {
        fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index1,&'a T)> where T : 'a {
            self.0
                .iter_indexed()
                .map(|(ind,t)|(ind.1,t))
        }
}

impl<'b,Index0,Index1 : ContainerIndex,T,C:AsMutSlice<T>+Size<(Index0,Index1)>+ItemT<T=T>> IterMutIndexed<Index1,T> for RowViewMut<'b,Index0,Index1,C>
    where (Index0,Index1) : ContainerSize {
        fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index1,&'a mut T)> where T : 'a {
            self.0
                .iter_mut_indexed()
                .map(|(ind,t)|(ind.1,t))
        }
}

impl<'a,C,Index0> OCTSize<usize> for RowViewMut<'a,Index0,usize,C> {
    const OCTSIZE:Option<usize>=None;
}

#[derive(
    container_derive::NumberOfDegreesOfFreedom,
    container_derive::IsEmpty,
    container_derive::Iter,
    container_derive::First,
    container_derive::Last,
    container_derive::ItemT,
    container_derive::IterMut)]
pub struct ColViewMut<'a,Index0,Index1,C>(
    MultiDSliceViewMut<'a,(Index0,Index1),C>
);

impl<'a,
     Index0 : ContainerSize, 
     Index1 : ContainerSize,
     C : Size<(Index0,Index1)>> ColViewMut<'a,Index0,Index1,C>
     where (Index0,Index1) : ContainerSize {
    pub fn try_new(c:&'a mut C, j_col:Index1) -> Result<Self, IndexOutOfBoundsError<Index1>> {
        let (sz0,n_cols)=c.size();
        IndexOutOfBoundsError::try_new(&n_cols,&j_col)?;
        let lb0=Index0::try_from_iter(std::iter::repeat(0).take(Index0::DIM)).unwrap();
        let ub0=Index0::try_from_iter(sz0.iter().map(|sz0i|if sz0i == &0 { 0 } else {sz0i-1})).unwrap();
        let sz1=Index1::try_from_iter(std::iter::repeat(1).take(Index1::DIM)).unwrap();
        Ok(Self(MultiDSliceViewMut::try_new((lb0,j_col.clone()),(ub0,j_col),(sz0,sz1),c).unwrap()))
    }
}

impl<'a,Index0,Index1,C:Size<(Index0,Index1)>> Size<Index0> for ColViewMut<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn size(&self) -> Index0 {
        self.0
            .size()
            .0
    }
}


impl<'a,Index0 : ContainerIndex,Index1 : Clone,T,C:Size<(Index0,Index1)>+Get<(Index0,Index1),T>+ItemT<T=T>> Get<Index0,T> for ColViewMut<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn get(&self, index:Index0) -> Result<&T,IndexOutOfBoundsError<Index0>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        let col_j=self.0.lb().1.clone();
        Ok(self.0
               .get((index,col_j))
               .unwrap())
    }
}

impl<'a,Index0 : ContainerIndex,Index1 : Clone,T,C:Size<(Index0,Index1)>+GetMut<(Index0,Index1),T>+ItemT<T=T>> GetMut<Index0,T> for ColViewMut<'a,Index0,Index1,C>
    where (Index0,Index1) : ContainerIndex {
    fn get_mut(& mut self, index:Index0) -> Result<& mut T,IndexOutOfBoundsError<Index0>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        let col_j=self.0.lb().1.clone();
        Ok(self.0
               .get_mut((index,col_j))
               .unwrap())
    }
}

impl<'b,Index0 : ContainerIndex,Index1,T,C:AsSlice<T>+Size<(Index0,Index1)>+ItemT<T=T>> IterIndexed<Index0,T> for ColViewMut<'b,Index0,Index1,C>
    where (Index0,Index1) : ContainerSize {
        fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index0,&'a T)> where T : 'a {
            self.0
                .iter_indexed()
                .map(|(ind,t)|(ind.0,t))
        }
}

impl<'b,Index0 : ContainerIndex,Index1,T,C:AsMutSlice<T>+Size<(Index0,Index1)>+ItemT<T=T>> IterMutIndexed<Index0,T> for ColViewMut<'b,Index0,Index1,C>
    where (Index0,Index1) : ContainerSize {
        fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index0,&'a mut T)> where T : 'a {
            self.0
                .iter_mut_indexed()
                .map(|(ind,t)|(ind.0,t))
        }
}

impl<'a,C:SizeFromORef<(Index0,usize)>,Index0> SizeFromORef<usize> for ColViewMut<'a,Index0,usize,C>
    where (Index0,usize) : ContainerSize,
        MultiDSliceView<'a,(Index0,usize),C> : SizeFromORef<(Index0,usize)> {
    fn size_from_oref(oref:Option<&Self>) -> usize {
        C::size_from_oref(oref.map(|s|s.0.c())).1
    }
}

impl<'a,Index0,Index1,C> OCTSize<Index0> for ColViewMut<'a,Index0,Index1,C> {
    const OCTSIZE:Option<Index0>=None;
}

// fn test_is_container_view<'a,T,C:AsSlice<T>+Container<(usize,usize),T=T>>(a:RowView<'a,usize,usize,C>) -> impl ContainerView<usize,T=T> {
//     a
// }

// fn test_is_linear_container<'a,T,C:ItemT<T=T>+AsSlice<T>+Container<(usize,usize)>>(a:RowView<'a,usize,usize,C>) -> impl LinearContainerView {
//     a
// }