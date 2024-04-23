use calculate::{traits::*, *};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::*;
use num_complex::Complex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsSpan {
    pub start: usize,
    pub end: usize,
}

fn evaluate<T: ComputableNumeral + std::string::ToString>(s: &str) -> Result<String, JsSpan> {
    Ok(to_nodes::<T>(s)
        .map_err(|s| JsSpan {
            start: s.start,
            end: s.end,
        })?
        .evaluate()
        .map_err(|s| JsSpan {
            start: s.start,
            end: s.end,
        })?
        .to_string())
}

#[wasm_bindgen]
pub fn evaluate_f32(s: &str) -> Result<String, JsSpan> {
    evaluate::<f32>(s)
}

#[wasm_bindgen]
pub fn evaluate_f64(s: &str) -> Result<String, JsSpan> {
    evaluate::<f64>(s)
}

#[wasm_bindgen]
pub fn evaluate_rational(s: &str) -> Result<String, JsSpan> {
    evaluate::<Rat>(s)
}

#[wasm_bindgen]
pub fn evaluate_cmplx_f32(s: &str) -> Result<String, JsSpan> {
    evaluate::<Complex<f32>>(s)
}

#[wasm_bindgen]
pub fn evaluate_cmplx_f64(s: &str) -> Result<String, JsSpan> {
    evaluate::<Complex<f64>>(s)
}

// #[wasm_bindgen]
// pub fn evaluate_cmplx_rational(s: &str) -> Result<String, JsSpan> {
//     evaluate::<Complex<Rat>>(s)
// }

use core::ops::*;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rat(BigRational);

impl std::str::FromStr for Rat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut numer = BigInt::from(0);
        let mut denom = BigInt::from(1);
        let mut di = false;

        for c in s.chars() {
            match (c, di) {
                ('0'..='9', _) => {
                    numer *= 10;
                    numer += c as u8 - '0' as u8;

                    if di {
                        denom *= 10;
                    }
                },
                ('.', false) => di = true,
                _ => return Err(()),
            }
        }

        Ok(Self(BigRational::new(numer, denom)))
    }
}

impl FromConstant for Rat {
    fn from_constant(c: char) -> Option<Self> {
        match c {
            'π' => Some(Self(BigRational::new_raw(312689.into(), 99532.into()))),
            'φ' | 'ϕ' => Some(Self(BigRational::new_raw(121393.into(), 75025.into()))),
            'e' => Some(Self(BigRational::new_raw(517656.into(), 190435.into()))),
            'τ' => Some(Self(BigRational::new_raw(312689.into(), 49766.into()))),
            'γ' => Some(Self(BigRational::new_raw(30316449.into(), 52521875.into()))),
            _ => None,
        }
    }
}

macro_rules! delegate_biop {
    ($t: path, $f: ident) => {
        impl $t for Rat {
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

impl Neg for Rat {
    type Output = Self;

    fn neg(self) -> Self { Self(-self.0) }
}

impl Num for Rat {
    type FromStrRadixErr = ();

    fn from_str_radix(s: &str, rad: u32) -> Result<Self, ()> {
        if rad == 10 {
            use std::str::FromStr;

            Self::from_str(s)
        } else {
            Err(())
        }
    }
}

impl Zero for Rat {
    fn zero() -> Self { Self(BigRational::zero()) }

    fn is_zero(&self) -> bool { self.0.is_zero() }

    fn set_zero(&mut self) { self.0.set_zero() }
}

impl One for Rat {
    fn one() -> Self { Self(BigRational::one()) }

    fn is_one(&self) -> bool { self.0.is_one() }

    fn set_one(&mut self) { self.0.set_one() }
}

impl Pow<Self> for Rat {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self {
        if rhs.0.is_integer() {
            return Self(self.0.pow(rhs.0.to_integer()));
        } else if self.0 == BigRational::from(BigInt::from(1)) {
            return self;
        }

        todo!();
    }
}

impl std::fmt::Display for Rat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.0.numer().to_f64().unwrap() / self.0.denom().to_f64().unwrap()).fmt(f)
    }
}
