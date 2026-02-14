use algebra::unit_vector::Unit;
use container_traits::{Get, IndexOutOfBoundsError, Inner, IntoInner, IntoIter, IntoIterIndexed, IsEmpty, ItemT, Iter, IterIndexed, Map, NumberOfDegreesOfFreedom, OCTSize, Size, TryIntoElement};
use algebra_traits::*;
use std::ops::Mul;
use matrix_wrappers::{Hermitian, Symmetric, orthogonality::*};

use utils::kronecker_delta::kron_delta;

use matrix_traits::*;
use cachingmap::CachingMap;

type U2=(usize,usize);

#[derive(Clone,
         Debug)]
pub struct HouseholderTrafoGeneric<Col:ColVectorView> {
    u:Unit<Col>, // trafo is x -> x - 2 * <x,u> *u
    a:CachingMap<U2,Col::T> // cashed matrix values
}

impl<Col:ColVectorView> HouseholderTrafoGeneric<Col> {
    pub fn n(&self) -> usize {
        self.u
            .as_ref()
            .len()
    }
}

impl<Col:ColVectorView+Clone+Norm> HouseholderTrafoGeneric<Col> where Col::NormT : RealNumber {
    pub fn try_new(v:Col) -> Result<Self, Col> {
        Unit::try_new(v)
         .map(|u|Self{u, a:CachingMap::new()})
    }
}

// finds householder trafo that transforms a to b up to a scalar factor
impl<Col : Clone
          +ScalarVector<T=F>
          +ColVectorView<T=F>,
      F  : Clone+Scalar> HouseholderTrafoGeneric<Col> {
    pub fn try_froma2b(a:Unit<Col>, b:Unit<Col>) -> Option<Self> {
        let sc_prod=a.clone().try_scalar_product(b.clone())?;
        let fac=-sc_prod.try_normalize().map(|r|r.1).unwrap_or(F::one());
        b.into_inner()
         .try_div(fac).ok()?
         .try_sub(a.into_inner()).ok()?
         .try_divide_by_norm().ok().map(|r|r.1)
         .and_then(|s|Self::try_new(s).ok())
    }
}

#[cfg(test)]
use nalgebra::{SVector,SMatrix};


#[test]
fn test_try_froma2b() {
    use container_traits::for_static::EZ;
    let a:Unit<SVector<f64,3>>=Unit::ez();
    let b=SVector::<f64,3>::new(1.0, 0.0, -1.0);
    let b:SVector<f64,3>=b.try_divide_by_norm().ok().unwrap().1;
    let b=Unit::try_new(b).ok().unwrap();
    let res=HouseholderTrafoGeneric::try_froma2b(a, b);
    assert!(res.is_some());
}


impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> Get<U2,F> for HouseholderTrafoGeneric<Col> {    
    fn get(&self, (i,j):U2) -> Result<&F,IndexOutOfBoundsError<U2>> {
        let n=self.n();
        IndexOutOfBoundsError::try_new(&(n,n),&(i,j))?;
        let r=self.a.get(&(i,j));
        if let Some(bf)=r {
            return Ok(bf.as_ref());
        }
        let f=|&(ii,jj)|{
            let get=|iii:usize|self.u.as_ref().get(iii).cloned();
            let ui=get(ii).unwrap();
            let uj=get(jj).unwrap();
            let ujh=uj.conjugate();
            kron_delta::<usize,F>(ii, jj) - (ui*ujh).muli(2)
        };
        Ok(*self.a.cache((i,j),f(&(i,j))))
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> Iter<F> for HouseholderTrafoGeneric<Col> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
        container_traits::iter::impl_iter_from_get(self, (self.n(),self.n()))
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> IterIndexed<U2,F> for HouseholderTrafoGeneric<Col> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a F)> where F:'a {
        container_traits::iter_indexed::impl_iter_indexed_from_get(self, (self.n(),self.n()))
    }
}

