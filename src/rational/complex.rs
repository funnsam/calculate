use super::*;
use num_complex::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ComplexRational<T: Clone + Integer>(pub Complex<Ratio<T>>);

impl<T: Clone + Integer + From<u8> + AddAssign + MulAssign> core::str::FromStr for ComplexRational<T> {
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

        Ok(Self(Complex::new(Ratio::new(numer, denom), Ratio::zero())))
    }
}

impl<T: Clone + Integer + From<usize>> FromConstant for ComplexRational<T> {
    fn from_constant(c: &str) -> Option<Self> {
        match c {
            "π" => Some(Self(Complex::new(Ratio::new_raw(312689.into(), 99532.into()), Ratio::zero()))),
            "φ" | "ϕ" => Some(Self(Complex::new(Ratio::new_raw(121393.into(), 75025.into()), Ratio::zero()))),
            "e" => Some(Self(Complex::new(Ratio::new_raw(517656.into(), 190435.into()), Ratio::zero()))),
            "τ" => Some(Self(Complex::new(Ratio::new_raw(312689.into(), 49766.into()), Ratio::zero()))),
            "γ" => Some(Self(Complex::new(Ratio::new_raw(30316449.into(), 52521875.into()), Ratio::zero()))),
            "c_m/s" => Some(Self(Complex::new(Ratio::new_raw(299792458.into(), 1.into()), Ratio::zero()))),
            "i" => Some(Self(Complex::new(Ratio::zero(), Ratio::one()))),
            _ => None,
        }
    }
}

delegate_biop!(ComplexRational, Add, add);
delegate_biop!(ComplexRational, Sub, sub);
delegate_biop!(ComplexRational, Mul, mul);
delegate_biop!(ComplexRational, Div, div);
delegate_biop!(ComplexRational, Rem, rem);

impl<T: Clone + Integer + Neg<Output = T>> Neg for ComplexRational<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl<T: Clone + Integer + From<u8> + AddAssign + MulAssign> Num for ComplexRational<T> {
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

impl<T: Clone + Integer> Zero for ComplexRational<T> {
    fn zero() -> Self { Self(Complex::zero()) }

    fn is_zero(&self) -> bool { self.0.is_zero() }

    fn set_zero(&mut self) { self.0.set_zero() }
}

impl<T: Clone + Integer> One for ComplexRational<T> {
    fn one() -> Self { Self(Complex::one()) }

    fn is_one(&self) -> bool { self.0.is_one() }

    fn set_one(&mut self) { self.0.set_one() }
}

impl<T: std::fmt::Debug+ Clone + Integer + Zero + ToPrimitive + Signed + From<i64> + TryFrom<u64> + TryInto<u64> + Pow<u64, Output = T>> Pow<Self> for ComplexRational<T> {
    type Output = Self;

    fn pow(self, exp: Self) -> Self {
        if exp.is_zero() {
            return Self::one();
        }

        (exp * self.ln()).exp()
    }
}

impl<T:std::fmt::Debug+ Clone + Integer + Zero + ToPrimitive + Signed + From<i64> + TryFrom<u64> + TryInto<u64> + Pow<u64, Output = T>> ComplexRational<T> {
    pub fn exp(self) -> Self {
        let Complex { re, im } = self.0;
        Self::from_polar(exp_approx(re), im)
    }

    pub fn from_polar(r: Ratio<T>, t: Ratio<T>) -> Self {
        let (sin, cos) = to_f64!(t).sin_cos();
        let sin = from_f64!(sin);
        let cos = from_f64!(cos);

        Self(Complex::new(r.clone() * cos, r * sin))
    }

    pub fn to_polar(self) -> (Ratio<T>, Ratio<T>) {
        let re = to_f64!(self.0.re);
        let im = to_f64!(self.0.im);
        let atan = from_f64!((im / re).atan());

        (Rational(self.0.re.clone() * self.0.re + self.0.im.clone() * self.0.im).pow(Rational(Ratio::new_raw(1.into(), 2.into()))).0, atan)
    }

    pub fn ln(self) -> Self {
        let (r, t) = self.to_polar();
        Self(Complex::new(from_f64!(to_f64!(r).ln()), t))
    }
}

impl<T: Clone + Integer + Signed + core::fmt::Display + ToPrimitive> core::fmt::Display for ComplexRational<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if !self.0.im.is_negative() {
            write!(f, "{}+{}i", Rational(self.0.re.clone()), Rational(self.0.im.clone()))
        } else {
            write!(f, "{}-{}i", Rational(self.0.re.clone()), Rational(self.0.im.abs().clone()))
        }
    }
}
