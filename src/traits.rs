use core::str::FromStr;

#[cfg(feature = "any_num")]
pub trait ComputableNumeral:
    Clone
    + Numeral
    + ExecuteFunction
    + num_traits::Num
    + core::ops::Neg<Output = Self>
    + num_traits::Pow<Self, Output = Self>
{
}

#[cfg(feature = "any_num")]
impl<
        T: Clone
            + Numeral
            + ExecuteFunction
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
            "c_m/s" => Some(299792458.0),
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
            "c_m/s" => Some(299792458.0),
            _ => None,
        }
    }
}

#[cfg(feature = "num_complex")]
impl<T: FromConstant + num_traits::Zero + num_traits::One> FromConstant
    for num_complex::Complex<T>
{
    fn from_constant(c: &str) -> Option<Self> {
        match c {
            "i" => Some(num_complex::Complex::new(T::zero(), T::one())),
            _ => Some(num_complex::Complex::new(T::from_constant(c)?, T::zero())),
        }
    }
}

pub trait ExecuteFunction
where
    Self: Sized,
{
    fn execute(f: &str, args: &[Self]) -> Result<Self, &'static str>;
}

macro_rules! map_fn {
    ($type: ty: $($n: pat $(= $ac: tt => $map: tt ($($th: tt)*))? $(=> $f: expr)?),* $(,)?) => {
        impl ExecuteFunction for $type {
            fn execute(f: &str, args: &[Self]) -> Result<Self, &'static str> {
                fn check(v: $type) -> Result<$type, &'static str> {
                    v.is_finite().then_some(v).ok_or("number is not finite")
                }

                match (f, args.len()) {
                    $(
                        $(($n, $ac) => check(Self::$map($(args[$th]),*)),)?
                        $(($n, _) => $f(args),)?
                    )*
                    _ => Err("function not supported"),
                }
            }
        }
    };
}

macro_rules! map_fns {
    ($($t: tt)*) => {
        map_fn!(f32: $($t)*);
        map_fn!(f64: $($t)*);
    };
}

map_fns!(
    "floor" = 1 => floor(0),
    "ceil" = 1 => ceil(0),
    "round" = 1 => round(0),
    "trunc" = 1 => trunc(0),
    "fract" = 1 => fract(0),
    "abs" = 1 => abs(0),
    "sqrt" | "√" = 1 => sqrt(0),
    "ln" = 1 => ln(0),
    "log" = 1 => log10(0),
    "log" = 2 => log(0 1),
    "min" => |args: &[_]| if args.len() != 0 {
        Ok(args.iter().fold(Self::INFINITY, |a, &b| a.min(b)))
    } else {
        Err("expect ≥1 arguments")
    },
    "max" => |args: &[_]| if args.len() != 0 {
        Ok(args.iter().fold(Self::NEG_INFINITY, |a, &b| a.max(b)))
    } else {
        Err("expect ≥1 arguments")
    },
    "cbrt" | "∛" = 1 => cbrt(0),
    "sin" = 1 => sin(0),
    "cos" = 1 => cos(0),
    "tan" = 1 => tan(0),
    "arcsin" = 1 => asin(0),
    "arccos" = 1 => acos(0),
    "arctan" = 1 => atan(0),
    "sinh" = 1 => sinh(0),
    "cosh" = 1 => cosh(0),
    "tanh" = 1 => tanh(0),
    "arcsinh" = 1 => asinh(0),
    "arccosh" = 1 => acosh(0),
    "arctanh" = 1 => atanh(0),
);
