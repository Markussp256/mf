use std::collections::BTreeMap;
use num_traits::Zero;
use utils::iter::LookAhead;
use crate::*;

use crate::Size;

pub struct ContainerSparse<Index,T> {
    bm:BTreeMap<Index,T>,
    default:T,
    size:Index
}

impl<Index:ContainerIndex,T> ContainerSparse<Index,T> {
    pub fn try_new(bm:BTreeMap<Index,T>,default:T,size:Index) -> Result<Self,IndexOutOfBoundsError<Index>> {
        bm.keys()
          .map(|k|IndexOutOfBoundsError::try_new(&size,k))
          .collect::<Result<(),_>>()
          .map(|_|Self{bm,default,size})
    }

    pub fn into_parts(self) -> (BTreeMap<Index,T>, T, Index) {
        (self.bm, self.default, self.size)
    }
}

impl<Index:Clone,C> Size<Index> for ContainerSparse<Index,C> {
    fn size(&self) -> Index {
        self.size
            .clone()
    }
}

fn iter_gen<
    'a,
    T          : 'a,
    I          : 'a+Iterator<Item=(IndexMBRef,T)>,
    IndexMBRef : 'a,
    Index      : ContainerIndex>(
        bm_iter : I,
        fi      : impl 'static+Fn(IndexMBRef)->Index,
        def     : impl Fn()-> T,
        size    : Index) -> impl ExactSizeIterator<Item=(Index,T)> {
    let bm_iter=bm_iter.map(move |(ind,t)|(fi(ind),t));
    let mut bm_iter=LookAhead::<_,1>::new(bm_iter);
    let peek_ind=move |iter:&LookAhead<_,1>|iter.peek_n(1).map(|(ind,_):&(Index,_)|ind.clone());
    let mut get=move |ind:Index|
    {
        if Some(ind) == peek_ind(&bm_iter) {
            bm_iter.next().unwrap().1
        } else {
            def()
        }
    };
    size.index_iterator()
        .map(move |ind|(ind.clone(),get(ind)))
}

impl<Index:'static+ContainerIndex,T> IndexedIter<Index,T> for ContainerSparse<Index,T> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T:'a {
        iter_gen(self.bm.iter(),Clone::clone,||&self.default,self.size())
    }
}

impl<Index,T> Iter<T> for ContainerSparse<Index,T> where Self:IndexedIter<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.indexed_iter()
            .map(|(_,t)|t)
    }
}


impl<Index:'static+ContainerIndex,T:'static+Clone> IntoIndexedIter<Index,T> for ContainerSparse<Index,T> {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(Index,T)> {
        let (bm,default,size)=self.into_parts();
        iter_gen(bm.into_iter(),|s|s,move ||default.clone(),size)
    }
}

impl<Index,T> IntoIter<T> for ContainerSparse<Index,T> where Self : IntoIndexedIter<Index,T> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T> {
        self.into_indexed_iter()
            .map(|(_,t)|t)
    }
}

impl<Index,T> IntoVec<T> for ContainerSparse<Index,T> where Self : IntoIter<T> {
    fn into_vec(self) -> Vec<T> {
        self.into_iterator()
            .collect()
    }
}

impl<Index,T> ItemT for ContainerSparse<Index,T> {
    type T=T;
}

impl<Index:ContainerIndex,T> Get<Index,T> for ContainerSparse<Index,T> {
    fn get(&self, index:Index) -> Option<&T> {
        (index.is_elem_wise_strictly_smaller(&self.size)).then(||
        self.bm
            .get(&index)
            .unwrap_or(&self.default))
    }
}

impl<Index:ContainerIndex,T> TryIntoElement<Index,T> for ContainerSparse<Index,T> {
    fn try_into_element(self,index:Index) -> Option<T> {
        let (mut bm,default, size)=self.into_parts();
        (index.is_elem_wise_strictly_smaller(&size)).then(||
            bm
              .remove(&index)
              .unwrap_or(default))
    }
}

impl<Index:ContainerIndex,T:Zero> TryPutAt<Index,T> for ContainerSparse<Index,T> {
    fn try_put_at(size:Index,index:Index,t:T) -> Result<Self,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&size,&index)?;
        let mut bm=BTreeMap::new();
        bm.insert(index, t);
        Self::try_new(bm,T::zero(),size)
    }
}