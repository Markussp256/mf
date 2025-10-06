use container_traits::*;
use utils::iter::IntoExactSizeIterator;

// we consider only the elements with index elementwise smalelr than max_size elements of C
// currently 

#[derive(Debug)]
pub struct MultiDSliceViewMut<'a,Index,C>{
    c:&'a mut C,
    cii:ContainerIndexIterator<Index>
}

impl<'a,Index,C> MultiDSliceViewMut<'a,Index,C> {
    pub fn c(&self) -> &C {
        self.c
    }

    pub fn lb(&self) -> &Index {
        self.cii
            .lb()
    }

    pub fn ub(&self) -> &Index {
        self.cii
            .ub()
    }
}

impl<'a,Index:ContainerIndex,C:Size<Index>> MultiDSliceViewMut<'a,Index,C> {
    pub fn try_new(lb:Index, ub:Index, size:Index, c:&'a mut C) -> Result<Self,DimensionMismatchError<Index>> {
        ContainerIndexIterator::try_new(lb, ub, size)
            .map(|cii|Self{c,cii})
    }

    pub fn from_size(size:Index, c:&'a mut C) -> Self {
        let cii=ContainerIndexIterator::from_size(size);
        Self{c,cii}
    }
}

impl<'a,Index:ContainerIndex,C> MultiDSliceViewMut<'a,Index,C> {
    fn try_index_into_c_index(&self, index:Index) -> Result<Index, IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&self.size(), &index)?;
        Ok(index.elem_wise_add(self.cii.lb().clone()))
    }
}



// fn filter_fn<Index:ContainerIndex>(lower:Index,upper:Index) -> impl for <'a> Fn(&'a Index) -> bool {
//   move |ind|
//     ind.is_elem_wise_smaller_eq(&upper) &&
//     ind.is_elem_wise_larger_eq( &lower)
// }


impl<'a,Index:Clone,C> Size<Index> for MultiDSliceViewMut<'a,Index,C> {
    fn size(&self) -> Index {
        self.cii.size().clone()
    }
}

impl<'a,Index:Iter<usize>,C> IsEmpty for MultiDSliceViewMut<'a,Index,C> {
    fn is_empty(&self) -> bool {
        self.cii
            .size()
            .iter()
            .any(|szi|szi == &0)
    }
}

impl<'a,T, Index : ContainerSize, C:ItemT<T=T>> NumberOfDegreesOfFreedom<T> for MultiDSliceViewMut<'a,Index,C> {
    fn ndofs(&self) -> usize {
        self.size()
            .numel()
    }
}


impl<'b, Index : ContainerSize, T, C : Size<Index>+AsSlice<T>> Iter<T> for MultiDSliceViewMut<'b,Index,C> where Self : IterIndexed<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        IterIterator::new(self.c, self.cii.clone())
    }
}

impl<'b,
     Index : ContainerSize, T,
     C : AsSlice<T>+Size<Index>> IterIndexed<Index,T> for MultiDSliceViewMut<'b,Index,C> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T:'a {
        let sz=self.size();
        let numel=self.cii.numel();
        ContainerIndexIterator::from_size(sz)
            .zip(IterIterator::new(self.c, self.cii.clone()))
            .into_exact_size_iter(numel)
    }
}


impl<'b, Index : ContainerSize, T, C:AsMutSlice<T>+Size<Index>> IterMut<T> for MultiDSliceViewMut<'b,Index,C> where Self : IterIndexed<Index,T> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
        IterMutIterator::new(self.c, self.cii.clone())
    }
}

impl<'b,
     Index : Clone+ContainerSize, T,
     C : AsMutSlice<T>+Size<Index>> IterMutIndexed<Index,T> for MultiDSliceViewMut<'b,Index,C> {
    fn iter_mut_indexed<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T:'a {
        let numel=self.cii.numel();
        let sz=self.size();
        ContainerIndexIterator::from_size(sz)
            .zip(IterMutIterator::new(self.c, self.cii.clone()))
            .into_exact_size_iter(numel)
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

impl<'a,T,C:Get<Index,T>+ItemT<T=T>,Index:ContainerIndex> First<T> for MultiDSliceViewMut<'a,Index,C> {
    first_from_get!();
}

impl<'a,T,C:Get<Index,T>+ItemT<T=T>,Index:ContainerSize> Last<T> for MultiDSliceViewMut<'a,Index,C> {
    last_from_get!();
}