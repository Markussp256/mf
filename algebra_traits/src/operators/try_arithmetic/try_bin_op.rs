use container_traits::LensNotEqualError;
use either::Either;

macro_rules! try_bin_op {
    ($tr:ident, $fn:ident) => {
        paste::paste!(
        pub trait [<Try $tr>]<Rhs=Self> {
            type Output;
            type Error;
            fn [<is_  $fn able_by>](&self, rhs:&Rhs) -> Result<(),          Self::Error>;
            fn [<try_ $fn>]        ( self, rhs: Rhs) -> Result<Self::Output,Self::Error>;
        }
        
        );
    }
}
try_bin_op!(Add, add);
try_bin_op!(Sub, sub);
try_bin_op!(Mul, mul);
try_bin_op!(Div, div);
try_bin_op!(Pow, pow);

// use super::super::basic::{Add,Sub};

macro_rules! try_add_sub {
    ($tr:ident, $fn:ident) => {
        paste::paste!(
        impl<T:[<Try $tr>]<T2,Output=TR,Error=E>,E,T2,TR> [<Try $tr>]<Vec<T2>> for Vec<T> {
            type Output = Vec<TR>;
            type Error  = Either<LensNotEqualError,E>;
            fn [<is_ $fn able_by>](&self,rhs:&Vec<T2>) -> Result<(),Self::Error> {
                LensNotEqualError::try_new(self.len(), rhs.len())
                    .map_err(Either::Left)?;
                self.iter()
                    .zip(rhs.iter())
                    .map(|(lhs,rhs)| <T as [<Try $tr>]<T2>>::[<is_ $fn able_by>](lhs,rhs))
                    .collect::<Result<Vec<()>,_>>()
                    .map_err(Either::Right)
                    .map(|_|())
            }

            fn [<try_ $fn>](self,rhs:Vec<T2>) -> Result<Self::Output,Self::Error> {
                LensNotEqualError::try_new(self.len(), rhs.len())
                    .map_err(Either::Left)?;
                self.into_iter()
                    .zip(rhs.into_iter())
                    .map(|(lhs,rhs)| <T as [<Try $tr>]<T2>>::[<try_ $fn>](lhs,rhs))
                    .collect::<Result<Vec<_>,E>>()
                    .map_err(Either::Right)
            }
        }

        // this operation always fails when Vec's do not have the same length
        // hence we implement Try whenever we have Try
        // impl<T:[<Try $tr>]<T2,Output=TR>,T2,TR> [<Try $tr>]<Vec<T2>> for Vec<T> where Self : [<Try $tr>]<Vec<T2>> {
        //     $crate::impl_tryop_from_tryop_impl!($tr, $fn,by rhs:Vec<T2>);
        // }


        impl<T:[<Try $tr>]<T2,Output=TR>,T2,TR,const N:usize> [<Try $tr>]<[T2;N]> for [T;N] {
            type Output = [TR;N];
            type Error  = <T as [<Try $tr>]<T2>>::Error;
            fn [<is_ $fn able_by>](&self, rhs:&[T2;N]) -> Result<(),       <T as [<Try $tr>]<T2>>::Error> {
                 self.iter()
                    .zip(rhs.iter())
                    .map(|(l,r)|<T as [<Try $tr>]<T2>>::[<is_ $fn able_by>](l,r))
                    .collect::<Result<Vec<()>,_>>()
                    .map(|_|())
            }
            fn [<try_ $fn>]    ( self, rhs: [T2;N]) -> Result<Self::Output,<T as [<Try $tr>]<T2>>::Error> {
                self.into_iter()
                    .zip(rhs.into_iter())
                    .map(|(l,r)|<T as [<Try $tr>]<T2>>::[<try_ $fn>](l,r))
                    .collect::<Result<Vec<TR>,_>>()
                    .map(|v|v.try_into().ok().unwrap())
            }
        }

        // this operation may only fail if T is Try... hence we implement it for this
        // impl<T:[<Try $tr>]<T2,Output=TR>,T2,TR,const N:usize> [<Try $tr>]<[T2;N]> for [T;N] where Self : [<Try $tr>]<[T2;N]> {
        //     $crate::impl_tryop_from_tryop_impl!($tr, $fn,by rhs:[T2;N]);
        // }

        );
    };
}
try_add_sub!(Add,add);
try_add_sub!(Sub,sub);

#[test]
fn test_try_sub_vecvec() {
    let vv0=vec![vec![1,2],vec![4]];
    let vv1=vec![vec![3,4],vec![7]];
    assert_eq!(vv1.try_sub(vv0), Ok(vec![vec![2,2],vec![3]]));
}