
pub fn unwrap_if_all_are_some_vec<T>(ovec:Vec<Option<T>>) -> Result<Vec<T>,Vec<Option<T>>> {
    if ovec.iter().all(Option::is_some) {
        Ok(ovec.into_iter().map(Option::unwrap).collect())
    } else {
        Err(ovec)
    }
}

pub fn unwrap_if_all_are_some_arr<T, const N:usize>(oarr:[Option<T>;N]) -> Result<[T;N],[Option<T>;N]> {
    if oarr.iter().all(Option::is_some) {
        Ok(oarr.map(Option::unwrap))
    } else {
        Err(oarr)
    }
}


pub trait OptionExt<T> {
    fn if_none(&self, f: impl FnOnce()) -> &Self;
}

impl<T> OptionExt<T> for Option<T> {
    fn if_none(&self, f: impl FnOnce()) -> &Self {
        if self.is_none() {
            f();
        }
        self
    }
}