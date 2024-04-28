use core::ops::*;
use crate::traits::*;
use num_traits::*;
use num_rational::*;
use num_integer::*;

macro_rules! to_f64 {
    ($f: expr) => {{
        $f.numer().to_f64().unwrap() / $f.denom().to_f64().unwrap()
    }};
}

macro_rules! from_f64 {
    ($f: expr) => {{
        let f = $f;
        Ratio::new(((f * 1e10).round() as i64).into(), (1e10 as i64).into())
    }};
}

#[cfg(feature = "num_complex")]
pub mod complex;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    ($base: tt, $t: path, $f: ident) => {
        impl<T: Clone + Integer> $t for $base<T> {
            type Output = Self;

            fn $f(self, rhs: Self) -> Self { Self(self.0.$f(rhs.0)) }
        }
    };
}

use delegate_biop;

delegate_biop!(Rational, Add, add);
delegate_biop!(Rational, Sub, sub);
delegate_biop!(Rational, Mul, mul);
delegate_biop!(Rational, Div, div);
delegate_biop!(Rational, Rem, rem);

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

impl<T:std::fmt::Debug+ Clone + Integer + ToPrimitive + Signed + From<i64> + TryInto<u64> + Pow<u64, Output = T>> Pow<Self> for Rational<T> {
    type Output = Self;

    fn pow(self, exp: Self) -> Self {
        if exp.0.is_negative() {
            return Self((self.pow(Self(-exp.0))).0.inv());
        }

        if exp.is_zero() {
            return Self::one();
        }

        if exp.0.is_integer() {
            return Self(Ratio::new(self.0.numer().clone().pow(exp.0.to_integer().to_u64().unwrap()), self.0.denom().clone().pow(exp.0.to_integer().to_u64().unwrap())));
        }

        // // LIGHT:
        // // |  a  |c    a^c
        // // | --- |  = -----
        // // |  b  |     b^c

        // let r = to_f64!(rhs.0);
        // let numer = ((self.0.numer().to_f64().unwrap().powf(r) * 1e10).round() as u64).try_into().ok().unwrap();
        // let denom = ((self.0.denom().to_f64().unwrap().powf(r) * 1e10).round() as u64).try_into().ok().unwrap();

        // Self(Ratio::new(numer, denom))

        panic!("{:?}", self.ln());

        // (exp * self.ln()).exp()
    }
}

impl<T: Clone + Integer + From<i64> + TryInto<u64> + Pow<u64, Output = T>> Rational<T> {
    pub fn ln(mut self) -> Self {
        let p = (Ratio::new(41904491.into(), 43538251.into()) * ((self.0.clone() - Ratio::one()) / (self.0.clone() + Ratio::one()))).round().numer().clone();
        self.0 = self.0 * T::from(10);
        let pp = p.clone().try_into().ok().unwrap();
        let smol_ln = Ratio::new(self.0.numer().clone().pow(pp.clone()), self.0.denom().clone().pow(pp)) - Ratio::one();

        Self(smol_ln + Ratio::new_raw(53443.into(), 23210.into()) * p)
    }
}

impl<T: Clone + Integer + core::fmt::Display + ToPrimitive> core::fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0.denom().is_one() {
            self.0.numer().fmt(f)
        } else if f.alternate() {
            write!(f, "{} / {} ({})", self.0.numer(), self.0.denom(), self.0.numer().to_f64().unwrap() / self.0.denom().to_f64().unwrap())
        } else {
            write!(f, "({} / {})", self.0.numer(), self.0.denom())
        }
    }
}

// fn exp_approx<T: Clone + Integer + ToPrimitive + Signed + TryFrom<u64> + TryInto<u64> + Pow<u64, Output = T>>(exp: Ratio<T>) -> Ratio<T> {
//     // Rational(Ratio::new_raw(517656.try_into().ok().unwrap(), 190435.try_into().ok().unwrap())).pow(Rational(exp)).0
// }

impl<T:std::fmt::Debug+ Clone + Integer + ToPrimitive + Signed + From<i64> + TryInto<u64> + Pow<u64, Output = T>> ExecuteFunction for Rational<T> {
    fn execute(f: &str, args: &[Self]) -> Result<Self, ()> {
        match (f, args.len()) {
            ("floor", 1) => Ok(Self(args[0].0.floor())),
            ("ceil", 1) => Ok(Self(args[0].0.ceil())),
            ("round", 1) => Ok(Self(args[0].0.round())),
            ("trunc", 1) => Ok(Self(args[0].0.trunc())),
            ("fract", 1) => Ok(Self(args[0].0.fract())),
            ("abs", 1) => Ok(Self(args[0].0.abs())),
            ("sqrt" | "√", 1) => Ok(args[0].clone().pow(Self(Ratio::new(T::one(), T::one() + T::one())))),
            ("ln", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).ln()))),
            ("log", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).log10()))),
            ("log", 2) => Ok(Self(from_f64!(to_f64!(args[0].0).log(to_f64!(args[0].0))))),
            ("min", _) => Ok(Self(args.iter().map(|a| a.0.clone()).min().ok_or(())?)),
            ("max", _) => Ok(Self(args.iter().map(|a| a.0.clone()).max().ok_or(())?)),
            ("cbrt" | "∛", 1) => Ok(args[0].clone().pow(Self(Ratio::new(T::one(), T::one() + T::one() + T::one())))),
            ("sin", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).sin()))),
            ("cos", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).cos()))),
            ("tan", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).tan()))),
            ("asin", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).asin()))),
            ("acos", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).acos()))),
            ("atan", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).atan()))),
            ("sinh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).sinh()))),
            ("cosh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).cosh()))),
            ("tanh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).tanh()))),
            ("asinh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).asinh()))),
            ("acosh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).acosh()))),
            ("atanh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).atanh()))),
            _ => Err(()),
        }
    }
}