impl<F   : Scalar,
     Col : ColVectorView<T=F>> ItemT for HouseholderTrafoGeneric<Col> {
    type T=F;
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> TryIntoElement<U2,F> for HouseholderTrafoGeneric<Col> {
    fn try_into_element(self,index:U2) -> Result<F,IndexOutOfBoundsError<U2>> {
        self.get(index)
            .cloned()
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> IntoIter<F> for HouseholderTrafoGeneric<Col> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
        let vs:Vec<F>=
            self.iter()
                .cloned()
                .collect();
        vs.into_iter()
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> IntoIterIndexed<U2,F> for HouseholderTrafoGeneric<Col> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(U2,F)> {
        let vs:Vec<(U2,F)>=
            self.iter_indexed()
                .map(|(i,f)|(i,f.clone()))
                .collect();
        vs.into_iter()
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> Size<U2> for HouseholderTrafoGeneric<Col> {
    fn size(&self) -> U2 {
        let n=self.n();
        (n,n)
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>+OCTSize<usize>> OCTSize<U2> for HouseholderTrafoGeneric<Col> {
    const OCTSIZE:Option<U2> = match Col::OCTSIZE {
        Some(n) => Some((n,n)),
        None => None
    };
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> NumberOfDegreesOfFreedom<F> for HouseholderTrafoGeneric<Col> {
    fn ndofs(&self) -> usize {
        let n=self.n();
        n*n
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorView<T=F>> IsEmpty for HouseholderTrafoGeneric<Col> {
        fn is_empty(&self) -> bool {
            self.n() == 0
        }
}


impl<F   : Clone+Scalar,
     Col : ColVectorTryConstruct<T=F>+Transpose<Output=Row>,
     Row : RowVectorTryConstruct<T=F>+Transpose<Output=Col>> MatrixView for HouseholderTrafoGeneric<Col> {

        type RowView<'a>=Row where Self : 'a;
        type ColView<'a>=Col where Self : 'a;

        fn nrows(&self) -> usize {
            self.n()
        }

        fn ncols(&self) -> usize {
            self.n()
        }
        
        fn try_row_view<'a>(&'a self, i:usize) -> Result<Self::RowView<'a>,IndexOutOfBoundsError<usize>> {
            self.try_row(i)
        }
        
        fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,IndexOutOfBoundsError<usize>> {
            self.try_col(j)
        }
}

impl<F   : Clone+Scalar,
     Row : RowVectorTryConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorTryConstruct<T=F>+Transpose<Output=Row>> Matrix for HouseholderTrafoGeneric<Col> {

        type Row=Row;

        type Col=Col;

        fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
            (0..self.n()).map(move |i|self.try_row(i).unwrap())
        }

        fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
            (0..self.n()).map(move |i|self.try_col(i).unwrap())
        }
}

impl<Col : ColVector> IntoBaseMatrix for HouseholderTrafoGeneric<Col> where Self : Matrix {
    type Output = Self;
    fn into_base_matrix(self) -> Self {
        self
    }
}

impl<Col : ColVectorView> AsBaseMatrix for HouseholderTrafoGeneric<Col> where Self : Matrix {
    type Output = Self;
    fn base_matrix(&self) -> &Self {
        self
    }
}

impl<Col : ColVectorView> MatrixViewNotWide for HouseholderTrafoGeneric<Col> where Self : Matrix {}
impl<Col : ColVectorView> MatrixViewNotTall for HouseholderTrafoGeneric<Col> where Self : Matrix {}
impl<Col : ColVectorView> MatrixViewSquare  for HouseholderTrafoGeneric<Col> where Self : Matrix {}

impl<F:RealNumber,
     Col:ScalarVector<T=F>+ColVector<T=F>> HouseholderTrafoGeneric<Col> where Self : Matrix<T=F> {
    pub fn try_into_orthogonal_matrix<M:Clone+PartialEq+AlgebraMatrix+MatrixSquareTryConstruct<T=F>+Transpose<Output=M>>(self) -> Option<Orthogonal<Symmetric<M>>>
    where M::Col : ScalarVector<T = F> {
        Orthogonal::try_from_matrix(self).ok()
    }
}

impl<F:ComplexNumber,
     Col:ScalarVector<T=F>+ColVector<T=F>> HouseholderTrafoGeneric<Col> where Self : Matrix<T=F> {
    pub fn try_into_unitary_matrix<M:Clone+AlgebraMatrix+MatrixSquareTryConstruct<T=F>+Transpose<Output=M>>(self) -> Option<Unitary<M>>
    where M::Col : ScalarVector<T = F> {
        Unitary::try_from_matrix(self).ok()
    }
}

impl<F:ComplexNumber,
     Col:ScalarVector<T=F>+ColVector<T=F>> HouseholderTrafoGeneric<Col> where Self : Matrix<T=F> {
    pub fn try_into_unitary_hermitian_matrix<M:Clone+PartialEq+AlgebraMatrix+MatrixSquareTryConstruct<T=F>+Transpose<Output=M>>(self) -> Option<Unitary<Hermitian<M>>>
    where M::Col : ScalarVector<T = F> {
        Unitary::try_from_matrix(self).ok()
    }
}


impl<F : Scalar+Mul<V,Output=V>,
     V : Vectorspace<F>+MulI,
     ColF : Clone+ColVector<T=F>+Map<F,V,Output=ColV>+Transpose<Output=ColFH>,
     ColFH: Conjugate<Output=ColFH>+RowVector<T=F>,
     ColV : Clone+ColVector<T=V>+ClosedTrySub> TryMatrixVectorProduct<ColV> for HouseholderTrafoGeneric<ColF>
    where Self : Matrix {
    type Output=ColV;
    fn try_matrix_vector_product(&self, rhs:&ColV) -> Result<ColV,VectorConstructError> {
        MatrixCanNotBeMultipliedWithVectorError::try_new(self.n(), rhs.len())?;
        let col_f:&ColF=self.u.inner();
        let sp= V::any_linear_combination(col_f.conjugate_transpose(),rhs.clone()).unwrap();
        let fac=sp.muli(2);
        Ok(rhs.clone().try_sub(col_f.clone().map(|f:F|f*fac.clone())).ok().unwrap())
    }
}


impl<F    : Scalar+Mul<V,Output=V>,
     V    : Vectorspace<F>+Clone,
     ColF : ColVectorTryConstruct<T=F>+Clone+Map<F,V,Output=M::Col>,
     M    : MatrixTryConstruct<T=V>> TryMatrixMatrixProduct<M> for HouseholderTrafoGeneric<ColF>
    where Self : TryMatrixVectorProduct<M::Col,Output=M::Col> {
    type Output=M;
    fn try_matrix_matrix_product(&self, rhs:&M) -> Result<M,MatrixConstructError> {
        let lhs_dims=self.matrix_dimensions();
        let rhs_dims=rhs.matrix_dimensions();
        MatricesCanNotBeMultipliedError::try_new(&lhs_dims,&rhs_dims)?;
        let out=M::try_from_cols(
                rhs.cols()
                   .map(|col|self.try_matrix_vector_product(&col).unwrap()))?;
        let out_dims=out.matrix_dimensions();
        assert_eq!(out_dims.0, lhs_dims.0);
        assert_eq!(out_dims.1, rhs_dims.1);
        Ok(out)
    }
}


#[cfg(test)]
use algebra_traits::Norm;

#[cfg(test)]
use container_traits::for_static::EX;


#[test]
fn test_froma2b() {
    let ex=Unit::<SVector::<f64,2>>::ex();
    let b=Unit::<SVector::<f64,2>>::try_new(SVector::from([0.8,0.6])).unwrap();
    let hh=HouseholderTrafoGeneric::<SVector<f64,2>>::try_froma2b(ex, b).unwrap();
    // let om=hh.clone().try_into_orthogonal_matrix().unwrap();
    let hh_m=SMatrix::<f64,2,2>::try_from_matrix(hh).ok().unwrap();
    println!("{:?}", hh_m);
    let rhs:SMatrix<f64,2,2>=nalgebra::matrix![-0.8, -0.6;-0.6, 0.8].into();
    assert!(rhs.is_close_to(&hh_m));
}



#[cfg(test)]
fn are_collinear<F:Scalar+nalgebra::Scalar, const N:usize>(a:SVector<F,N>, b:SVector<F,N>) -> bool
where SVector<F,2> : Norm<NormT=F::RealType> {
    let an=a.norm().into_signed();
    let bn=b.norm().into_signed();
    let diff=a.scalar_mul(&bn)-b.scalar_mul(&an);
    diff.into_norm()
        .is_small()
}


#[test]
fn test_froma2b_complex() {
    use algebra::complex::c64;
    use matrix_traits::MatrixVectorProduct;
    let ex=Unit::<SVector::<c64,2>>::ex();
    let onedivsqrt2=1.0/2_f64.sqrt();
    let bs: Vec<SVector<c64,2>>=vec![
        SVector::from([c64::from(0.8),             c64::new(0.0, 0.6)]),
        SVector::from([c64::i(),                   c64::from(0.0)]),
        SVector::from([c64::new(0.0, onedivsqrt2), c64::from(onedivsqrt2)]),
    ];
    for b in bs {
        let b=Unit::<SVector::<c64,2>>::try_new(b).unwrap();
        let hh=HouseholderTrafoGeneric::<SVector<c64,2>>::try_froma2b(ex.clone(), b.clone()).unwrap();
        let bv=b.inner();
        let e0v=ex.inner();
        let hhu=hh.try_into_unitary_matrix::<SMatrix<c64,2,2>>().unwrap();
        let hb=hhu.clone().matrix_vector_product(bv);
        let he0=hhu.clone().matrix_vector_product(e0v);
        assert!(are_collinear(hb, e0v.clone()));
        assert!(are_collinear(he0, bv.clone()));
    }
}