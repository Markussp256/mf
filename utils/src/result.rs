

pub fn inspect_err_ref<T,E>(r:&Result<T,E>,f:impl Fn(&E)) {
    if let Err(e)=r {
        f(e)
    }
}



#[macro_export]
macro_rules! ret_or_err {
    ($e:expr) => {
        match $e {
            Ok(ok) => { return Ok(ok); },
            Err(err) => err
        }
    };
}