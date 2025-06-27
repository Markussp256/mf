use crate::Matrix;
use container_traits::{AnyFromVec, Concatenated, ContainerIndex, Get, IndexedIter, IntoIndexedIter, IntoIter, ItemT, Iter, NumberOfDegreesOfFreedom, OCTSize, Size, TryIntoElement};
use num_traits::Zero;
use utils::iter::{ChainExactSize, IntoExactSizeIterator};

type U2=(usize,usize);

// tl=top left
// br=bottom right
#[derive(Clone, Debug, PartialEq,
          algebra_derive::ScalarContainer)]
pub struct BlockDiagonal<TL:Matrix, BR:Matrix<T=TL::T>> where TL::T : Zero {
    tl:TL,
    br:BR,
    zero:TL::T
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> BlockDiagonal<TL,BR> {
    pub fn new        (tl:TL, br:BR) -> Self     { Self{tl, br, zero:F::zero()} }
    pub fn into_parts ( self)        -> (TL, BR) { (self.tl, self.br)}
        fn nrows_total(&self)        -> usize    { self.tl.nrows()+self.br.nrows() }
        fn ncols_total(&self)        -> usize    { self.tl.ncols()+self.br.ncols() }
        fn size_total (&self)        -> U2       { (self.nrows_total(),self.ncols_total()) }
        fn len_total  (&self)        -> usize    { self.nrows_total() * self.ncols_total() }
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> BlockDiagonal<TL,BR> {
    fn into_rows_impl(self) -> impl ExactSizeIterator<Item=Concatenated<TL::Row,BR::Row>> {
        let (tl,br)=self.into_parts();
        let br_ncols=br.ncols();
        let tl_ncols=tl.ncols();
        let z=std::iter::repeat_with(F::zero);
        let br_zero_row=move ||BR::Row::any_from_vec(z.take(br_ncols).collect()).unwrap();
        let tl_zero_row=move ||TL::Row::any_from_vec(z.take(tl_ncols).collect()).unwrap();
        ChainExactSize::chain_exact_size(
            tl.into_rows()
                .map(move |r|Concatenated::new(r,br_zero_row())),
            br.into_rows()
                .map(move |r|Concatenated::new(tl_zero_row(),r)))
    }
}


impl<F:Zero, TL:Matrix<T=F>, BR:Matrix<T=F>> Get<U2,F> for BlockDiagonal<TL,BR> {
    fn get(&self,(i,j):U2) -> Option<&F> {
        let (tl_nrows,tl_ncols) =self.tl.matrix_dimensions();
        if i < tl_nrows &&
           j < tl_ncols {
            self.tl.get((i,j))
        } else if
           i >= tl_nrows &&
           j >= tl_ncols {
            self.br.get((i-tl_nrows,j-tl_ncols))
        } else if 
           i < self.nrows_total() &&
           j < self.ncols_total() {
            Some(&self.zero)
        } else {
            None
        }
    }
}

impl<F:Zero, TL:Matrix<T=F>, BR:Matrix<T=F>> Iter<F> for BlockDiagonal<TL,BR> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
        container_traits::for_any::iter::iter::impl_iter_from_get(self, self.size_total())
    }
}

impl<F:Zero, TL:Matrix<T=F>, BR:Matrix<T=F>> IndexedIter<U2,F> for BlockDiagonal<TL,BR> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a F)> where F:'a {
        container_traits::indexed_iter::impl_indexed_iter_from_get(self, self.size_total())
    }
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> ItemT for BlockDiagonal<TL,BR> {
    type T=F;
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> TryIntoElement<U2,F> for BlockDiagonal<TL,BR> {
    fn try_into_element(self,index:U2) -> Option<F> {
        let stldim=self.tl.matrix_dimensions();
        let sdim=stldim.elem_wise_add(self.br.matrix_dimensions());
        if        index.is_elem_wise_strictly_smaller(&stldim) {
            self.tl
                .try_into_element(index)
        } else if index.is_elem_wise_strictly_smaller(&sdim) {
            index.try_elem_wise_sub(stldim)
                 .and_then(|d|self.br.try_into_element(d))
        } else {
            None
        }
    }
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> IntoIter<F> for BlockDiagonal<TL,BR> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
        let len=self.len_total();
        self.into_rows_impl()
            .map(|r|r.into_iterator())
            .flatten()
            .into_exact_size_iter(len)
    }
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> IntoIndexedIter<U2,F> for BlockDiagonal<TL,BR> {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(U2,F)> {
        let len=self.len_total();
        self.into_rows_impl()
            .enumerate()
            .map(|(i,ri)|ri.into_iterator().enumerate().map(move |(j,rij)|((i,j),rij)))
            .flatten()
            .into_exact_size_iter(len)
    }
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> Size<U2> for BlockDiagonal<TL,BR> {
    fn size(&self) -> U2 {
        self.tl.size()
            .elem_wise_add(
        self.br.size())
    }
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> OCTSize<U2> for BlockDiagonal<TL,BR> {
    const OCTSIZE:Option<U2> = match (TL::OCTSIZE,BR::OCTSIZE) {
        (Some((r0,c0)),Some((r1,c1))) => Some((r0+r1,c0+c1)),
        _ => None
    };
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> NumberOfDegreesOfFreedom<F> for BlockDiagonal<TL,BR> {
    fn ndofs(&self) -> usize {
        self.tl.ndofs()
       +self.br.ndofs()
    }
}

impl<F:Zero, TL:Matrix<T=F>,BR:Matrix<T=F>> Matrix for BlockDiagonal<TL,BR> {
    
    type Row=Concatenated<TL::Row,BR::Row>;

    type Col=Concatenated<TL::Col,BR::Col>;

    fn nrows(&self) -> usize { self.nrows_total() }

    fn ncols(&self) -> usize { self.ncols_total() }

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        self.into_rows_impl()
    }

    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        let (tl,br)=self.into_parts();
        let br_nrows=br.nrows();
        let tl_nrows=tl.nrows();
        let z=std::iter::repeat_with(F::zero);
        let br_zero_col=move ||BR::Col::any_from_vec(z.take(br_nrows).collect()).unwrap();
        let tl_zero_col=move ||TL::Col::any_from_vec(z.take(tl_nrows).collect()).unwrap();
        ChainExactSize::chain_exact_size(
            tl.into_cols()
              .map(move |c|Concatenated::new(c,br_zero_col())),
            br.into_cols()
              .map(move |c|Concatenated::new(tl_zero_col(),c)))
    }
}