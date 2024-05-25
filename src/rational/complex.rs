use super::*;
use num_complex::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ComplexRational<T: Clone + Integer>(pub Complex<Ratio<T>>);

impl<T: Clone + Integer + From<u8> + AddAssign + MulAssign> core::str::FromStr
    for ComplexRational<T>
{
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
            "i" => Some(Self(Complex::i())),
            _ => Rational::from_constant(c).map(|i| ComplexRational(Complex::new(i.0, Ratio::zero()))),
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

    fn neg(self) -> Self { Self(self.0.neg()) }
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

impl<
        T: Clone
            + Integer
            + Zero
            + ToPrimitive
            + Signed
            + From<i64>
            + TryFrom<u64>
            + TryInto<u64>
            + Pow<u64, Output = T>,
    > Pow<Self> for ComplexRational<T>
{
    type Output = Self;

    fn pow(self, exp: Self) -> Self {
        if exp.is_zero() {
            return Self::one();
        }

        // TODO: make ln not fail
        (exp * self.ln().unwrap()).exp()
    }
}

impl<
        T: Clone
            + Integer
            + Zero
            + ToPrimitive
            + Signed
            + From<i64>
            + TryFrom<u64>
            + TryInto<u64>
            + Pow<u64, Output = T>,
    > ComplexRational<T>
{
    pub fn exp(self) -> Self {
        let Complex { re, im } = self.0;
        Self::from_polar(Rational(re).exp().0, im)
    }

    pub fn from_polar(r: Ratio<T>, t: Ratio<T>) -> Self {
        let _t = Rational(t);
        let sin = _t.sin().0;
        let cos = _t.cos().0;

        Self(Complex::new(r.clone() * cos, r * sin))
    }

    pub fn to_polar(self) -> (Ratio<T>, Ratio<T>) {
        let re = Rational(self.0.re);
        let im = Rational(self.0.im);
        let atan = im.atan2(&re);
        let re = re.0;
        let im = im.0;

        (
            Rational(re.clone() * re + im.clone() * im)
                .pow(Rational(Ratio::new_raw(1.into(), 2.into())))
                .0,
            atan.0,
        )
    }

    pub fn ln(self) -> Option<Self> {
        let (r, t) = self.to_polar();
        Some(Self(Complex::new(Rational(r).ln()?.0, t)))
    }
}

impl<T: Clone + Integer + Signed + core::fmt::Display + ToPrimitive> core::fmt::Display
    for ComplexRational<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0.im.is_zero() {
            Rational(self.0.re.clone()).fmt(f)
        } else if self.0.re.is_zero() {
            write!(f, "{}i", Rational(self.0.im.clone()))
        } else if !self.0.im.is_negative() {
            write!(
                f,
                "{}+{}i",
                Rational(self.0.re.clone()),
                Rational(self.0.im.clone())
            )
        } else {
            write!(
                f,
                "{}-{}i",
                Rational(self.0.re.clone()),
                Rational(self.0.im.abs().clone())
            )
        }
    }
}

impl<
        T: Clone
            + Integer
            + Zero
            + ToPrimitive
            + Signed
            + From<i64>
            + TryFrom<u64>
            + TryInto<u64>
            + Pow<u64, Output = T>,
    > ExecuteFunction for ComplexRational<T>
{
    fn execute(f: &str, args: &[Self]) -> Result<Self, &'static str> {
        match (f, args.len()) {
            ("conj", 1) => Ok(Self(Complex::new(
                args[0].0.re.clone(),
                -args[0].0.im.clone(),
            ))),
            ("ln", 1) => Ok(args[0].clone().ln().ok_or("`ln` math error")?),
            ("exp", 1) => Ok(args[0].clone().exp()),
            _ => Err("function not supported"),
        }
    }
}

impl<T: Clone + Integer> ComplexRational<T> {
    pub fn limit_denom(&self, md: T) -> Self {
        Self(Complex::new(Rational(self.0.re.clone()).limit_denom(md.clone()).0, Rational(self.0.im.clone()).limit_denom(md).0))
    }
}
