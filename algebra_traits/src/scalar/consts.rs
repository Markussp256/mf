use crate::TryDiv;

pub trait ConstZero {    
    const ZERO:Self;
}

pub trait ConstNonZero : Sized {    
    const NONZERO:Self;
    // division by NonZero constant

    fn div_nz_gen<Lhs:TryDiv<Self>>(lhs:Lhs) -> <Lhs as TryDiv<Self>>::Output {
        lhs.try_div(Self::NONZERO).ok().unwrap()
    }


    fn div_nz(self) -> <Self as TryDiv>::Output where Self : TryDiv {
        Self::div_nz_gen(self)
    }
}

pub trait ConstOne {
    const ONE:Self;
}

pub trait ConstPi {
    const PI:Self;
}

pub trait ConstRad2Deg {
    const RAD2DEG:Self;
}

pub trait ConstDeg2Rad {
    const DEG2RAD:Self;
}