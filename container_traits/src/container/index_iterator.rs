use std::iter::Take;

use crate::{ContainerIndex, ContainerSize, DimensionMismatchError, IntoProduct, LowerBoundUpperBoundError};
use utils::iter::{IntoExactSizeIterator, WithExactSize};

#[derive(Clone,Debug)]
pub struct ContainerIndexIterator<Index> {
    lb                 : Index,
    ub                 : Index,
    size               : Index,
    current            : Index,
    linear_index_steps : Index,
    exhausted          : bool
}

impl<Index> ContainerIndexIterator<Index> {
    pub fn lb                 (&self) -> &Index { &self.lb }
    pub fn ub                 (&self) -> &Index { &self.ub }
    pub fn size               (&self) -> &Index { &self.size }    
    pub fn current            (&self) -> &Index { &self.current }
    pub fn linear_index_steps (&self) -> &Index { &self.linear_index_steps }
    pub fn exhausted          (&self) -> bool   {  self.exhausted }
}

impl<Index:ContainerIndex> ContainerIndexIterator<Index> {
    // iterator includes lb and ub
    pub fn try_new(lb:Index, ub:Index, size:Index) -> Result<Self,DimensionMismatchError<Index>> {
        LowerBoundUpperBoundError::try_new(&lb,&ub)?;
        let current=lb.clone();
        let mut sp=1;
        let mut m=0;
        let ub_m_lb=ub.iter().zip(lb.iter()).map(|(ubi,lbi)|ubi-lbi);
        let linear_index_steps=Index::try_from_iter(
            std::iter::once(1).chain(
                size.iter()
                    .zip(ub_m_lb)
                    .map(|(si,ubmlbi)|{ m+=sp*ubmlbi; sp*=si; sp-m})
            )
            .into_exact_size_iter(Index::DIM)).unwrap();
        let exhausted=!lb.is_elem_wise_strictly_smaller(&ub);
        Ok(ContainerIndexIterator { lb, ub, size, current, linear_index_steps, exhausted})
    }

    pub fn from_size(size:Index) -> Self {
        let lb=Index::try_from_iter(std::iter::repeat(0)).unwrap();
        let ub=Index::try_from_iter(size.iter().map(|si|if si == &0 { 0 } else { si-1 })).unwrap();
        ContainerIndexIterator::try_new(lb,ub,size).unwrap()
    }

    pub fn new_exact_size(size:Index) -> WithExactSize<Take<Self>> {
        let s=Self::from_size(size);
        let numel=s.numel();
        s.into_exact_size_iter(numel)
    }

    pub fn try_new_exact_size(lb:Index,ub:Index, size:Index) -> Result<WithExactSize<Take<Self>>,DimensionMismatchError<Index>> {
        let s=Self::try_new(lb,ub,size)?;
        let numel=s.numel();
        Ok(s.into_exact_size_iter(numel))
    }

    // 2nd arguments means how much linear index changes, i.e. how much the pointer has to propagate
    pub fn next_linear_index_step(& mut self) -> Option<(Index,usize)> {
        if self.exhausted {
            return None;
        }
        let rv=self.current.clone();
        let lub=self.lb.iter().zip(self.ub.iter());
        let i_dim_fn=||{
            let mut res=0;
            for (i,(ci,(lbi,ubi))) in self.current.iter_mut().zip(lub).enumerate() {
                let or=if *ci < *ubi {
                    *ci += 1;
                    Some(i)
                } else {
                    *ci = *lbi;
                    None
                };
                if let Some(r)=or {
                    res=r;
                    break;
                }
                if i == 0 {
                    self.exhausted=true;
                }
            }
            res};
        let i_dim=i_dim_fn();
        let lic=self.linear_index_steps.iter().nth(i_dim).unwrap();
        Some((rv,lic.clone()))
    }

    pub fn numel(&self) -> usize {
        self.size
            .iter()
            .cloned()
            .into_product()
    }
}


impl<Index:ContainerSize> ContainerIndexIterator<Index> {
    pub fn linear_index(&self) -> usize {
        self.size
            .linear_index(self.current
                              .clone()).unwrap()
    }
}

impl<Index:ContainerIndex> Iterator for ContainerIndexIterator<Index> {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        let rv=self.current.clone();
        // prepare for next
        let lub=self.lb.iter().zip(self.ub.iter());
        for (i,(ci,(lbi,ubi))) in self.current.iter_mut().zip(lub).enumerate() {
            if *ci < *ubi {
                *ci += 1;
                break;
            } else {
                *ci = *lbi;
                if i == 0 {
                    self.exhausted=true;
                }
            }
        }
        Some(rv)
    }
}

pub fn row_major_index_iterator(size:(usize,usize)) -> impl ExactSizeIterator<Item=(usize,usize)> {
    let flip=|(l,r)|(r,l);
    ContainerIndexIterator::from_size(flip(size))
              .map(flip)
              .into_exact_size_iter(size.0*size.1)
}

pub fn column_major_index_iterator(size:(usize,usize)) -> impl ExactSizeIterator<Item=(usize,usize)> {
    ContainerIndexIterator::from_size(size)
        .into_exact_size_iter(size.0*size.1)
}


#[test]
fn test23() {
    let mut iter=row_major_index_iterator((2,2));
    assert_eq!(iter.next(),Some((0,0)));
    assert_eq!(iter.next(),Some((0,1)));
    assert_eq!(iter.next(),Some((1,0)));
    assert_eq!(iter.next(),Some((1,1)));
    assert_eq!(iter.next(),Some((2,0)));
    assert_eq!(iter.next(),Some((2,1)));
    assert_eq!(iter.next(),None);
}