use crate::traits::*;
use core::ops::*;
use num_integer::*;
use num_rational::*;
use num_traits::*;

mod ln_const;

#[cfg(feature = "num_complex")]
pub mod complex;

impl<T: Clone + Integer> Rational<T> {
    pub fn limit_denom(&self, md: T) -> Self {
        if md > self.0.denom().clone() {
            return self.clone();
        }

        let mut p0 = T::zero();
        let mut q0 = T::one();
        let mut p1 = T::one();
        let mut q1 = T::zero();
        let mut n = self.0.numer().clone();
        let mut d = self.0.denom().clone();

        loop {
            let a = n.clone() / d.clone();
            let q2 = q0.clone() + a.clone() * q1.clone();

            if q2 > md {
                break;
            }

            let tp1 = p0 + a.clone() * p1.clone();
            p0 = p1;
            q0 = q1;
            p1 = tp1;
            q1 = q2;

            let td = n - a * d.clone();
            n = d;
            d = td;
        }
        let k = (md - q0.clone()) / q1.clone();

        let two = T::one() + T::one();
        if two * d.clone() * (q0.clone() + k.clone() * q1.clone()) <= d {
            Self(Ratio::new_raw(p1, q1))
        } else {
            Self(Ratio::new_raw(p0 + k.clone() * p1, q0 + k * q1))
        }
    }
}

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

            "↉" => Some(Self(Ratio::zero())),

            "½" => Some(Self(Ratio::new_raw(1.into(), 2.into()))),
            "⅓" => Some(Self(Ratio::new_raw(1.into(), 3.into()))),
            "¼" => Some(Self(Ratio::new_raw(1.into(), 4.into()))),
            "⅕" => Some(Self(Ratio::new_raw(1.into(), 5.into()))),
            "⅙" => Some(Self(Ratio::new_raw(1.into(), 6.into()))),
            "⅐" => Some(Self(Ratio::new_raw(1.into(), 7.into()))),
            "⅛" => Some(Self(Ratio::new_raw(1.into(), 8.into()))),
            "⅑" => Some(Self(Ratio::new_raw(1.into(), 9.into()))),
            "⅒" => Some(Self(Ratio::new_raw(1.into(), 10.into()))),

            "⅔" => Some(Self(Ratio::new_raw(2.into(), 3.into()))),
            "⅖" => Some(Self(Ratio::new_raw(2.into(), 5.into()))),

            "¾" => Some(Self(Ratio::new_raw(3.into(), 4.into()))),
            "⅗" => Some(Self(Ratio::new_raw(3.into(), 5.into()))),
            "⅜" => Some(Self(Ratio::new_raw(3.into(), 8.into()))),

            "⅘" => Some(Self(Ratio::new_raw(4.into(), 5.into()))),

            "⅚" => Some(Self(Ratio::new_raw(5.into(), 6.into()))),
            "⅝" => Some(Self(Ratio::new_raw(5.into(), 8.into()))),

            "⅞" => Some(Self(Ratio::new_raw(7.into(), 8.into()))),
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

    fn neg(self) -> Self { Self(self.0.neg()) }
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

impl<
        T: Clone + Integer + TryFrom<u64> + TryInto<u64> + Pow<u64, Output = T> + Signed + ToPrimitive,
    > Pow<Self> for Rational<T>
{
    type Output = Self;

    fn pow(self, exp: Self) -> Self {
        if exp.0.is_negative() {
            return Self((self.pow(Self(-exp.0))).0.inv());
        }

        if exp.is_zero() {
            return Self::one();
        }

        if exp.0.is_integer() {
            return Self(Ratio::new(
                self.0
                    .numer()
                    .clone()
                    .pow(exp.0.to_integer().to_u64().unwrap()),
                self.0
                    .denom()
                    .clone()
                    .pow(exp.0.to_integer().to_u64().unwrap()),
            ));
        }

        if exp.0.numer().is_odd() && exp.0.denom().is_even() && self.0.is_negative() {
            panic!("invalid exp");
        } else {
            let inv = exp.0.numer().is_odd() && exp.0.denom().is_odd() && self.0.is_negative();
            let r = (Self(self.0.abs()).ln().unwrap() * exp).exp();

            if inv { -r } else { r }
        }
    }
}

