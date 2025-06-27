use num_traits::{One, Zero};

use container_traits::{AnyFromIterator, AnyMap, ChangeT, CommonLengthError, FromFn, Get, IndexedIter,  IntoIndexedIter, IntoIter, ItemT, Iter, Len, LenTooSmallError, LinearContainerConstructError, Map, NumberOfDegreesOfFreedom, OCTSize, Size, SizeFromORef, TryAccept, TryCommonLength, TryFromFn, TryIntoElement};
use matrix_traits::{row_col::*, AsBaseMatrix, IntoBaseMatrix, Identity, Matrix, MatrixTryConstruct, MatrixConstructError, Transpose};
use std::fmt::Display;
use std::ops::{Index,IndexMut};
use utils::iter::IntoExactSizeIterator;

type U2=(usize,usize);

#[derive(algebra_derive::ScalarContainer,
         container_derive::Empty,
         derive_more::IntoIterator,
         derive_more::AsRef,
         derive_more::AsMut,
)]
pub struct MatrixGeneric<Row,
                         Col:ChangeT<Row>>(<Col as ChangeT<Row>>::Output); // =<Row as Transpose>::Output

pub type MatrixGenericFromRow<Row> = MatrixGeneric<Row, <Row as Transpose>::Output>;

impl<Row : Len,
     Col : ChangeT<Row,Output = C>,
     C: Len+Get<usize,Row>> MatrixGeneric<Row,Col> {
    fn nrows_private(&self) -> usize {
        self.0.len()
    }

    fn ncols_private(&self) -> usize {
        self.0
            .get(0)
            .map(|r|r.len())
            .unwrap_or(0)
    }

    fn size_private(&self) -> U2 {
        (self.nrows_private(),self.ncols_private())
    }

    fn len_private(&self) -> usize {
        let (nrows,ncols)=self.size_private();
        nrows*ncols
    }
}


impl<Row,
     Col : ChangeT<Row,Output = C>,
     C   : Clone> Clone for MatrixGeneric<Row,Col> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Row : ItemT,
     Col : ChangeT<Row,Output = C>,
     C> ItemT for MatrixGeneric<Row,Col> {
    type T=<Row as ItemT>::T;
}

impl<Row,
     Col : ChangeT<Row,Output = C>,
     C   : std::fmt::Debug> std::fmt::Debug for MatrixGeneric<Row,Col> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("MatrixGeneric").field(&self.0).finish()
        }
}

impl<Row,
     Col : ChangeT<Row,Output = C>,
     C   : PartialEq> PartialEq for MatrixGeneric<Row,Col> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
}

impl<Row,
     Col : ChangeT<Row,Output = C>,
     C> MatrixGeneric<Row,Col> {
    pub fn from_col_of_rows(c:C) -> Self {
        Self(c)
    }
}

impl<Row : Display,
     Col : ChangeT<Row,Output=C>,
     C   : ColVector<T=Row>> Display for MatrixGeneric<Row,Col> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        let width=f.width().unwrap_or(8);
        let precision= f.precision().unwrap_or(4);
        let write_row=|f:&mut std::fmt::Formatter, t:&Row|write!(f, "{:+width$.precision$}", t, width=width, precision=precision);
        write!(f,"[")?;
        if let Some(first) = iter.next() {
            write_row(f, first)?;
            for value in iter {
                write!(f, ";\n ")?;
                write_row(f, value)?;
            }
        }
        writeln!(f,"]")?;
        Ok(())
    }
}

impl<F,
     Row : RowVectorAnyConstruct<T=F>,
     Col : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C   : 'static+ColVectorAnyConstruct<T=Row>> container_traits::Get<U2,F> for MatrixGeneric<Row,Col> {
    fn get(&self, (i,j):U2) -> Option<&F> {
        self.0
            .get(i)
            .and_then(|r|r.get(j))
    }
}

impl<F,
     Row  : RowVectorAnyConstruct<T=F>,
     Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C    : 'static+ColVectorAnyConstruct<T=Row>> Iter<F> for MatrixGeneric<Row,Col> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
        container_traits::impl_iter_from_get(self, self.size_private())
    }
}

impl<F,
Row  : RowVectorAnyConstruct<T=F>,
Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
C    : 'static+ColVectorAnyConstruct<T=Row>> IndexedIter<U2,F> for MatrixGeneric<Row,Col> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a F)> where F:'a {
        container_traits::indexed_iter::impl_indexed_iter_from_get(self, self.size_private())
    }
}

impl<F,
Row  : RowVectorAnyConstruct<T=F>,
Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
C    : 'static+ColVectorAnyConstruct<T=Row>> TryIntoElement<U2,F> for MatrixGeneric<Row,Col> {
    fn try_into_element(self,index:U2) -> Option<F> {
        self.0
            .try_into_element(index.0)
            .and_then(|r|r.try_into_element(index.1))
    }
}

