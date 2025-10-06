use std::collections::BTreeMap;
use utils::iter::{IntoExactSizeIterator, LookAhead};
use container_traits::index_iterator::ContainerIndexIterator;
use container_traits::*;

#[derive(Clone,Debug)]
pub struct ContainerSparseView<'a,Index,T> {
    bm:BTreeMap<Index,&'a T>,
    default:&'a T,
    size:Index
}

impl<'a,Index:ContainerIndex,T> ContainerSparseView<'a,Index,T> {
    pub fn new(default:&'a T,size:Index) -> Self {
        let bm=BTreeMap::<Index,&'a T>::new();
        Self{bm,default,size}
    }

    pub fn into_parts(self) -> (BTreeMap<Index,&'a T>, &'a T, Index) {
        (self.bm, self.default, self.size)
    }
}

impl<'a,Index:ContainerIndex,T> ContainerSparseView<'a,Index,T> {
    pub fn try_new(bm:BTreeMap<Index,&'a T>,default:&'a T,size:Index) -> Result<Self,IndexOutOfBoundsError<Index>> {
        bm.keys()
          .map(|k|IndexOutOfBoundsError::try_new(&size,k))
          .collect::<Result<(),_>>()
          .map(|_|Self{bm,default,size})
    }

    pub fn insert(&mut self, key:Index, value:&'a T) -> Result<Option<&'a T>,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&self.size, &key)?;
        Ok(self.bm
               .insert(key, value))
    }
}

impl<'a,Index:Clone,T> Size<Index> for ContainerSparseView<'a,Index,T> {
    fn size(&self) -> Index {
        self.size
            .clone()
    }
}

impl<'a,Index : ContainerIndex, T> IsEmpty for ContainerSparseView<'a,Index,T> {
    fn is_empty(&self) -> bool {
        self.size()
            .iter()
            .any(|szi|szi == &0)
    }
}

impl<'a,Index,T> OCTSize<Index> for ContainerSparseView<'a,Index,T> {
    const OCTSIZE:Option<Index>=None;
}

// or should it number of nonzero elements?
impl<'a,Index : ContainerSize, T> NumberOfDegreesOfFreedom<T> for ContainerSparseView<'a,Index,T> {
    fn ndofs(&self) -> usize {
        self.size()
            .numel()
    }
}

fn iter_gen<
    'a,
    T          : 'a,
    I          : 'a+Iterator<Item=(IndexMBRef,&'a &'a T)>,
    IndexMBRef : 'a,
    Index      : ContainerSize>(
        bm_iter : I,
        fi      : impl 'static+Fn(IndexMBRef)->Index,
        def     : impl Fn()-> &'a T,
        size    : Index) -> impl ExactSizeIterator<Item=(Index,&'a T)> {
    let bm_iter=bm_iter.map(move |(ind,t)|(fi(ind),*t));
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
    let ndofs=size.iter().cloned().into_product();
    ContainerIndexIterator::from_size(size)
        .map(move |ind|(ind.clone(),get(ind)))
        .into_exact_size_iter(ndofs)
}

impl<'b,Index:'static+ContainerSize,T> IterIndexed<Index,T> for ContainerSparseView<'b,Index,T> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T : 'a {
        iter_gen(self.bm.iter(),Clone::clone,||&self.default,self.size())
    }
}

impl<'b,Index,T> Iter<T> for ContainerSparseView<'b,Index,T> where Self:IterIndexed<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.iter_indexed()
            .map(|(_,t)|t)
    }
}

impl<'a,Index,T> ItemT for ContainerSparseView<'a,Index,T> {
    type T=T;
}

impl<'b,Index:ContainerIndex,T> Get<Index,T> for ContainerSparseView<'b,Index,T> {
    fn get<'a>(&'a self, index:Index) -> Result<&'a T,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        Ok(self.bm
               .get(&index)
               .unwrap_or(&self.default))
    }
}

impl<'a,Index:ContainerIndex, T> First<T> for ContainerSparseView<'a,Index,T> {
    first_from_get!();
}

impl<'a,Index:ContainerSize, T> Last<T> for ContainerSparseView<'a,Index,T> {
    last_from_get!();
}