impl<T: Clone + Integer + TryFrom<u64> + TryInto<u64> + Pow<u64, Output = T> + Signed> Rational<T> {
    pub fn ln(self) -> Option<Self> {
        if !self.0.is_positive() {
            return None;
        }

        let b = (self
            .0
            .round()
            .numer()
            .clone()
            .min(ln_const::S.try_into().ok().unwrap())
            - T::one())
        .max(T::zero())
        .try_into()
        .ok()
        .unwrap() as usize;
        let consts = &ln_const::LN_CONSTS[b];
        let b_a = Ratio::new(
            consts[0].try_into().ok().unwrap(),
            consts[1].try_into().ok().unwrap(),
        );
        let b_b = Ratio::new(
            consts[2].try_into().ok().unwrap(),
            consts[3].try_into().ok().unwrap(),
        );
        let b_c = Ratio::new(
            consts[4].try_into().ok().unwrap(),
            consts[5].try_into().ok().unwrap(),
        );

        let p_a =
            (b_a * ((self.0.clone() - T::one()) / (self.0.clone() + T::one()))).max(Ratio::zero());
        let p = (p_a * T::try_from(ln_const::U).ok().unwrap()).round();

        let mut x = (self.0.clone() / b_b.pow(p.numer().clone().try_into().ok().unwrap()))
            + (p / T::try_from(ln_const::U).ok().unwrap()) * b_c
            - T::one();

        for _ in 0..(self.0.clone()
            / T::try_from(ln_const::S).ok().unwrap())
        .floor()
        .numer()
        .clone()
        .try_into()
        .ok()
        .unwrap()
        .min(8)
        {
            // #[cfg(debug_assertions)]
            // println!("iter {i}");
            let denom = (Ratio::from(T::try_from(1_000_000_000_000).ok().unwrap()) / x.clone().max(Ratio::one())).round().numer().clone().max(T::one());
            let exp = Self(x.clone()).limit_denom(denom.clone()).exp();
            let denom = (Ratio::from(T::try_from(1_000_000_000_000).ok().unwrap()) / exp.0.clone().max(Ratio::one())).round().numer().clone().max(T::one());
            let exp = exp.limit_denom(denom).0;
            x = x - (exp.clone() - self.0.clone()) / exp;
        }

        Some(Self(x).limit_denom(1_000_000_000_000.try_into().ok().unwrap()))
    }

    pub fn exp(self) -> Self {
        Self(
            Ratio::new_raw(
                517656.try_into().ok().unwrap(),
                190435.try_into().ok().unwrap(),
            )
            .pow(self.0.floor().numer().clone().try_into().ok().unwrap())
                * exp_corr(self.0.fract()),
        )
    }

    pub fn sin(&self) -> Self {
        let pi = Ratio::new_raw(
            312689.try_into().ok().unwrap(),
            99532.try_into().ok().unwrap(),
        );
        let halfpi = Ratio::new_raw(
            312689.try_into().ok().unwrap(),
            199064.try_into().ok().unwrap(),
        );
        let tau = Ratio::new_raw(
            312689.try_into().ok().unwrap(),
            49766.try_into().ok().unwrap(),
        );
        let halfthreepi = Ratio::new_raw(
            938067.try_into().ok().unwrap(),
            199064.try_into().ok().unwrap(),
        );

        let c = (self.0.clone() + halfpi.clone()) % tau;
        let c = if c > pi { halfthreepi - c } else { c - halfpi };

        let f = c.clone() - c.clone().pow(3_u64) / T::try_from(6).ok().unwrap()
            + c.clone().pow(5_u64) / T::try_from(120).ok().unwrap();

        Self(f)
    }

    pub fn cos(&self) -> Self {
        let halfpi = Ratio::new_raw(
            312689.try_into().ok().unwrap(),
            199064.try_into().ok().unwrap(),
        );

        Self(self.0.clone() + halfpi).sin()
    }

    pub fn tan(&self) -> Self {
        self.sin() / self.cos()
        // let pi = Ratio::new_raw(
        //     312689.try_into().ok().unwrap(),
        //     99532.try_into().ok().unwrap(),
        // );
        // let halfpi = Ratio::new_raw(
        //     312689.try_into().ok().unwrap(),
        //     199064.try_into().ok().unwrap(),
        // );

        // let x = (self.0.clone() + halfpi.clone()) % pi - halfpi;

        // let x1 = x.clone() + x.clone().pow(3_u64) / T::try_from(3).ok().unwrap();
        // let x2 = x1 + x.clone().pow(3_u64) * T::try_from(2).ok().unwrap() / T::try_from(15).ok().unwrap();
        // let x3 = x2 + x.clone().pow(5_u64) * T::try_from(17).ok().unwrap() / T::try_from(315).ok().unwrap();
        // let x4 = x3 + x.clone().pow(7_u64) * T::try_from(62).ok().unwrap() / T::try_from(2835).ok().unwrap();
        // let x5 = x4 + x.pow(9_u64) * T::try_from(1382).ok().unwrap() / T::try_from(155925).ok().unwrap();

        // Self(x5)
    }

