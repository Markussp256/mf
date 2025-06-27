use super::{AdditiveGroup, MultiplicativeGroup};

use std::ops::{Mul,Div};
use num_traits::Inv;


// example of a semigroup would be isometries
// then A would be rotations (which build a group)
// and B translations

#[derive(Clone, Debug)]
pub struct SemiGroup<A:MultiplicativeGroup+Mul<B,Output=B>+Clone,
                     B:AdditiveGroup>{
                        a:A,
                        b:B
                     }

macro_rules! impl_semi_group {
    ($( $tr:ident  $(< $rhs:ident >)?)? ; $($t:tt)*) => {
        impl<A:MultiplicativeGroup+Mul<B,Output=B>+Clone,
             B:AdditiveGroup> $($tr $(<$rhs>)? for )? SemiGroup<A,B> {
          $($t)*
        }
    };
}

impl_semi_group!( ;
    pub fn new(a:A,b:B) -> Self {
        Self{a,b}
    }

    pub fn into_ab(self) -> (A,B) {
        (self.a,self.b)
    }
);

impl_semi_group!(Mul<B>;
    type Output=B;
    fn mul(self,rhs:B) -> B {
        let (a,b)=self.into_ab();
        a * rhs + b
    }
);

impl_semi_group!(Mul;
    type Output=Self;
    fn mul(self,rhs:Self) -> Self {
        let a=self.a.clone();
        let (ra,rb)=rhs.into_ab();
        Self::new(a*ra,self*rb)
    }
);

impl_semi_group!(Inv;
    type Output=Self;
    fn inv(self) -> Self {
        let (a,b)=self.into_ab();
        let ainv=<A as Inv>::inv(a);
        Self::new(ainv.clone(), -(ainv*b))
    }
);

impl_semi_group!(Div;
    type Output=Self;
    fn div(self,rhs:Self) -> Self {
        self*rhs.inv()
    }
);