impl<F,
Row  : RowVectorAnyConstruct<T=F>,
Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
C    : 'static+ColVectorAnyConstruct<T=Row>> IntoIter<F> for MatrixGeneric<Row,Col> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
        let len=self.len_private();
        self.0
            .into_iterator()
            .map(|r|r.into_iterator())
            .flatten()
            .into_exact_size_iter(len)
    }
}

impl<F,
Row  : RowVectorAnyConstruct<T=F>,
Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
C    : 'static+ColVectorAnyConstruct<T=Row>> IntoIndexedIter<U2,F> for MatrixGeneric<Row,Col> {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(U2,F)> {
        let len=self.len_private();
        self.0
            .into_iterator()
            .enumerate()
            .map(|(i,r)|r.into_iterator().enumerate().map(move |(j,rij)|((i,j),rij)))
            .flatten()
            .into_exact_size_iter(len)
    }
}

impl<F,
     Row  : RowVectorAnyConstruct<T=F>,
     Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C    : 'static+ColVectorAnyConstruct<T=Row>> Size<U2> for MatrixGeneric<Row,Col> {
    fn size(&self) -> U2 {
        self.size_private()
    }
}

impl<F,
     Row  : RowVectorAnyConstruct<T=F>,
     Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C    : 'static+ColVectorAnyConstruct<T=Row>> OCTSize<U2> for MatrixGeneric<Row,Col> {
    const OCTSIZE:Option<U2> = match (Row::OCTSIZE, Col::OCTSIZE) {
        (Some(r),Some(c)) => Some((r,c)),
        _ => None
    };
}

impl<F,
     Row  : RowVectorAnyConstruct<T=F>,
     Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C    : 'static+ColVectorAnyConstruct<T=Row>> NumberOfDegreesOfFreedom<F> for MatrixGeneric<Row,Col> {
    fn ndofs(&self) -> usize {
        self.len_private()    
    }
}

impl<F,
     Row  : RowVectorAnyConstruct<T=F>,
     Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C    : 'static+ColVectorAnyConstruct<T=Row>> Matrix for MatrixGeneric<Row,Col> {
    type Row=Row;
    type Col=Col;

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        self.0
            .into_iterator()
    }

    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        let ncols=self.ncols_private();
        let mut cols:Vec<Vec<Self::T>>=utils::iter::RepeaterN::new(||Vec::new(),ncols).collect();
        for row in self.0.into_iterator() {
            cols.iter_mut()
                .zip(row.into_iterator())
                .for_each(|(c,ri)|c.push(ri));
        }
        let cols=cols.map(|c|Self::Col::any_from_vec(c).unwrap());
        cols.into_exact_size_iter(ncols)
    }
}

impl<Row : Transpose<Output=RowT>,
     Col : Transpose<Output=ColT>+ChangeT<Row>,
     ColT,
     RowT : ChangeT<ColT,Output=C2>,
     C2 : ColVectorAnyConstruct<T=ColT>>
    Transpose for MatrixGeneric<Row,Col>
    where                Self : Matrix<Row=Row,  Col=Col>, 
     MatrixGeneric<ColT,RowT> : Matrix<Row=ColT, Col=RowT>{
    type Output=MatrixGeneric<ColT,RowT>;
    fn transpose(self) -> Self::Output {
        let cols_transposed=
            self.into_cols()
                .map(|c|c.transpose())
                .collect();
        MatrixGeneric::from_col_of_rows(C2::any_from_vec(cols_transposed).unwrap())
    }
}

impl<F,
     Row : RowVectorAnyConstruct<T=F>,
     Col : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C   : ColVectorAnyConstruct<T=Row>> TryAccept<U2,F,MatrixConstructError> for MatrixGeneric<Row,Col> where Self : Matrix<T=F,Row=Row,Col=Col> {    
    
    fn try_accept<'a>(size:U2, f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F : 'a {
        let (nrows,ncols)=id.size();
        for i in 0..nrows {
            Row::try_accept(InstanceStructureDescriptor::Size(ncols),|j|f((i,j)))
            .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)?;
        }
        for j in 0..ncols {
            Col::try_accept(InstanceStructureDescriptor::Size(nrows),|i|f((i,j)))
            .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)?;
        }
        Ok(())
    }
}

