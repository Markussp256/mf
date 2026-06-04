use generic_array::{GenericArray, ArrayLength};


pub fn inspect_err_ref<T,E>(r:&Result<T,E>,f:impl Fn(&E)) {
    if let Err(e)=r {
        f(e)
    }
}


pub fn unwrap_if_all_are_ok_vec<T,E>(ovec:Vec<Result<T,E>>) -> Result<Vec<T>,E> {
    let mut res=Vec::with_capacity(ovec.len());
    for r in ovec {
        if r.is_err() {
            return Err(r.err().unwrap());
        }
        res.push(r.ok().unwrap())
    }
    Ok(res)
}


pub fn unwrap_if_all_are_ok_gen_arr<T, E, N: ArrayLength>(
    oarr: GenericArray<Result<T,E>, N>
) -> Result<GenericArray<T, N>, E> {
    match oarr.iter().enumerate().find(|r|r.1.is_err()).map(|e|e.0) {
        Some(i) => Err(oarr.into_iter().nth(i).unwrap().err().unwrap()),
        None => 
        Ok(oarr
            .into_iter()
            .map(|x| x.ok().unwrap())
            .collect::<GenericArray<_, N>>())
    }
}



pub fn unwrap_if_all_are_ok_arr<T, E, const N:usize>(
    oarr: [Result<T,E>; N]
) -> Result<[T; N], E> {
    match oarr.iter().enumerate().find(|r|r.1.is_err()).map(|e|e.0) {
        Some(i) => Err(oarr.into_iter().nth(i).unwrap().err().unwrap()),
        None =>
        Ok(oarr
            .map(|x| x.ok().unwrap()))
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