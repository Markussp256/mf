use algebra_traits::*;
use algebra::unit_vector::Unit;
use container_traits::{AnyFromContainer, Get, IndexedIter, IntoIndexedIter, IntoInner, IntoIter, ItemT, Iter, NumberOfDegreesOfFreedom, OCTSize, Size, TryIntoElement};
use algebra_traits::ScalarVector;
use matrix_wrappers::{Hermitian, Symmetric, orthogonality::*};

use utils::kronecker_delta::kron_delta;

use matrix_traits::{AlgebraMatrix, AsBaseMatrix, IntoBaseMatrix, MatrixSquare, MatrixNotWide, MatrixNotTall, MatrixSquareTryConstruct, Matrix,RowVectorAnyConstruct,ColVector,ColVectorAnyConstruct, Transpose, TryFromMatrix};
use cachingmap::CachingMap;

type U2=(usize,usize);

#[derive(Clone,
         Debug)]
pub struct HouseholderTrafoGeneric<Col:ColVector>{
    u:Unit<Col>, // trafo is x -> x - 2 * <x,u> *u
    a:CachingMap<U2,Col::T> // cashed matrix values
}

impl<Col:ColVector> HouseholderTrafoGeneric<Col> {
    pub fn n(&self) -> usize {
        self.u
            .as_ref()
            .len()
    }
}

impl<Col:ColVector+Clone+Norm> HouseholderTrafoGeneric<Col> where Col::NormT : RealNumber {
    pub fn try_new(v:Col) -> Result<Self, Col> {
        Unit::try_new(v)
         .map(|u|Self{u, a:CachingMap::new()})
    }
}

// finds householder trafo that transforms a to b up to a scalar factor
impl<Col : Clone
          +ScalarVector<T=F>
          +ColVector<T=F>,
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

#[cfg(test)]
use container_traits::for_static::{X, Z};

#[test]
fn test_try_froma2b() {
    let a:Unit<SVector<f64,3>>=Unit::ez();
    let b=SVector::<f64,3>::new(1.0, 0.0, -1.0);
    let b:SVector<f64,3>=b.try_divide_by_norm().ok().unwrap().1;
    let b=Unit::try_new(b).ok().unwrap();
    let res=HouseholderTrafoGeneric::try_froma2b(a, b);
    assert!(res.is_some());
}


impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> Get<U2,F> for HouseholderTrafoGeneric<Col> {    
    fn get(&self, (i,j):U2) -> Option<&F> {
        let r=self.a.get(&(i,j));
        if r.is_some() {
            return r.map(|b|b.as_ref());
        }
        let n=self.n();
        if i >= n || j >= n {
            return None;
        }
        let f=|&(ii,jj)|{
            let get=|iii:usize|self.u.as_ref().get(iii).cloned();
            let ui=get(ii).unwrap();
            let uj=get(jj).unwrap();
            let ujh=uj.conjugate();
            kron_delta::<usize,F>(ii, jj) - (ui*ujh).muli(2)
        };
        Some(*self.a.cache((i,j),f(&(i,j))))
    }
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> Iter<F> for HouseholderTrafoGeneric<Col> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a F> where F:'a {
        container_traits::iter::impl_iter_from_get(self, (self.n(),self.n()))
    }
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> IndexedIter<U2,F> for HouseholderTrafoGeneric<Col> {
    fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a F)> where F:'a {
        container_traits::indexed_iter::impl_indexed_iter_from_get(self, (self.n(),self.n()))
    }
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> ItemT for HouseholderTrafoGeneric<Col> {
    type T=F;
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> TryIntoElement<U2,F> for HouseholderTrafoGeneric<Col> {
    fn try_into_element(self,index:U2) -> Option<F> {
        self.get(index)
            .cloned()
    }
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> IntoIter<F> for HouseholderTrafoGeneric<Col> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
        let vs:Vec<F>=
            self.iter()
                .cloned()
                .collect();
        vs.into_iter()
    }
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> IntoIndexedIter<U2,F> for HouseholderTrafoGeneric<Col> {
    fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(U2,F)> {
        let vs:Vec<(U2,F)>=
            self.indexed_iter()
                .map(|(i,f)|(i,f.clone()))
                .collect();
        vs.into_iter()
    }
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> Size<U2> for HouseholderTrafoGeneric<Col> {
    fn size(&self) -> U2 {
        let n=self.n();
        (n,n)
    }
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> OCTSize<U2> for HouseholderTrafoGeneric<Col> {
    const OCTSIZE:Option<U2> = match Row::OCTSIZE {
        Some(n) => Some((n,n)),
        None => None
    };
}

impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> NumberOfDegreesOfFreedom<F> for HouseholderTrafoGeneric<Col> {
    fn ndofs(&self) -> usize {
        let n=self.n();
        n*n
    }
}


impl<F   : Clone+Scalar,
     Row : RowVectorAnyConstruct<T=F>+Transpose<Output=Col>,
     Col : ColVectorAnyConstruct<T=F>+Transpose<Output=Row>> Matrix for HouseholderTrafoGeneric<Col> {
        
        type Row=Row;
     
        type Col=Col;
     
        fn nrows(&self) -> usize {
            self.n()
        }
     
        fn ncols(&self) -> usize {
            self.n()
        }
     
        fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
            let n=self.n();
            let row=move |i|Row::any_from_iter(None,(0..n).map(|j|self.get((i,j)).unwrap().clone())).unwrap();
            (0..n).map(row)
        }
     
        fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
            let n=self.n();
            let col=move |j|Col::any_from_iter(None,(0..n).map(|i|self.get((i,j)).unwrap().clone())).unwrap();
            (0..n).map(col)
        }
}

impl<Col : ColVector> IntoBaseMatrix for HouseholderTrafoGeneric<Col> where Self : Matrix {
    type Output = Self;
    fn into_base_matrix(self) -> Self {
        self
    }
}

impl<Col : ColVector> AsBaseMatrix for HouseholderTrafoGeneric<Col> where Self : Matrix {
    type Output = Self;
    fn base_matrix(&self) -> &Self {
        self
    }
}

impl<Col : ColVector> MatrixNotWide for HouseholderTrafoGeneric<Col> where Self : Matrix {}
impl<Col : ColVector> MatrixNotTall for HouseholderTrafoGeneric<Col> where Self : Matrix {}
impl<Col : ColVector> MatrixSquare  for HouseholderTrafoGeneric<Col> where Self : Matrix {}

impl<F:RealNumber,
     Col:ScalarVector<T=F>+ColVector<T=F>> HouseholderTrafoGeneric<Col> where Self : Matrix<T=F> {
    pub fn try_into_orthogonal_matrix<M:Clone+PartialEq+AlgebraMatrix+MatrixSquareTryConstruct<T=F>+Transpose<Output=M>>(self) -> Option<Orthogonal<Symmetric<M>>>
    where M::Col : ScalarVector<T = F> {
        Orthogonal::try_from_matrix(self).ok()
    }
}

impl<F:ComplexNumber,
     Col:ScalarVector<T=F>+ColVector<T=F>> HouseholderTrafoGeneric<Col> where Self : Matrix<T=F> {
    pub fn try_into_unitary_matrix<M:Clone+PartialEq+AlgebraMatrix+MatrixSquareTryConstruct<T=F>+Transpose<Output=M>>(self) -> Option<Unitary<Hermitian<M>>>
    where M::Col : ScalarVector<T = F> {
        Unitary::try_from_matrix(self).ok()
    }
}

impl<F   : Clone+Scalar,
     Col : ColVectorAnyConstruct<T=F>
          +ScalarVector<T=F>
          +Clone> HouseholderTrafoGeneric<Col> {
    pub fn any_vector_mul<Col2 : ColVector<T=F>>(self, rhs:Col2) -> Option<Col> {
        let col=self.u.into_inner();
        let rhs:Col=Col::any_from_container(rhs).ok()?;
        let sp=col.clone().try_scalar_product(rhs.clone())?;
        let fac=sp.muli(2);
        rhs.try_sub(col.map(|v:F|v*fac.clone())).ok()
    }
}


// impl<F    : Clone+Scalar,
//      Row  : RowVectorTryConstruct<T=F>,
//      Col  : ColVectorTryConstruct<T=F>
//            +TryScalarproduct<ScProdT =F>
//            +TrySub<Output=Col>
//            +Clone
//            +ChangeT<Row,Output=C>
//            +ChangeT<F,Output=Col>,
//      C    : ColVectorTryConstruct<T=Row>,
//      MRhs : Matrix<F=F,Row=Row>> TryMatrixMatrixProduct<MRhs> for HouseholderTrafoGeneric<Col>
//     where Self : Matrix {
//         fn try_matrix_matrix_product<M:MatrixTryConstruct<Col=Col>>(self, rhs:MRhs) -> Option<M> {
//             if self.n() != rhs.nrows() {
//                 return None;
//             }
//             MatrixGeneric::try_from_cols(
//                 rhs.into_cols()
//                    .map(|c|self.clone().any_vector_mul(c).unwrap())).ok()
//         }
// }
// impl<Col:ColVector,M> TryMatrixMatrixProduct<M> for HouseholderTrafoGeneric<Col> where Self : AnyGeneralizedMatrixProduct<M,Output=M> {}


// impl<F   : Clone+Scalar,
//      Col : ColVectorTryConstruct<T=F>+ChangeT<F,Output=Col>+Clone+Norm<NormT=F::RealType>+TrySub<Output=Col>+TryScalarproduct<ScProdT = F>+ScalarMul<F>,
//      > HouseholderTrafoGeneric<Col> {
//     pub fn vector_mul(self, rhs:Col) -> Option<Col> {
//         let col=self.0.into_inner();
//         let sp=col.clone().try_scalar_product(rhs.clone())?;
//         let fac=sp.muli(2);
//         rhs.try_sub(col.colvector_map(|v|v*fac.clone()))
//     }

//     pub fn matrix_mul<M: MatrixTryConstruct<F=F,Col=Col>>(self,rhs:M) -> Option<M> {
//         (self.0.as_ref().len() == rhs.nrows()).then(||
//             M::try_from_cols(
//                 rhs.into_cols()
//                    .map(|c|self.clone().vector_mul(c).unwrap())).ok().unwrap())
//     }
// }


#[cfg(test)]
use algebra_traits::Norm;



#[test]
fn test_froma2b() {
    let ex=Unit::<SVector::<f64,2>>::ex();
    let b=Unit::<SVector::<f64,2>>::try_new(SVector::from([0.8,0.6])).unwrap();
    let hh=HouseholderTrafoGeneric::<SVector<f64,2>>::try_froma2b(ex, b).unwrap();
    // let om=hh.clone().try_into_orthogonal_matrix().unwrap();
    let hh_m=SMatrix::<f64,2,2>::try_from_matrix(hh).ok().unwrap();
    println!("{:?}", hh_m);
    let rhs:SMatrix<f64,2,2>=nalgebra::matrix![-0.8, -0.6;-0.6, 0.8].into();
    assert!(rhs.is_close_to(hh_m));
}



#[cfg(test)]
fn are_collinear<F:Scalar+nalgebra::Scalar, const N:usize>(a:SVector<F,N>, b:SVector<F,N>) -> bool
where SVector<F,2> : Norm<NormT=F::RealType> {
    let an=a.clone().norm();
    let bn=b.clone().norm();
    a.scalar_product(b)
     .norm()
     .is_close_to(an*bn)
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
        let bv=b.clone().into_inner();
        let e0v=ex.clone().into_inner();
        let hhu=hh.try_into_unitary_matrix::<SMatrix<c64,2,2>>().unwrap();
        let hb=hhu.clone().matrix_vector_product(bv.clone());
        let he0=hhu.clone().matrix_vector_product(e0v.clone());
        assert!(are_collinear(hb, e0v));
        assert!(are_collinear(he0, bv));
    }
}