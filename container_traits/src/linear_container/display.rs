
use std::fmt::{Display,Formatter};
use crate::Iter;

pub fn linear_container_display<C:Iter<T>,T:Display>(
    c : &C,
    f : &mut Formatter<'_>,
    default_width : usize,
    default_prec  : usize
) -> std::fmt::Result {
    let mut iter = c.iter();
    let width=f.width().unwrap_or(default_width);
    let precision= f.precision().unwrap_or(default_prec);
    let write_number=|f:&mut Formatter, t:&T|write!(f, "{:+width$.precision$}", t, width=width, precision=precision);
    if let Some(first) = iter.next() {
        write_number(f, first)?;
        for value in iter {
            write!(f, ", ")?;
            write_number(f, value)?;
        }
    }
    Ok(())
}

#[macro_export]
macro_rules! impl_display_for_linear_container {

    ($def_width:literal, $def_prec:literal  ) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            $crate::linear_container::display::linear_container_display(self, f, $def_width, $def_prec)
        }
    };

    () => { $crate::impl_display_for_linear_container!(8, 4); };
}