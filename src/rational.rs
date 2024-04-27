use core::ops::*;
use crate::traits::*;
use num_traits::*;
use num_rational::*;
use num_integer::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rational<T: Clone + Integer>(pub Ratio<T>);

impl<T: Clone + Integer + From<u8> + AddAssign + MulAssign> core::str::FromStr for Rational<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut numer = T::from(0);
        let mut denom = T::from(1);
        let mut di = false;

        for c in s.chars() {
            match (c, di) {
                ('0'..='9', _) => {
                    numer *= 10.into();
                    numer += (c as u8 - '0' as u8).into();

                    if di {
                        denom *= 10.into();
                    }
                },
                ('.', false) => di = true,
                _ => return Err(()),
            }
        }

        Ok(Self(Ratio::new(numer, denom)))
    }
}

impl<T: Clone + Integer + From<usize>> FromConstant for Rational<T> {
    fn from_constant(c: &str) -> Option<Self> {
        match c {
            "π" => Some(Self(Ratio::new_raw(312689.into(), 99532.into()))),
            "φ" | "ϕ" => Some(Self(Ratio::new_raw(121393.into(), 75025.into()))),
            "e" => Some(Self(Ratio::new_raw(517656.into(), 190435.into()))),
            "τ" => Some(Self(Ratio::new_raw(312689.into(), 49766.into()))),
            "γ" => Some(Self(Ratio::new_raw(30316449.into(), 52521875.into()))),
            "c_m/s" => Some(Self(Ratio::new_raw(299792458.into(), 1.into()))),
            _ => None,
        }
    }
}

macro_rules! delegate_biop {
    ($t: path, $f: ident) => {
        impl<T: Clone + Integer> $t for Rational<T> {
            type Output = Self;

            fn $f(self, rhs: Self) -> Self { Self(self.0.$f(rhs.0)) }
        }
    };
}

delegate_biop!(Add, add);
delegate_biop!(Sub, sub);
delegate_biop!(Mul, mul);
delegate_biop!(Div, div);
delegate_biop!(Rem, rem);

impl<T: Clone + Integer + Neg<Output = T>> Neg for Rational<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl<T: Clone + Integer + From<u8> + AddAssign + MulAssign> Num for Rational<T> {
    type FromStrRadixErr = ();

    fn from_str_radix(s: &str, rad: u32) -> Result<Self, ()> {
        if rad == 10 {
            use core::str::FromStr;

            Self::from_str(s)
        } else {
            Err(())
        }
    }
}

impl<T: Clone + Integer> Zero for Rational<T> {
    fn zero() -> Self { Self(Ratio::zero()) }

    fn is_zero(&self) -> bool { self.0.is_zero() }

    fn set_zero(&mut self) { self.0.set_zero() }
}

impl<T: Clone + Integer> One for Rational<T> {
    fn one() -> Self { Self(Ratio::one()) }

    fn is_one(&self) -> bool { self.0.is_one() }

    fn set_one(&mut self) { self.0.set_one() }
}

impl<T: Clone + Integer + ToPrimitive + Signed + TryFrom<u64> + TryInto<u64> + Pow<u64, Output = T>> Pow<Self> for Rational<T> {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self {
        if rhs.0.is_negative() {
            return Self((self.pow(Self(-rhs.0))).0.inv());
        }

        if rhs.0.is_integer() {
            return Self(Ratio::new(self.0.numer().clone().pow(rhs.0.to_integer().try_into().ok().unwrap()), self.0.denom().clone().pow(rhs.0.to_integer().try_into().ok().unwrap())));
        }

        // LIGHT:
        // |  a  |c    a^c
        // | --- |  = -----
        // |  b  |     b^c

        let r = rhs.0.numer().to_f64().unwrap() / rhs.0.denom().to_f64().unwrap();
        let mul = 1e10 / r;
        let numer = ((self.0.numer().to_f64().unwrap().powf(r) * mul).round() as u64).try_into().ok().unwrap();
        let denom = ((self.0.denom().to_f64().unwrap().powf(r) * mul).round() as u64).try_into().ok().unwrap();

        Self(Ratio::new(numer, denom))
    }
}

impl<T: Clone + Integer + core::fmt::Display + ToPrimitive> core::fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0.denom().is_one() {
            self.0.numer().fmt(f)
        } else {
            write!(f, "{} / {} ({})", self.0.numer(), self.0.denom(), self.0.numer().to_f64().unwrap() / self.0.denom().to_f64().unwrap())
        }
    }
}
