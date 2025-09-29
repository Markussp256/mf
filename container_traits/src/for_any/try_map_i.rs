use crate::IndexOutOfBoundsError;

pub trait TryMapI<Index,T> : Sized {
    fn try_map_i(self, index:Index, f:impl FnOnce(& mut T)) -> Result<Self,IndexOutOfBoundsError<Index>>;
}


// error is infallible but we will need LCCE
impl<F> TryMapI<usize, F> for Vec<F> {
    fn try_map_i(self,index:usize,f:impl FnOnce(& mut F)) -> Result<Vec<F>,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.len(),&index)?;
        let mut smut=self;
        let x=smut.get_mut(index).unwrap();
        f(x);
        Ok(smut)
    }
}

impl<F, const N:usize> TryMapI<usize, F> for [F;N] {
    fn try_map_i(self,index:usize, f:impl FnOnce(& mut F)) -> Result<[F; N],IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.len(),&index)?;
        let mut smut=self;
        if let Some(x)=smut.get_mut(index) {
            f(x);
        }
        Ok(smut)
    }
}