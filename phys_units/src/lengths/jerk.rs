crate::unit_trait!(MetersPerSecond3, m_per_sec3, M_PER_SEC3);

/// construct pairs of jerk types 
/// # Example
///
/// ```rust
/// phys_units::gen_jerk_types!(
///     JerkMeasure0,Jerk0,
///     JerkMeasure1,Jerk1);
/// use phys_units::MetersPerSecond3;
/// let j0=JerkMeasure0::from_m_per_sec3(42.0);
/// assert_eq!(j0.m_per_sec3(), 42.0);
/// let dj0=Jerk0::from_m_per_sec3(1.0);
/// let sum:JerkMeasure0=j0+dj0;
/// assert_eq!(sum.m_per_sec3(),42.0+1.0);
/// ```
#[macro_export]
macro_rules! gen_jerk_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(M_PER_SEC3);

       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            MetersPerSecond3, m_per_sec3, M_PER_SEC3);
    

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, MetersPerSecond3, m_per_sec3, TIME_DIM=-3, LENG_DIM=1);
        );
        )*
    };
}
crate::gen_jerk_types!(JerkMeasure, Jerk);