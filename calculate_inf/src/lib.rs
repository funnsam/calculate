use calculate::{*, traits::*};
use fraction::*;
use std::ops::*;

type Rational = GenericFraction<BigUint>;

#[derive(Clone)]
pub struct InfPrecNumber {
    factors: Vec<Factor>,
}

#[derive(Clone)]
enum Factor {
    Rational(Rational),
    Pi(Rational),
    E(Rational),
    GoldenRatio(Rational),
    EulersConstant(Rational),
    IrrationalRoot(Rational, Rational),
}

impl std::str::FromStr for InfPrecNumber {
    type Err = <Rational as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(Self {
            factors: vec![Factor::Rational(s.parse()?)],
        })
    }
}

impl FromConstant for InfPrecNumber {
    fn from_constant(c: char) -> Option<Self> {
        match c {
            'π' => Some(Self { factors: vec![Factor::Pi(Rational::from(1))] }),
            'e' => Some(Self { factors: vec![Factor::E(Rational::from(1))] }),
            'φ' => Some(Self { factors: vec![Factor::GoldenRatio(Rational::from(1))] }),
            'ψ' => Some(Self { factors: vec![Factor::EulersConstant(Rational::from(1))] }),
            _ => None,
        }
    }
}

pub trait InfPrecEval {
    fn evaluate(&self) -> InfPrecNumber;
}

impl InfPrecEval for Node<InfPrecNumber> {
    fn evaluate(&self) -> InfPrecNumber {
        match &self.kind {
            NodeKind::Number(n) => n.clone(),
            _ => todo!(),
        }
    }
}

fn binop(op: BiOpr, l: &InfPrecNumber, r: &InfPrecNumber) -> InfPrecNumber {
    match op {
        BiOpr::Add => l + r,
        _ => todo!(),
    }
}

impl Add for &InfPrecNumber {
    type Output = InfPrecNumber;

    fn add(self, r: &InfPrecNumber) -> InfPrecNumber {
    }
}