impl<F,
     Row : RowVectorAnyConstruct<T=F>,
     Col : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C   : ColVectorAnyConstruct<T=Row>> TryFromFn<U2,F,MatrixConstructError> for MatrixGeneric<Row,Col> where Self : Matrix<T=F,Row=Row,Col=Col> {
    fn try_from_fn(size:U2, f:impl Fn(U2) -> F) -> Result<Self,MatrixConstructError> {
        let (nrows,ncols)=id.size();
        let idc=InstanceStructureDescriptor::Size(nrows);
        let idr=||InstanceStructureDescriptor::Size(ncols);
        let orow=|i:usize|Row::try_from_fn(idr(),|j|f((i,j)));
        for i in 0..nrows {
            orow(i)
                .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)?;
        }
        C::try_from_fn(idc,|i| orow(i).unwrap())
            .map(|c|Self(c))
            .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
    }
}

impl<F,
     Row : RowVectorAnyConstruct<T=F>,
     Col : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C   : ColVectorAnyConstruct<T=Row>> AnyFromIterator<F,MatrixConstructError> for MatrixGeneric<Row,Col> where Self : Matrix<T=F,Row=Row,Col=Col> {
    fn any_take_away<I:    Iterator<Item=F>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,MatrixConstructError> {
        let sz=Self::size_from_oref(oref);
        let len=sz.0*sz.1;
        let vs:Vec<F>=utils::iter::next_chunk_dyn(iter,len)
                        .map_err(|e|LenTooSmallError::new(len,e.len()))?;
        let mut iter=vs.into_iter();
        let orowref:Option<&Row>=oref.and_then(|r|r.0.get(0));
        let rows=
            std::iter::repeat_with(||Row::any_take_away(orowref,& mut iter).ok().unwrap());
        Ok(Self(C::any_from_iter(oref.map(|s|&s.0),rows).unwrap()))
    }

    container_traits::any_from_iter_impl!(F,MatrixConstructError);
}

impl<F,
     Row : RowVectorAnyConstruct<T=F>,
     Col : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C   : ColVectorAnyConstruct<T=Row>> MatrixTryConstruct for MatrixGeneric<Row,Col> where Self : Matrix<T=F,Row=Row,Col=Col> {
    fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError> {
        let rows:Vec<Self::Row>=rows.collect();
        if let Err(CommonLengthError::NotAllHaveSameLength(vs)) = Self::Row::try_common_length(rows.iter()) {
            return Err(MatrixConstructError::RowsDoNotHaveTheSameLength(vs));
        }
        C::any_from_iter(None, rows.into_iter())
            .map(|c|Self(c))
            .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
    }
}

impl<F,
     Row : RowVectorMut<T=F>,
     Col : ColVectorMut<T=F>+ChangeT<Row,Output=C>,
     C   : 'static+ColVectorMut<T=Row>> container_traits::IterMut<F> for MatrixGeneric<Row,Col> 
    where Self : Matrix<T=F,Row=Row> {
    fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut F> where F : 'a {
        let len=self.len_private();
         self.0
             .iter_mut()
             .map(|r|r.iter_mut())
             .flatten()
             .into_exact_size_iter(len)
    }
}

impl<F,F2,
     Row : RowVectorAnyConstruct<T=F>+AnyMap<F,F2,LinearContainerConstructError,Output=Row2>,
     Row2: RowVectorAnyConstruct<T=F2>,
     Col : ColVectorAnyConstruct<T=F>+AnyMap<F,F2,LinearContainerConstructError,Output=Col2>+ChangeT<Row,Output=C>,
     Col2: ColVectorAnyConstruct<T=F>+ChangeT<Row2,Output=C2>,
     C   : ColVectorAnyConstruct<T=Row>+AnyMap<Row,Row2,LinearContainerConstructError,Output=C2>,
     C2  : ColVectorAnyConstruct<T=Row2>> AnyMap<F,F2,MatrixConstructError> for MatrixGeneric<Row,Col> where Self : Matrix<T=F,Row=Row,Col=Col> {    
    type Output=MatrixGeneric<Row2,Col2>;
    fn any_map(self,f:impl Fn(F) -> F2) -> Result<Self::Output,MatrixConstructError> {
        Ok(MatrixGeneric(
            self.0
                .any_map(|r:Row|r.any_map(&f).ok().unwrap())
                .unwrap()))
    }
}

impl<F, FOut,
     Row    : RowVectorAnyConstruct<T=F>+Map<F,FOut,Output=RowOut>,
     RowOut : RowVectorAnyConstruct<T=FOut>,
     Col    : ColVectorAnyConstruct<T=F>+Map<F,FOut,Output=ColOut>+ChangeT<Row,Output=C>,
     ColOut : ColVectorAnyConstruct<T=FOut>+ChangeT<RowOut,Output=COut>,
     C      : 'static+ColVectorAnyConstruct<T=Row>,
     COut   : 'static+ColVectorAnyConstruct<T=RowOut>> Map<F,FOut> for MatrixGeneric<Row,Col> {
        type Output=MatrixGeneric<RowOut,ColOut>;
     
        fn map(self, f:impl Fn(F) -> FOut) -> Self::Output {
             matrix_traits::matrices::matrix::impl_map(self,f)
        }
    // 
}

