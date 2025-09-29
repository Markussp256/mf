
// returns corresponding type that holds container of size (R,C)
// same if dynamic
pub trait ChangeDim {
    type Output<const R:usize,const C:usize>;
}