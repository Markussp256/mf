
// One problem when trying to implement ratio in a regular struct with associated types rather than in a macro
// is that we can not define multiplication with f64 and den because potentially den could be f64 and then there would be
// conflicting implementations. Therefore a macro has been used. The macro can be called for types that implement Vectorspace<f64>,
// Div<Output=f64> and the ConstNonzero Trait above





#[macro_export]
macro_rules! ratio_type {
    ($New:ident, $Nom:ident, $Den:ident) => {
        $crate::ratio_type_without_self_div!($New,$Nom,$Den);

        impl<F:algebra_traits::RealNumber> algebra_traits::TryDiv<$Den<F>> for $Nom<F> {
            type Output=$New<F>;
            fn try_div(self, rhs:$Den<F>) -> Option<Self::Output> {
                let nz_rhs=algebra_traits::NonZero::try_new(rhs)?;
                Some($New::<F>::new(self, nz_rhs))
            }
        }

        impl<F:algebra_traits::RealNumber> algebra_traits::TryDiv<$New<F>> for $Nom<F> {
            type Output=$Den<F>;
            fn try_div(self, rhs:$New<F>) -> Option<Self::Output> {
                let fac=self.try_div(rhs.nom)?;
                Some(rhs.den.into_inner()*fac)
            }
        }
    };
}

// can not implement division by self because then we have contradicting implementations
#[macro_export]
macro_rules! self_ratio_type {
    ($New:ident, $Nom_Den:ident) => {
        $crate::ratio_type_without_self_div!($New, $Nom_Den, $Nom_Den);
    };
}

// private fns for convenient implementation
// pub fn unsafe_div<T:algebra_traits::TryDiv>(a:T, b:T) -> <T as algebra_traits::TryDiv>::Output {
//     <T as algebra_traits::TryDiv>::unsafe_div(a, b)
// }

// pub fn is_zero<T:num_traits::Zero>(a:&T) -> bool {
//     <T as num_traits::Zero>::is_zero(a)
// }

