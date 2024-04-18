use std::str::FromStr;

pub trait Numeral: FromStr + FromConstant {}

impl<T: Clone + FromStr + FromConstant> Numeral for T {}

pub trait FromConstant where Self: Sized {
    fn from_constant(_c: char) -> Option<Self> {
        None
    }
}

impl FromConstant for f32 {
    fn from_constant(c: char) -> Option<Self> {
        match c {
            'π' => Some(std::f32::consts::PI),
            'φ' => Some(1.61803398874989484820),
            'e' => Some(std::f32::consts::E),
            'τ' => Some(std::f32::consts::TAU),
            'γ' => Some(0.57721566490153286060),
            _ => None,
        }
    }
}

impl FromConstant for f64 {
    fn from_constant(c: char) -> Option<Self> {
        match c {
            'π' => Some(std::f64::consts::PI),
            'φ' => Some(1.61803398874989484820),
            'e' => Some(std::f64::consts::E),
            'τ' => Some(std::f64::consts::TAU),
            'γ' => Some(0.57721566490153286060),
            _ => None,
        }
    }
}
