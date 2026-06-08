use std::collections::BTreeMap;
use num_traits::Zero;
use utils::{iter::{IntoExactSizeIterator, LookAhead}, most_frequent_index};
use container_traits::index_iterator::ContainerIndexIterator;
use container_traits::*;


#[derive(Clone,Debug)]
pub struct ContainerSparse<Index,T> {
    bm:BTreeMap<Index,T>,
    default:T,
    size:Index
}

impl<Index:ContainerIndex,T> ContainerSparse<Index,T> {
    pub fn new(default:T,size:Index) -> Self {
        let bm=BTreeMap::<Index,T>::new();
        Self{bm,default,size}
    }

    pub fn into_parts(self) -> (BTreeMap<Index,T>, T, Index) {
        (self.bm, self.default, self.size)
    }
}

impl<Index:ContainerIndex,T> ContainerSparse<Index,T> {
    pub fn try_new(bm:BTreeMap<Index,T>,default:T,size:Index) -> Result<Self,IndexOutOfBoundsError<Index>> {
        bm.keys()
          .map(|k|IndexOutOfBoundsError::try_new(&size,k))
          .collect::<Result<(),_>>()
          .map(|_|Self{bm,default,size})
    }

    pub fn insert(&mut self, key:Index, value:T) -> Result<Option<T>,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&self.size, &key)?;
        Ok(self.bm
               .insert(key, value))
    }
}

impl<Index:Clone,T> Size<Index> for ContainerSparse<Index,T> {
    fn size(&self) -> Index {
        self.size
            .clone()
    }
}

impl<Index : ContainerIndex, T> IsEmpty for ContainerSparse<Index,T> {
    fn is_empty(&self) -> bool {
        self.size()
            .iter()
            .any(|szi|szi == &0)
    }
}

impl<Index,T> OCTSize<Index> for ContainerSparse<Index,T> {
    const OCTSIZE:Option<Index>=None;
}

// or should it number of nonzero elements?
impl<Index : ContainerSize, T> NumberOfDegreesOfFreedom<T> for ContainerSparse<Index,T> {
    fn ndofs(&self) -> usize {
        self.size()
            .numel()
    }
}

fn iter_gen<
    'a,
    T          : 'a,
    I          : 'a+Iterator<Item=(IndexMBRef,T)>,
    IndexMBRef : 'a,
    Index      : ContainerSize>(
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
    let ndofs=size.iter().cloned().into_product();
    ContainerIndexIterator::from_size(size)
        .map(move |ind|(ind.clone(),get(ind)))
        .into_exact_size_iter(ndofs)
}

impl<Index:'static+ContainerSize,T> IterIndexed<Index,T> for ContainerSparse<Index,T> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T:'a {
        iter_gen(self.bm.iter(),Clone::clone,||&self.default,self.size())
    }
}

impl<Index,T> Iter<T> for ContainerSparse<Index,T> where Self:IterIndexed<Index,T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.iter_indexed()
            .map(|(_,t)|t)
    }
}


impl<Index:'static+ContainerSize,T:'static+Clone> IntoIterIndexed<Index,T> for ContainerSparse<Index,T> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(Index,T)> {
        let (bm,default,size)=self.into_parts();
        iter_gen(bm.into_iter(),|s|s,move ||default.clone(),size)
    }
}

impl<Index,T> IntoIter<T> for ContainerSparse<Index,T> where Self : IntoIterIndexed<Index,T> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T> {
        self.into_iter_indexed()
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
    fn get(&self, index:Index) -> Result<&T,IndexOutOfBoundsError<Index>> {
        IndexOutOfBoundsError::try_new(&self.size(),&index)?;
        Ok(self.bm
               .get(&index)
               .unwrap_or(&self.default))
    }
}

impl<Index:ContainerIndex, T> First<T> for ContainerSparse<Index,T> {
    first_from_get!();
}

impl<Index:ContainerSize, T> Last<T> for ContainerSparse<Index,T> {
    last_from_get!();
}