#[macro_export]
macro_rules! ratio_type_without_self_div {
    ($New:ident, $Nom:ident, $Den:ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $New<F> {
            nom: $Nom<F>,
            den: algebra_traits::NonZero<$Den<F>>,
        }

        impl<F> $New<F> {
            pub fn nom(&self) -> &$Nom<F> {
                &self.nom
            }

            pub fn den(&self) -> &algebra_traits::NonZero<$Den<F>> {
                &self.den
            }

            pub fn into_nom_den(self) -> ($Nom<F>, algebra_traits::NonZero<$Den<F>>) {
                (self.nom, self.den)
            }
        }



        impl<F:algebra_traits::RealNumber> PartialEq for $New<F> {
            fn eq(&self, rhs: &Self) -> bool {
                if <Self as num_traits::Zero>::is_zero(rhs) {
                    return <Self as num_traits::Zero>::is_zero(self);
                }
                let rhs=rhs.clone();
                let s=self.clone();
                <$Nom<F> as algebra_traits::TryDiv>::unsafe_div(s.nom, rhs.nom).eq(&(s.den /rhs.den).into_inner())
            }
        }

        impl<F:Clone+algebra_traits::RealNumber> PartialOrd for $New<F> {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                let rhs=rhs.clone();
                let s=self.clone();
                if <Self as num_traits::Zero>::is_zero(&rhs) {
                    if <Self as num_traits::Zero>::is_zero(self) {
                        Some(std::cmp::Ordering::Equal)
                    } else {
                        (rhs.den / s.den).into_inner().partial_cmp(
                        &<$Nom<F> as algebra_traits::TryDiv>::unsafe_div(rhs.nom, s.nom))
                    }
                } else {
                        <$Nom<F> as algebra_traits::TryDiv>::unsafe_div(s.nom, rhs.nom).partial_cmp(
                        &(rhs.den / s.den).into_inner())
                }
            }
        }

        impl<F:algebra_traits::RealNumber> $New<F> {
            pub fn new(nom: $Nom<F>, den: algebra_traits::NonZero<$Den<F>>) -> Self {
                    Self { nom, den }
            }
        }

        impl<F:algebra_traits::RealNumber> std::ops::Mul<$Den<F>> for $New<F> {
            fn mul(self, den: $Den<F>) -> $Nom<F> {
                self.nom * (den / self.den)
            }
            type Output = $Nom<F>;
        }

        impl<F:algebra_traits::RealNumber> algebra_traits::TryDiv<Self> for $New<F> {
            type Output = F;
            fn try_div(self, rhs: Self) -> Option<Self::Output> {
                <$Nom<F> as algebra_traits::TryDiv>::try_div(self * rhs.den.into_inner(), rhs.nom)
            }
        }

        impl<F:algebra_traits::RealNumber> std::ops::Mul<F> for $New<F> {
            fn mul(self, f: F) -> Self::Output {
                Self {
                    nom: self.nom * f,
                    den: self.den,
                }
            }
            type Output = $New<F>;
        }

        impl<F> algebra_traits::IntegralDomain for $New<F>
        where $Nom<F> : algebra_traits::IntegralDomain {}

        impl<F:algebra_traits::RealNumber> algebra_traits::TryDiv<F> for $New<F> {
            fn try_div(self, f: F) -> Option<Self::Output> {
                // let f:F=f.into();
                let nom=self.nom.try_div(f)?;
                Some(Self {
                    nom,
                    den: self.den,
                })
            }
            type Output = $New<F>;
        }

        impl<F:algebra_traits::RealNumber> std::ops::Add<Self> for $New<F> {
            fn add(self, rhs: Self) -> Self {
                Self {
                    nom: self.nom + rhs.nom * (self.den.clone()/ rhs.den).into_inner(),
                    den: self.den,
                }
            }
            type Output = Self;
        }

        impl<F> algebra_traits::TryAdd for $New<F> where Self : std::ops::Add<Output=Self> {
            type Output=Self;
            fn try_add(self, rhs:Self) -> Option<Self> {
                Some(self + rhs)
            }
        }

        impl<F:algebra_traits::RealNumber> std::ops::Sub for $New<F> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    nom: self.nom - rhs.nom * (self.den.clone()/ rhs.den).into_inner(),
                    den: self.den,
                }
            }
        }

        impl<F> algebra_traits::TrySub for $New<F> where Self : std::ops::Sub<Output=Self> {
            type Output=Self;
            fn try_sub(self, rhs:Self) -> Option<Self> {
                let rhs:Self=rhs.into();
                Some(self - rhs)
            }
        }

        impl<F:Clone+algebra_traits::RealNumber> std::ops::AddAssign<Self> for $New<F> {
            fn add_assign(&mut self, rhs: Self) {
                let result = self.clone() + rhs;
                self.den = result.den;
                self.nom = result.nom;
            }
        }

        impl<F:Clone+algebra_traits::RealNumber> std::ops::SubAssign<Self> for $New<F> {
            fn sub_assign(&mut self, rhs: Self) {
                let result = self.clone() - rhs;
                self.den = result.den;
                self.nom = result.nom;
            }
        }

        impl<F:algebra_traits::RealNumber> std::ops::Neg for $New<F> {
            fn neg(self) -> Self {
                Self {
                    nom: -self.nom,
                    den: self.den,
                }
            }
            type Output = Self;
        }

        impl<F:algebra_traits::RealNumber> algebra_traits::AdditiveGroup for $New<F> {}
        impl<F:algebra_traits::RealNumber> algebra_traits::ScalarMul<F> for $New<F> {
            fn scalar_mul(self, f:&F) -> Self {
                self * f.clone()
            }
        }
        impl<F:algebra_traits::RealNumber> algebra_traits::TryScalarDiv<F> for $New<F> {}
        impl<F:algebra_traits::RealNumber> algebra_traits::Vectorspace<F> for $New<F> {}

        impl<F:algebra_traits::RealNumber> core::iter::Sum for $New<F> {
            fn sum<I>(iter: I) -> $New<F>
            where
                I: Iterator<Item = $New<F>>,
            {
                iter.fold(<$New<F> as num_traits::Zero>::zero(), |acc, x| acc + x)
            }
        }

        impl<F:algebra_traits::RealNumber> num_traits::Zero for $New<F> {
            fn zero() -> Self {
                Self::new(
                    <$Nom<F>>::zero(),
                    NonZero::try_new(<$Den<F> as algebra_traits::ConstNonzero>::NonZero).unwrap(),
                )
            }

            fn is_zero(&self) -> bool {
                <$Nom<F> as num_traits::Zero>::is_zero(&self.nom)
            }
        }

        impl<F:algebra_traits::RealNumber> algebra_traits::Parameters<F> for $New<F> {
            fn parameters(&self) -> Vec<F> {
                vec![<Self as algebra_traits::Parameters1<F>>::parameter(&self)]
            }

            fn try_from_iter<I:Iterator<Item=F>>(iter: & mut I) -> Option<Self> {
                iter.next()
                    .map(<Self as algebra_traits::Parameters1<F>>::from_parameter)
            }
        }

        impl<F:algebra_traits::RealNumber> algebra_traits::NumberOfDegreesOfFreedom<F> for $New<F> {
            const NDOFS:usize=1;
        }

        impl<F:algebra_traits::RealNumber> algebra_traits::Parameters1<F> for $New<F> where Self:Clone {
            fn parameter(&self) -> F {
                 // assumes 0 is maped to 0
                 <F as algebra_traits::TryDiv<F>>::unsafe_div(
                <$Nom<F> as algebra_traits::Parameters1<F>>::parameter(&self.nom),
                <$Den<F> as algebra_traits::Parameters1<F>>::parameter(self.den().as_ref()))
            }

            fn from_parameter(f:F) -> Self {
                Self::new(<$Nom<F> as algebra_traits::Parameters1<F>>::from_parameter(f),
                          algebra_traits::NonZero::try_new(
                          <$Den<F> as algebra_traits::Parameters1<F>>::from_parameter(F::one())).unwrap())
            }
        }



    };
}

#[cfg(feature = "phys_units_support")]
use phys_units::LengthGeneric;

#[cfg(feature = "phys_units_support")]
crate::self_ratio_type!(LengthRatioGeneric, LengthGeneric);

#[cfg(feature = "phys_units_support")]
pub type LengthRatio=LengthRatioGeneric<f64>;

#[test]
#[cfg(feature = "phys_units_support")]
fn test_length_ratio() {
    use algebra_traits::{NonZero, Parameters1};
    use phys_units::{PhysicalQuantity, Length};
    let a:f64=1.0;
    let b:f64=2.0;
    let c=LengthRatio::new(
        Length::from_si(a),
        NonZero::try_new(Length::from_si(b)).unwrap());
    assert_eq!(c.parameter(), a/b);
}