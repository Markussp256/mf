


pub type HomogeneousMatrix<F,const N:usize>=Homogeneous<matrix::Matrix<F,N,N>>;

pub type HomogeneousMatrix3<F>=HomogeneousMatrix<F,3>;
pub type HomogeneousMatrix4<F>=HomogeneousMatrix<F,4>;

// pub type [<$uc Matrix>]<F,const N:usize>=$uc<matrix::Matrix<F,N,N>>;
// pub type [<$uc MatrixDyn>]<F>=$uc<SquareMatrixDyn<F>>;

// pub type   [<Special $uc Matrix>]<F,const N:usize>=[<Special $uc>]<matrix::Matrix<F,N,N>>;
// pub type   [<Special $uc MatrixDyn>]<F>=[<Special $uc>]<SquareMatrixDyn<F>>;

pub type StiefelMatrixDyn<F>=Stiefel<MatrixDyn<F>>;
pub type StiefelMatrix<F,const M:usize, const N:usize>=Stiefel<matrix::Matrix<F,M,N>>;

pub type SquareStiefelMatrixDyn<F>            = Stiefel<SquareMatrixDyn<F>>;
pub type SquareStiefelMatrix<F,const M:usize> = Stiefel<Matrix<F,M,M>>;

// shaped
pub type NotTallMatrixDyn<F>=NotTall<MatrixDyn<F>>;
pub type NotTallMatrix<F,const M:usize,const N:usize>=NotTall<matrix::Matrix<F,M,N>>;

pub type NotWideMatrixDyn<F>=NotWide<MatrixDyn<F>>;
pub type NotWideMatrix<F,const M:usize,const N:usize>=NotWide<matrix::Matrix<F,M,N>>;

pub type SquareMatrixDyn<F>=Square<MatrixDyn<F>>;
// Matrix<F,N,N> does not need to be wrapped, its already clear its square
pub type SquareMatrix<F,const N:usize>=matrix::Matrix<F,N,N>;

#[test]
fn test_from_diagonal() {
    use container_traits::Get;
    use matrix::DiagonalMatrixDyn;
    let a00 = 1;
    let a11 = 2;
    let m:Square<_> = DiagonalMatrixDyn::<i32>::new(vec![a00, a11]).into();
    assert_eq!(m.get((0, 0)), Some(&a00));
    assert_eq!(m.get((1, 1)), Some(&a11));
}

pub type TallMatrixDyn<F>=Tall<MatrixDyn<F>>;
pub type TallMatrix<F,const M:usize,const N:usize>=Tall<matrix::Matrix<F,M,N>>;

pub type WideMatrixDyn<F>=Wide<MatrixDyn<F>>;
pub type WideMatrix<F,const M:usize,const N:usize>=Wide<matrix::Matrix<F,M,N>>;

// symmetry

pub type AntiHermitianMatrix<F,const N:usize> = AntiHermitian<matrix::Matrix<F,N,N>>;
pub type AntiHermitianMatrixDyn<F>            = AntiHermitian<SquareMatrixDyn<F>>;

pub type HermitianMatrixDyn<F>            = Hermitian<SquareMatrixDyn<F>>;
pub type HermitianMatrix<F,const N:usize> = Hermitian<matrix::Matrix<F,N,N>>;


pub type SkewSymmetricMatrix<F,const N:usize> = SkewSymmetric<matrix::Matrix<F,N,N>>;
pub type SkewSymmetricMatrixDyn<F>            = SkewSymmetric<SquareMatrixDyn<F>>;

pub type SymmetricMatrixDyn<F>=Symmetric<SquareMatrixDyn<F>>;
pub type SymmetricMatrix<F,const N:usize>=Symmetric<matrix::Matrix<F,N,N>>;

// triangular

pub type LeftTriangularMatrix<F, const M:usize, const N:usize>=LeftTriangular<matrix::Matrix<F,M,N>>;
pub type LeftTriangularMatrixDyn<F>=LeftTriangular<MatrixDyn<F>>;


pub type SquareLeftTriangularMatrix<F,const N:usize>=LeftTriangularMatrix<F,N,N>;
pub type SquareLeftTriangularMatrixDyn<F>=LeftTriangular<Square<MatrixDyn<F>>>;

pub type RightTriangularMatrix<F, const M:usize, const N:usize>=RightTriangular<matrix::Matrix<F,M,N>>;

pub type RightTriangularMatrixDyn<F>=RightTriangular<MatrixDyn<F>>;


pub type SquareRightTriangularMatrix<F,const N:usize>=RightTriangularMatrix<F,N,N>;
pub type SquareRightTriangularMatrixDyn<F>=RightTriangular<Square<MatrixDyn<F>>>;


pub use homogeneous::{
    HomogeneousMatrix, Homogeneous, HomogeneousMatrix3, HomogeneousMatrix4
};

pub mod orthogonal_unitary;
pub use orthogonal_unitary::{
    OrthogonalMatrix,    Orthogonal,    OrthogonalMatrixDyn,
    UnitaryMatrix,       Unitary,       UnitaryMatrixDyn,
    SquareStiefelMatrix, SquareStiefel, SquareStiefelMatrixDyn
};

pub mod special_orthogonal_unitary;
pub use special_orthogonal_unitary::{
    SpecialOrthogonalMatrix, SpecialOrthogonal, SpecialOrthogonalMatrixDyn,
    SpecialUnitaryMatrix,    SpecialUnitary,    SpecialUnitaryMatrixDyn,
    SpecialStiefelMatrix,    SpecialStiefel,    SpecialStiefelMatrixDyn
};

pub mod stiefel;
pub use stiefel::{StiefelMatrix, Stiefel, StiefelMatrixDyn};


pub type HouseholderTrafo<F,const N:usize>=HouseholderTrafoGeneric<MatrixCol<F,N>>;
pub type HouseholderTrafoDyn<F>=HouseholderTrafoGeneric<MatrixColDyn<F>>;


