crate::unit_trait!(MetersPerSecond,      m_per_sec,      M_PER_SEC);
crate::unit_trait!(MillimetersPerMinute, mm_per_min,     MM_PER_MIN);
crate::unit_trait!(InchesPerMinute,      inches_per_min, INCHES_PER_MIN);

/// construct pairs of speed types 
/// # Example
///
/// ```rust
/// phys_units::gen_speed_types!(
///     SpeedMeasure0,Speed0,
///     SpeedMeasure1,Speed1);
/// use phys_units::MetersPerSecond;
/// let s0=SpeedMeasure0::from_m_per_sec(42.0);
/// assert_eq!(s0.m_per_sec(), 42.0);
/// let ds0=Speed0::from_m_per_sec(1.0);
/// let sum:SpeedMeasure0=s0+ds0;
/// assert_eq!(sum.m_per_sec(),42.0+1.0);
/// ```
#[macro_export]
macro_rules! gen_speed_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(M_PER_SEC,
            MM_PER_MIN = M_PER_SEC * 1e-3 / 60.0,
            INCHES_PER_MIN = MM_PER_MIN * 25.4 );

       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            MetersPerSecond,      m_per_sec,      M_PER_SEC,
            MillimetersPerMinute, mm_per_min,     MM_PER_MIN,
            InchesPerMinute,      inches_per_min, INCHES_PER_MIN);
    
        $crate::conv_meas!($pname, MetersPerSecond, m_per_sec, Speed, meters_per_second);
        $crate::conv_meas!($dname, MetersPerSecond, m_per_sec, Speed, meters_per_second);

        $crate::conv_simple_si_units!($pname, MetersPerSecond, m_per_sec, simple_si_units::mechanical::Velocity::<f64>, meters_per_second);
        $crate::conv_simple_si_units!($dname, MetersPerSecond, m_per_sec, simple_si_units::mechanical::Velocity::<f64>, meters_per_second);

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, MetersPerSecond, m_per_sec, TIME_DIM=-1, LENG_DIM=1);
        );
        )*
    };
}
crate::gen_speed_types!(SpeedMeasure, Speed);