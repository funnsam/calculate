use core::str::FromStr;

#[cfg(feature = "any_num")]
pub trait ComputableNumeral:
    Clone
    + Numeral
    + num_traits::Num
    + core::ops::Neg<Output = Self>
    + num_traits::Pow<Self, Output = Self>
{
}

#[cfg(feature = "any_num")]
impl<
        T: Clone
            + Numeral
            + num_traits::Num
            + core::ops::Neg<Output = T>
            + num_traits::Pow<T, Output = T>,
    > ComputableNumeral for T
{
}

pub trait Numeral: FromStr + FromConstant {}

impl<T: FromStr + FromConstant> Numeral for T {}

pub trait FromConstant
where
    Self: Sized,
{
    fn from_constant(_c: &str) -> Option<Self> { None }
}

impl FromConstant for f32 {
    fn from_constant(c: &str) -> Option<Self> {
        match c {
            "π" => Some(core::f32::consts::PI),
            "φ" | "ϕ" => Some(1.61803398874989484820),
            "e" => Some(core::f32::consts::E),
            "τ" => Some(core::f32::consts::TAU),
            "γ" => Some(0.57721566490153286060),
            _ => None,
        }
    }
}

impl FromConstant for f64 {
    fn from_constant(c: &str) -> Option<Self> {
        match c {
            "π" => Some(core::f64::consts::PI),
            "φ" | "ϕ" => Some(1.61803398874989484820),
            "e" => Some(core::f64::consts::E),
            "τ" => Some(core::f64::consts::TAU),
            "γ" => Some(0.57721566490153286060),
            _ => None,
        }
    }
}

#[cfg(feature = "num_complex")]
impl<T: FromConstant + num_traits::Zero + num_traits::One> FromConstant for num_complex::Complex<T> {
    fn from_constant(c: &str) -> Option<Self> {
        match c {
            "i" => Some(num_complex::Complex::new(T::zero(), T::one())),
            _ => Some(num_complex::Complex::new(T::from_constant(c)?, T::zero())),
        }
    }
}