impl<F   : 'static,
     Row : RowVectorMut<T=F>,
     Col : ColVectorMut<T=F>+ChangeT<Row,Output=C>,
     C   : 'static+ColVectorMut<T=Row>> container_traits::GetMut<U2,F> for MatrixGeneric<Row,Col> 
    where Self : Matrix<T=F,Row=Row> {
    fn get_mut(& mut self, (i,j):U2) -> Option<& mut F> {
        self.0
            .get_mut(i)
            .and_then(|r|r.get_mut(j))
    }
}

impl<F, F2,
     Row   : RowVector<T=F>+ChangeT<F2,Output=RowF2>,
     Col   : ColVector<T=F>+ChangeT<F2,Output=ColF2>+ChangeT<Row>,
     RowF2 : RowVector<T=F2>,
     ColF2 : ColVector<T=F2>+ChangeT<RowF2>> ChangeT<F2> for MatrixGeneric<Row,Col> {
        type Output = MatrixGeneric<RowF2,ColF2>;
    }


impl<Row,Col:ChangeT<Row>> AsBaseMatrix for MatrixGeneric<Row,Col> where Self:Matrix {
    type Output = Self;
    fn base_matrix(&self) -> &Self {
        self
    }
}

impl<Row,Col:ChangeT<Row>> IntoBaseMatrix for MatrixGeneric<Row,Col> where Self:Matrix {
    type Output = Self;
    fn into_base_matrix(self) -> Self {
        self
    }
}

impl<F,
     Row : RowVectorConstruct<T=F>,
     Col : ColVectorConstruct<T=F>+ChangeT<Row,Output=C>,
     C   : 'static+ColVectorConstruct<T=Row>> FromFn<U2,F> for MatrixGeneric<Row,Col> {
        fn from_fn(size:U2,f:impl Fn(U2) -> F) -> Self {
            let (nrows,ncols)=id.size();
            Self(C::from_fn(InstanceStructureDescriptor::Size(nrows),
                |i|Row::from_fn(InstanceStructureDescriptor::Size(ncols),|j|f((i,j)))))
        }
}

impl<F   : Zero+One,
     Row : RowVectorConstruct<T=F>,
     Col : ColVectorConstruct<T=F>+ChangeT<Row,Output=C>,
     C   : 'static+ColVectorConstruct<T=Row>> Identity for MatrixGeneric<Row,Col> {
    fn identity(n:usize) -> Self {
        Self::from_fn(
            InstanceStructureDescriptor::Size((n,n)),
            |(i,j)|utils::kron_delta(i, j))
    }
}

impl<F,
     Row : Index<usize,Output=F>,
     Col : ChangeT<Row,Output=C>,
     C   : 'static+Index<usize,Output=Row>> Index<U2> for MatrixGeneric<Row,Col> {
    type Output=F;
    fn index(&self, index: U2) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<F,
     Row : IndexMut<usize,Output=F>,
     Col : ChangeT<Row,Output=C>,
     C   : 'static+IndexMut<usize,Output=Row>> IndexMut<U2> for MatrixGeneric<Row,Col> {
    fn index_mut(& mut self, index: U2) -> & mut Self::Output {
        & mut self.0[index.0][index.1]
    }
}

// impl<Row : TryFromSuperContainer<usize, Row2>,
//      Row2: Container<usize>,
//      Col : ChangeT<Row,Output=C>,
//      C   : 'static+AnyFromIterator<Row>,
//      M2  : Matrix<Row=Row2>> TryFromSuperContainer<U2, M2, MatrixConstructError> for MatrixGeneric<Row,Col> {
    
//     fn try_from_super(m:M2, start:U2, size:U2) -> Result<Self, MatrixConstructError> {
//         LenTooSmallError::try_new(start.0+size.0, m.nrows())?;

//         let rows:Vec<Row>=
//             m.into_rows()
//              .skip(start.0)
//              .take(size.0)
//              .map(|r|Row::try_from_super(r, start.1, size.1))
//              .collect()?;
//         C::any_from_iter(None,rows)
//             .map(|c|Self(c))
//             .map_err(|_|OtherDimensionMismatchError.into())
//            // Ok(c) => Ok(Self(c)),
//            // Err(_) => Err(MatrixConstructError::DimensionMismatch(DimensionMismatchError::Other(OtherDimensionMismatchError)))
//         }
// }