use crate::AdditiveGroup;
use std::ops::{Add,Sub};



// Torsor is basically an affinespace without field and whose corresponding vectorspace does not have scalar multiplication
// name suggestion by ChatGPT
pub trait Torsor
        : Sized
         +Sub
         +Sub<<Self as Sub>::Output, Output = Self>
         +Add<<Self as Sub>::Output, Output = Self>
    // where <Self as Sub>::Output : AdditiveGroup
{}

impl<V:AdditiveGroup> Torsor for V {}


    // where <Self as Sub>::Output : NormedSpace<NormT=Self::DistT> + AdditiveGroup,
    //    Self::DistT : Tolerance