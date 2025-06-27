use std::cmp::PartialOrd;

// we need a Max trait because we do not want to restrict Ord all over the library
// it is "safe" because we do not use NAN or infinity values anyways
pub trait Max : Sized+PartialOrd {
    fn max<'a>(&'a self, rhs:&'a Self) -> &'a Self;

    fn into_max(self, rhs:Self) -> Self {
        if self.max(&rhs) == &self {
            self
        } else {
            rhs
        }
    }
}


pub trait TryMin : Sized+PartialOrd {
    // provided method
    fn try_min(self, rhs: Self) -> Option<Self> {
        if self <= rhs {
            Some(self)
        } else if rhs <= self {
            Some(rhs)
        } else {
            None
        }
    }
}
impl<T:Sized+PartialOrd> TryMin for T {}

pub trait TryMax : Sized+PartialOrd {
    // provided method
    fn try_max(self, rhs: Self) -> Option<Self> {
        if self >= rhs {
            Some(self)
        } else if rhs >= self {
            Some(rhs)
        } else {
            None
        }
    }
}
impl<T:Sized+PartialOrd> TryMax for T {}