    pub fn atan(&self) -> Self {
        // https://www.desmos.com/calculator/xcayzk5v4i

        let w = (-Ratio::new(T::one(), 100.try_into().ok().unwrap()) * self.0.abs() + Ratio::new(17.try_into().ok().unwrap(), 10.try_into().ok().unwrap())).max(Ratio::new(8.try_into().ok().unwrap(), 5.try_into().ok().unwrap()));

        let abs = self.0.abs();
        let num = w * &abs;
        let den = Ratio::new(23.try_into().ok().unwrap(), 20.try_into().ok().unwrap()) + &abs;
        Self(self.0.signum()) * Self(num / den)
    }

    pub fn atan2(&self, x: &Self) -> Self {
        let pi = Self(Ratio::new_raw(
            312689.try_into().ok().unwrap(),
            99532.try_into().ok().unwrap(),
        ));

        if x.is_zero() {
            return Self((pi.0 / (T::one() + T::one())) * self.0.signum());
        }

        let atan = (self.clone() / x.clone()).atan();

        if x >= &Self::zero() {
            atan
        } else if self >= &Self::zero() {
            atan + pi
        } else {
            atan - pi
        }
    }
}

fn exp_corr<T: Clone + Integer + TryFrom<u64> + Pow<u64, Output = T>>(r: Ratio<T>) -> Ratio<T> {
    let two = T::try_from(2_u64).ok().unwrap();
    let six = T::try_from(6_u64).ok().unwrap();
    (r.clone() * two.clone()) / (r.clone().pow(2_u64) / six - r + two) + T::one()
}

impl<T: Clone + Integer + core::fmt::Display + ToPrimitive> core::fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0.denom().is_one() {
            self.0.numer().fmt(f)
        } else if f.alternate() {
            write!(
                f,
                "{} / {} ({})",
                self.0.numer(),
                self.0.denom(),
                self.0.numer().to_f64().unwrap() / self.0.denom().to_f64().unwrap()
            )
        } else {
            write!(f, "({} / {})", self.0.numer(), self.0.denom())
        }
    }
}

impl<
        T: Clone + Integer + TryFrom<u64> + TryInto<u64> + Pow<u64, Output = T> + Signed + ToPrimitive,
    > ExecuteFunction for Rational<T>
{
    fn execute(f: &str, args: &[Self]) -> Result<Self, &'static str> {
        match (f, args.len()) {
            ("floor", 1) => Ok(Self(args[0].0.floor())),
            ("ceil", 1) => Ok(Self(args[0].0.ceil())),
            ("round", 1) => Ok(Self(args[0].0.round())),
            ("trunc", 1) => Ok(Self(args[0].0.trunc())),
            ("fract", 1) => Ok(Self(args[0].0.fract())),
            ("abs", 1) => Ok(Self(args[0].0.abs())),
            ("sqrt" | "√", 1) => Ok(args[0]
                .clone()
                .pow(Self(Ratio::new(T::one(), T::one() + T::one())))),
            ("cbrt" | "∛", 1) => Ok(args[0]
                .clone()
                .pow(Self(Ratio::new(T::one(), T::one() + T::one() + T::one())))),
            ("ln", 1) => args[0].clone().ln().ok_or("`ln` math error"),
            // ("log", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).log10()))),
            // ("log", 2) => Ok(Self(from_f64!(to_f64!(args[0].0).log(to_f64!(args[0].0))))),
            ("min", _) => Ok(Self(args.iter().map(|a| a.0.clone()).min().ok_or("expected ≥1 arguments")?)),
            ("max", _) => Ok(Self(args.iter().map(|a| a.0.clone()).max().ok_or("expected ≥1 arguments")?)),
            ("sin", 1) => Ok(args[0].sin()),
            ("cos", 1) => Ok(args[0].cos()),
            ("tan", 1) => Ok(args[0].tan()),
            // ("arcsin", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).asin()))),
            // ("arccos", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).acos()))),
            ("arctan", 1) => Ok(args[0].atan()),
            ("arctan2", 2) => Ok(args[1].atan2(&args[0])),
            // ("sinh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).sinh()))),
            // ("cosh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).cosh()))),
            // ("tanh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).tanh()))),
            // ("arcsinh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).asinh()))),
            // ("arccosh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).acosh()))),
            // ("arctanh", 1) => Ok(Self(from_f64!(to_f64!(args[0].0).atanh()))),
            _ => Err("function not supported"),
        }
    }
}
