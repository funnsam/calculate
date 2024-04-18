use calculate::{*, traits::*};
use fraction::*;
use std::ops::*;

type Rational = GenericFraction<BigUint>;

#[derive(Clone)]
pub struct InfPrecNumber {
    factors: Vec<Factor>,
}

#[repr(u8)]
#[derive(Clone)]
enum Factor {
    Rational(Rational),
    Pi(Rational),
    E(Rational),
    GoldenRatio(Rational),
    EulersConstant(Rational),
    IrrationalRoot(Rational, Rational),
}

impl Factor {
    fn key(&self) -> u8 {
        unsafe { *(self as *const Factor as *const u8) }
    }
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
            NodeKind::BiOp(l, op, r) => binop(*op, &l.evaluate(), &r.evaluate()),
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

lazy_static::lazy_static! {
    static ref ZERO: Rational = Rational::zero();
}

impl Add for &InfPrecNumber {
    type Output = InfPrecNumber;

    fn add(self, r: &InfPrecNumber) -> InfPrecNumber {
        let mut fac = Vec::new();
        for f in self.factors.iter().chain(r.factors.iter()) {
            match f {
                Factor::Rational(n) => fac.push(
                    Factor::Rational(n + match r.factors.iter().find(|a| matches!(a, Factor::Rational(..))) {
                        Some(Factor::Rational(n)) => n,
                        None => &ZERO,
                        _ => unreachable!(),
                    })
                ),
                _ => todo!(),
            }
        }

        fac.dedup_by_key(|a| a.key());
        InfPrecNumber { factors: fac }
    }
}