impl<Index:ContainerIndex,T> TryIntoElement<Index,T> for ContainerSparse<Index,T> {
    fn try_into_element(self,index:Index) -> Result<T,IndexOutOfBoundsError<Index>> {
        let (mut bm,default, size)=self.into_parts();
        IndexOutOfBoundsError::try_new(&size,&index)?;
        Ok(bm
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

impl<T:Zero> TryFromVec<T,LinearContainerConstructError> for ContainerSparse<usize,T> {
    fn try_from_vec(v:Vec<T>) -> Result<Self,LinearContainerConstructError> {
        let len=v.len();
        let bm=BTreeMap::<usize,T>::from_iter(
            v.into_iter()
             .enumerate()
        );
        Ok(Self::try_new(bm,T::zero(),len).unwrap())
    }
}

impl<Index : Clone+Ord, T:Zero, T2:Zero> Map<T,T2> for ContainerSparse<Index,T> {
    type Output=ContainerSparse<Index,T2>;
    fn map(self, f:impl Fn(T) -> T2) -> Self::Output {
        let size=self.size();
        let bm=BTreeMap::<Index,T2>::from_iter(
            self.bm
                .into_iter()
                .map(|(k,v)|(k,f(v)))
        );
        Self::Output{bm,default:T2::zero(),size}
    }
}

impl<Index : Clone+Ord, T:Zero, T2:Zero> TryMap<T,T2,ContainerConstructError<Index>> for ContainerSparse<Index,T> {
    type Output=ContainerSparse<Index,T2>;
    fn try_map(self, f:impl Fn(T) -> T2) -> Result<Self::Output,ContainerConstructError<Index>> {
        Ok(self.map(f))
    }
}

impl<Index : ContainerSize, T : PartialEq> AnyFromIterator<T,ContainerConstructError<Index>> for ContainerSparse<Index,T> where Self : SizeFromORef<Index> {
    fn any_take_away<I:    Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,ContainerConstructError<Index>> {       
        let size=<Self as SizeFromORef<Index>>::size_from_oref(oref);
        let required_len=size.numel();
        if required_len == 0 {
            panic!("size is not allowed to be empty");
        }
        let vals=utils::iter::next_chunk_dyn(iter, required_len)
                .map_err(|v|ContainerConstructError::from(LenTooSmallError::new(required_len,v.len())))?;
        let ind_first_default=most_frequent_index(&vals).expect("size is not allowed to be empty");
        let mut o_default:Option<T>=None;
        let mut bm= BTreeMap::<Index,T>::new();
        let mut count=0;
        for (index,value) in ContainerIndexIterator::from_size(size.clone()).zip(vals) {
            if count == ind_first_default {
                o_default=Some(value);
            } else if o_default.is_none() || &value != o_default.as_ref().unwrap() {
                bm.insert(index,value);
            }
            count += 1;
        }
        Ok(Self{bm,default:o_default.unwrap(),size})
    }
}

impl<Index:'static+ContainerSize, T:'static+Clone+Zero> TryFromFn<Index,T> for ContainerSparse<Index,T> {
    fn try_from_fn(size:Index, f:impl Fn(Index) -> T) -> Result<Self,ContainerConstructError<Index>> {
        let bm=
            BTreeMap::<Index,T>::from_iter(
            ContainerIndexIterator::from_size(size.clone())
                .map(|ind|(ind.clone(),f(ind)))
                .filter(|(_,t)|!t.is_zero()));
        Ok(Self{bm,default:T::zero(),size})
    }
}

impl<Index, T> TryAccept<Index,T> for ContainerSparse<Index,T> {
    fn try_accept<'a>(_:Index,_:impl Fn(Index) -> &'a T) -> Result<(),ContainerConstructError<Index>> where T: 'a {
        Ok(())
    }
}

impl<Index : 'static+ContainerSize, T> RebindNAlgebraScalar<ContainerConstructError<Index>> for ContainerSparse<Index,T> {
    type WithNAlgebraScalar<T2 : rebind_nalgebra_scalar::NAlgebraScalar> = ContainerSparse<Index,T2>;
}
