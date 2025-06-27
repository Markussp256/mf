crate::unit_trait!(MetersPerSecond2, m_per_sec2, M_PER_SEC2);

/// construct pairs of acceleration types 
/// # Example
///
/// ```rust
/// phys_units::gen_acceleration_types!(
///     AccelerationMeasure0,Acceleration0,
///     AccelerationMeasure1,Acceleration1);
/// use phys_units::MetersPerSecond2;
/// let ac0=AccelerationMeasure0::from_m_per_sec2(42.0);
/// assert_eq!(ac0.m_per_sec2(), 42.0);
/// let dac0=Acceleration0::from_m_per_sec2(1.0);
/// let sum:AccelerationMeasure0=ac0+dac0;
/// assert_eq!(sum.m_per_sec2(),42.0+1.0);
/// ```
#[macro_export]
macro_rules! gen_acceleration_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(M_PER_SEC2);

       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            MetersPerSecond2, m_per_sec2, M_PER_SEC2);
    

        $crate::conv_meas!($pname, MetersPerSecond2, m_per_sec2, Acceleration, meters_per_second_per_second);
        $crate::conv_meas!($dname, MetersPerSecond2, m_per_sec2, Acceleration, meters_per_second_per_second);

        $crate::conv_simple_si_units!($pname, MetersPerSecond2, m_per_sec2, simple_si_units::mechanical::Acceleration::<f64>, meters_per_second_squared);
        $crate::conv_simple_si_units!($dname, MetersPerSecond2, m_per_sec2, simple_si_units::mechanical::Acceleration::<f64>, meters_per_second_squared);

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, MetersPerSecond2, m_per_sec2, TIME_DIM=-2, LENG_DIM=1);
        );
        )*
    };
}

gen_acceleration_types!(AccelerationMeasure, Acceleration);