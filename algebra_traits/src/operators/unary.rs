macro_rules! un_op {
    ($tr:ident, $fn:ident) => {
        paste::paste!(

        pub trait [<Try $tr>] {
            type Output;
            type Error;
            fn [<is_  $fn able>](&self) -> Result<(),          Self::Error>;
            fn [<try_ $fn>]     ( self) -> Result<Self::Output,Self::Error>;
        }

        pub trait $tr {
            type Output;
            fn $fn(self) -> Self::Output;
        }
        );
    }
}

#[macro_export]
macro_rules! impl_tryop_from_op_impl {
    ($tr:ident, $fn:ident $(,$by:ident $rhs_lc:ident:$rhs_t:ty)?) => {
        paste::paste!(
            type Output=<Self as $crate::$tr $(<$rhs_t>)? >::Output;
            type Error=std::convert::Infallible;

            fn [<is_ $fn able $(_ $by)? >](&self $(, _ : & $rhs_t)?) -> Result<(),std::convert::Infallible> {
                Ok(())
            }

            fn [<try_ $fn>]    ( self $(,$rhs_lc :   $rhs_t)?) -> Result<Self::Output,std::convert::Infallible> {
                Ok(<Self as $crate::$tr $(<$rhs_t>)? >::$fn(self $(,$rhs_lc)?))
            }
        );
    };
}

#[macro_export]
macro_rules! impl_op_from_tryop_impl {
    ($tr:ident, $fn:ident $(,$rhs_lc:ident:$rhs_t:ty)?) => {
        paste::paste!(
        type Output=<Self as $crate::[<Try $tr>]$(<$rhs_t>)?>::Output;
        fn $fn(self $(,$rhs_lc:$rhs_t)?) -> Self::Output {
            <Self as $crate::[<Try $tr>] $(<$rhs_t>)? >::[<try_ $fn>](self $(,$rhs_lc)? ).unwrap()
        });
    };
}

// #[macro_export]
// macro_rules! impl_tryop_from_anyop_impl {
//     ($tr:ident, $fn:ident $(,$by:ident $rhs_lc:ident:$rhs_t:ty)?) => {
//         paste::paste!(
//         type Output=<Self as $crate::[<Any $tr>]$(<$rhs_t>)?>::Output;
//         fn [<is_ $fn able $(_ $by)?>](&self $(,$rhs_lc:&$rhs_t)?) -> Result<(),$crate::[<$tr Error>]>{
//             <Self as $crate::[<Any $tr>]$(<$rhs_t>)?>::[<is_ $fn able $(_ $by)?>](self $(,$rhs_lc)?)
//         }
//         fn [<try_ $fn>](self $(,$rhs_lc:$rhs_t)?) -> Result<<Self as $crate::[<Any $tr>]$(<$rhs_t>)?>::Output, $crate::[<$tr Error>]> {
//             <Self as $crate::[<Any $tr>]$(<$rhs_t>)?>::[<any_ $fn>](self $(,$rhs_lc)?)
//         });
//     };
// }

#[macro_export]
macro_rules! impl_tryop_from_op {
    ($tr:ident $(<$rhs_t:ty>)?, $fn:ident, $name:ident$(<$t:ident>)?) => {
        paste::paste!(
        impl$(<$t>)? $crate::[<Try $tr>]$(<$rhs_t>)? for $name$(<$t>)?
            where Self : $crate::$tr$(<$rhs_t>)? {
            $crate::impl_tryop_from_op_impl!($tr,$fn $(,_by rhs : $rhs_t)?);
        });
    };
}

#[macro_export]
macro_rules! impl_op_from_tryop {
    ($tr:ident $(<$rhs_t:ty>)? , $fn:ident, $name:ident$(<$t:ident>)?) => {
        paste::paste!(
        impl$(<$t>)? $crate::$tr$(<$rhs_t>)? for $name$(<$t>)?
            where Self : $crate::[<Try $tr>]$(<$rhs_t>)? {
            $crate::impl_op_from_tryop_impl!($tr,$fn $(,rhs : $rhs_t)?);
        });
    };
}

// #[macro_export]
// macro_rules! impl_tryop_from_anyop {
//     ($tr:ident $(<$rhs_t:ty>)?, $fn:ident, $name:ident$(<$t:ident>)?) => {
//         paste::paste!(
//         impl$(<$t>)? $crate::[<Try $tr>]$(<$rhs_t>)? for $name$(<$t>)?
//             where Self : $crate::[<Any $tr>]$(<$rhs_t>)? {
//             $crate::impl_tryop_from_anyop_impl!($tr, $fn $(,by rhs : $rhs_t)?);
//         });
//     };
// }

macro_rules! un_onto_op {
    ($tr_name:ident, $fn_name:ident) => {
        pub trait $tr_name<OntoType> {
            fn $fn_name(self) -> OntoType;
        }
    }
}
un_onto_op!(Project,   project);


// traits without f64 impl
un_op!(Det,  det);
un_op!(Pow2, pow2);
un_op!(Exp,  exp);
un_op!(Log,  log);

// Sqrt could also be used for Matrix types therefore we can not constraint output to Nonnegative
// Sqrt for Nonnegative<f64> is automatically done using trysqrt, see Nonnegative
un_op!(     Sqrt,      sqrt);


// since we do not want fn to be called is_invable we manually define TryInv

pub trait TryInv : Sized {
    type Output;
    type Error;
    fn is_invertible(&self) -> Result<(),   Self::Error>;
    fn try_inv(self) -> Result<Self::Output,Self::Error>;
}

pub trait TryIntoReal {
    type Output;
    fn try_into_real(self) -> Option<Self::Output>;
}

