#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod traits;

#[cfg(feature = "num_rational")]
pub mod rational;

pub mod error;
pub use error::*;
pub mod latex;

use alloc::{boxed::Box, string::ToString};
use traits::*;

/// A span in `char`s
pub type Span = core::ops::Range<usize>;

type PeekingLexer<'src, Number> = Peeking<Lexer<'src, Number>, Result<Token<Number>, Error>>;

pub fn to_nodes<T: Clone + Numeral>(s: &str) -> Result<Node<T>, Error> {
    let lex = Lexer::<T> {
        source: s.chars(),
        start_index: 0,
        current_idx: 0,
        skipped: None,

        _num: core::marker::PhantomData::default(),
    };
    let mut lex = Peeking::from_iter(lex);

    let e = parse_expr_climb(&mut lex, 0)?;

    if lex.next().is_some() {
        return Err(Error {
            message: "expected end of expression",
            location: lex.report_span(),
        });
    }

    Ok(e)
}

fn parse_expr_climb<T: Clone + Numeral>(
    lex: &mut PeekingLexer<'_, T>,
    percedence: usize,
) -> Result<Node<T>, Error> {
    let mut rest = parse_single(lex)?;
    let mut depth = 0;

    loop {
        match lex.peek() {
            Some(Ok(Token::Operator(op))) if op.binary().is_some() => {
                let op = op.binary().unwrap();

                if op.percedence() < percedence {
                    break;
                }

                lex.next();

                let rhs =
                    parse_expr_climb(lex, op.percedence() + op.is_left_associative() as usize)?;

                let rest_start = rest.span.start;
                let rhs_end = rhs.span.end;

                if op.is_left_associative() {
                    rest = Node {
                        kind: NodeKind::BiOp(Box::new(rest), op, Box::new(rhs)),
                        span: rest_start..rhs_end,
                    };
                } else {
                    add_node_right(&mut rest, depth, op, rhs);
                }
            },
            Some(Ok(Token::FStart(..) | Token::BStart(..) | Token::Number(..)))
                if BiOpr::Multiply.percedence() >= percedence =>
            {
                let rhs = parse_expr_climb(
                    lex,
                    BiOpr::Multiply.percedence() + BiOpr::Multiply.is_left_associative() as usize,
                )?;

                let rest_start = rest.span.start;
                let rhs_end = rhs.span.end;

                rest = Node {
                    kind: NodeKind::BiOp(Box::new(rest), BiOpr::Multiply, Box::new(rhs)),
                    span: rest_start..rhs_end,
                };
            },
            _ => break,
        }

        depth += 1;
    }

    Ok(rest)
}

fn add_node_right<T: Clone>(rest: &mut Node<T>, depth: usize, op: BiOpr, right: Node<T>) {
    let rse = right.span.end;

    if depth == 0 {
        *rest = Node {
            span: rest.span.start..right.span.end,
            kind: NodeKind::BiOp(Box::new(rest.clone()), op, Box::new(right)),
        };

        return;
    }

    match &mut rest.kind {
        NodeKind::BiOp(_, op2, r) if op == *op2 => {
            add_node_right(r, depth - 1, op, right);
        },
        _ => {
            *rest = Node {
                span: rest.span.start..right.span.end,
                kind: NodeKind::BiOp(Box::new(rest.clone()), op, Box::new(right)),
            };
        },
    }

    rest.span.end = rse;
}

fn parse_single<T: Clone + Numeral>(lex: &mut PeekingLexer<'_, T>) -> Result<Node<T>, Error> {
    let t = lex.next();

    match t.ok_or_else(|| Error {
        message: "unexpected end of expression",
        location: lex.report_span(),
    })?? {
        Token::Number(num) => Ok(Node {
            kind: NodeKind::Number(num),
            span: lex.report_span(),
        }),
        Token::BStart(k) => {
            let sbs = lex.report_span();
            let inner = parse_expr_climb(lex, 0)?;
            if let Some(Ok(Token::BEnd(ke))) = lex.next() {
                if k == ke {
                    Ok(Node {
                        kind: inner.kind,
                        span: sbs.start..lex.report_span().end,
                    })
                } else {
                    Err(Error {
                        message: "bracket type mismatch",
                        location: lex.report_span(),
                    })
                }
            } else {
                Err(Error {
                    message: "expected vracket end",
                    location: lex.report_span(),
                })
            }
        },
        Token::FStart(f, k) => {
            let sbs = lex.report_span();
            let mut args = Vec::new();

            while let Some(t) = lex.peek() {
                match t {
                    Ok(Token::BEnd(_)) => break,
                    _ => args.push(parse_expr_climb(lex, 0)?),
                }

                match lex.peek() {
                    Some(Ok(Token::BEnd(_))) => break,
                    Some(Ok(Token::Comma)) => {
                        lex.next();
                    },
                    _ => {
                        lex.next();
                        return Err(Error {
                            message: "expected comma or bracket end",
                            location: lex.report_span(),
                        });
                    },
                }
            }

            if let Some(Ok(Token::BEnd(ke))) = lex.next() {
                if k == ke {
                    Ok(Node {
                        kind: NodeKind::Function(f, args),
                        span: sbs.start..lex.report_span().end,
                    })
                } else {
                    Err(Error {
                        message: "bracket type mismatch",
                        location: lex.report_span(),
                    })
                }
            } else {
                Err(Error {
                    message: "expected bracket end",
                    location: lex.report_span(),
                })
            }
        },
        Token::Operator(op) if op.unary().is_some() => {
            let op_span = lex.report_span();
            let op = op.unary().unwrap();
            let expr = parse_expr_climb(lex, op.percedence())?;
            let expr_end = expr.span.end;
            Ok(Node {
                kind: NodeKind::UnOp(op, Box::new(expr)),
                span: op_span.start..expr_end,
            })
        },
        _ => Err(Error {
            message: "did not expect this",
            location: lex.report_span(),
        }),
    }
}

#[derive(Debug, Clone)]
pub struct Node<Number> {
    pub kind: NodeKind<Number>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum NodeKind<Number> {
    Number(Number),
    BiOp(Box<Node<Number>>, BiOpr, Box<Node<Number>>),
    UnOp(UnOpr, Box<Node<Number>>),
    Function(String, Vec<Node<Number>>),
}

#[derive(Debug, Clone, Copy)]
enum OperatorRaw {
    Plus,
    Minus,
    Multiply,
    Divide,
    PercentageSign,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiOpr {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOpr {
    Plus,
    Minus,
}

impl OperatorRaw {
    fn binary(self) -> Option<BiOpr> {
        match self {
            Self::Plus => Some(BiOpr::Add),
            Self::Minus => Some(BiOpr::Subtract),
            Self::Multiply => Some(BiOpr::Multiply),
            Self::Divide => Some(BiOpr::Divide),
            Self::PercentageSign => Some(BiOpr::Modulo),
            Self::Power => Some(BiOpr::Power),
        }
    }

    fn unary(self) -> Option<UnOpr> {
        match self {
            Self::Plus => Some(UnOpr::Plus),
            Self::Minus => Some(UnOpr::Minus),
            _ => None,
        }
    }
}

impl BiOpr {
    fn percedence(self) -> usize {
        match self {
            Self::Power => 3,
            Self::Multiply | Self::Divide | Self::Modulo => 2,
            Self::Add | Self::Subtract => 1,
        }
    }

    fn is_left_associative(self) -> bool { !matches!(self, Self::Power) }

    #[cfg(not(feature = "any_num"))]
    fn operate(self, l: f32, r: f32) -> f32 {
        match self {
            Self::Add => l + r,
            Self::Subtract => l - r,
            Self::Multiply => l * r,
            Self::Divide => l / r,
            Self::Modulo => l % r,
            #[cfg(feature = "std")]
            Self::Power => l.powf(r),
            #[cfg(not(feature = "std"))]
            Self::Power => unimplemented!(),
        }
    }

    #[cfg(feature = "any_num")]
    fn operate<F: ComputableNumeral>(self, l: F, r: F) -> Result<F, &'static str> {
        match self {
            Self::Add => Ok(l + r),
            Self::Subtract => Ok(l - r),
            Self::Multiply => Ok(l * r),
            Self::Divide => {
                if !r.is_zero() {
                    Ok(l / r)
                } else {
                    Err("division by zero")
                }
            },
            Self::Modulo => Ok(l % r),
            Self::Power => Ok(l.pow(r)),
        }
    }
}

impl UnOpr {
    fn percedence(self) -> usize {
        match self {
            Self::Plus | Self::Minus => 4,
        }
    }

    #[cfg(not(feature = "any_num"))]
    fn operate(self, v: f32) -> f32 {
        match self {
            Self::Plus => v,
            Self::Minus => -v,
        }
    }

    #[cfg(feature = "any_num")]
    fn operate<F: ComputableNumeral>(self, v: F) -> Result<F, &'static str> {
        match self {
            Self::Plus => Ok(v),
            Self::Minus => Ok(-v),
        }
    }
}

#[cfg(not(feature = "any_num"))]
impl Node<f32> {
    pub fn evaluate(&self) -> f32 {
        match &self.kind {
            NodeKind::BiOp(l, op, r) => op.operate(l.evaluate(), r.evaluate()),
            NodeKind::UnOp(op, v) => op.operate(v.evaluate()),
            NodeKind::Number(v) => *v,
        }
    }
}

#[cfg(feature = "any_num")]
impl<F: ComputableNumeral> Node<F> {
    pub fn evaluate(&self) -> Result<F, Error> {
        match &self.kind {
            NodeKind::BiOp(l, op, r) => op.operate(l.evaluate()?, r.evaluate()?).map_err(|message| Error {
                message,
                location: self.span.clone(),
            }),
            NodeKind::UnOp(op, v) => op.operate(v.evaluate()?).map_err(|message| Error {
                message,
                location: self.span.clone(),
            }),
            NodeKind::Number(v) => Ok(v.clone()),
            NodeKind::Function(f, a) => {
                let mut av = Vec::with_capacity(a.len());
                for i in a.iter() {
                    av.push(i.evaluate()?);
                }

                F::execute(f, &av).map_err(|message| Error {
                    message,
                    location: self.span.clone(),
                })
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Token<Number> {
    Operator(OperatorRaw),
    Number(Number),
    BStart(BKind),
    BEnd(BKind),
    Comma,
    FStart(String, BKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BKind {
    Round,
    Square,
    Curly,
}

struct Lexer<'src, Number> {
    source: core::str::Chars<'src>,
    start_index: usize,
    current_idx: usize,
    skipped: Option<char>,

    _num: core::marker::PhantomData<Number>,
}

impl<Number: Numeral> Iterator for Lexer<'_, Number> {
    type Item = Result<Token<Number>, Error>;

    fn next(&mut self) -> Option<Result<Token<Number>, Error>> {
        self.start_index = self.current_idx;
        let c = self.next_char()?;

        match c {
            '+' => Some(Ok(Token::Operator(OperatorRaw::Plus))),
            '-' => Some(Ok(Token::Operator(OperatorRaw::Minus))),
            '*' | '×' => Some(Ok(Token::Operator(OperatorRaw::Multiply))),
            '/' | '÷' => Some(Ok(Token::Operator(OperatorRaw::Divide))),
            '%' => Some(Ok(Token::Operator(OperatorRaw::PercentageSign))),
            '^' => Some(Ok(Token::Operator(OperatorRaw::Power))),
            ',' => Some(Ok(Token::Comma)),
            '(' | '[' | '{' => Some(Ok(Token::BStart(btype(c).unwrap()))),
            ')' | ']' | '}' => Some(Ok(Token::BEnd(btype(c).unwrap()))),
            '0'..='9' | '.' => {
                let mut acc = c.to_string();

                while let Some(c) = self.peek_char() {
                    if matches!(c, '0'..='9' | '.') {
                        acc.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }

                Some(
                    acc.parse()
                        .map_or_else(|_| Err(Error {
                            message: "number format is incorrect",
                            location: self.report_span(),
                        }), |a| Ok(Token::Number(a))),
                )
            },
            _ if c.is_whitespace() => self.next(),
            _ => {
                let mut s = c.to_string();

                if let Some(c) = Number::from_constant(repl_greeks(&s)) {
                    return Some(Ok(Token::Number(c)));
                }

                while let Some(c) = self.next_char() {
                    if c.is_whitespace() {
                        break;
                    }

                    if matches!(c, '(' | '[' | '{') {
                        return Some(Ok(Token::FStart(
                            repl_greeks(&s).to_string(),
                            btype(c).unwrap(),
                        )));
                    }

                    s.push(c);

                    let s = repl_greeks(&s);
                    if let Some(c) = Number::from_constant(s) {
                        return Some(Ok(Token::Number(c)));
                    }
                }

                Some(Err(Error {
                    message: "this constant is not supported",
                    location: self.report_span(),
                }))
            },
        }
    }
}

fn btype(c: char) -> Option<BKind> {
    match c {
        '(' | ')' => Some(BKind::Round),
        '[' | ']' => Some(BKind::Square),
        '{' | '}' => Some(BKind::Curly),
        _ => None,
    }
}

fn repl_greeks(s: &str) -> &str {
    match s {
        "\\alpha" => "α",
        "\\Alpha" => "Α",
        "\\beta" => "β",
        "\\Beta" => "Β",
        "\\gamma" => "γ",
        "\\Gamma" => "Γ",
        "\\delta" => "δ",
        "\\Delta" => "Δ",
        "\\epsilon" => "ε",
        "\\Epsilon" => "Ε",
        "\\zeta" => "ζ",
        "\\Zeta" => "Ζ",
        "\\eta" => "η",
        "\\Eta" => "Η",
        "\\theta" => "θ",
        "\\Theta" => "Θ",
        "\\iota" => "ι",
        "\\Iota" => "Ι",
        "\\kappa" => "κ",
        "\\Kappa" => "Κ",
        "\\lambda" => "λ",
        "\\Lambda" => "Λ",
        "\\mu" => "μ",
        "\\Mu" => "Μ",
        "\\nu" => "ν",
        "\\Nu" => "Ν",
        "\\xi" => "ξ",
        "\\Xi" => "Ξ",
        "\\omicron" => "ο",
        "\\Omicron" => "Ο",
        "\\pi" => "π",
        "\\Pi" => "Π",
        "\\rho" => "ρ",
        "\\Rho" => "Ρ",
        "\\sigma" => "σ",
        "\\Sigma" => "Σ",
        "\\tau" => "τ",
        "\\Tau" => "Τ",
        "\\upsilon" => "υ",
        "\\Upsilon" => "Υ",
        "\\phi" => "φ",
        "\\Phi" => "Φ",
        "\\chi" => "χ",
        "\\Chi" => "Χ",
        "\\psi" => "ψ",
        "\\Psi" => "Ψ",
        "\\omega" => "ω",
        "\\Omega" => "Ω",

        "\\varpi" => "ϖ",
        "\\varphi" => "ϕ",
        "\\varkai" => "ϗ",
        "\\varsigma" => "ς",
        "\\stigma" => "ҁ",
        "\\Stigma" => "Ҁ",
        "\\digamma" => "ϝ",
        "\\Digamma" => "Ϝ",
        "\\koppa" => "ϟ",
        "\\Koppa" => "Ϟ",
        "\\sampi" => "ϡ",
        "\\Sampi" => "Ϡ",
        _ => s,
    }
}

impl<Number> Lexer<'_, Number> {
    fn next_char(&mut self) -> Option<char> {
        if self.skipped.is_some() {
            self.current_idx += 1;
            return core::mem::take(&mut self.skipped);
        }

        self.source.next().map(|a| {
            self.current_idx += 1;
            a
        })
    }

    fn peek_char(&mut self) -> Option<char> {
        if let Some(c) = self.skipped {
            return Some(c);
        }

        let c = self.source.next()?;
        self.skipped = Some(c);
        Some(c)
    }

    fn report_span(&self) -> Span { self.start_index..self.current_idx.max(self.start_index + 1) }
}

struct Peeking<Inner, Item> {
    inner: Inner,
    peeked: Option<Item>,
}

impl<Inner: Iterator<Item = Item>, Item: Clone> Peeking<Inner, Item> {
    fn peek(&mut self) -> Option<Item> {
        if self.peeked.is_none() {
            let next = self.next();
            self.peeked = next.clone();
            next
        } else {
            self.peeked.clone()
        }
    }

    fn from_iter(inner: Inner) -> Self {
        Self {
            inner,
            peeked: None,
        }
    }
}

impl<Inner: Iterator<Item = Item>, Item> Iterator for Peeking<Inner, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        if self.peeked.is_some() {
            core::mem::take(&mut self.peeked)
        } else {
            self.inner.next()
        }
    }
}

impl<Inner: Iterator<Item = Item>, Item> core::ops::Deref for Peeking<Inner, Item> {
    type Target = Inner;

    fn deref(&self) -> &Inner { &self.inner }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests() {
        let lex = crate::Lexer::<f64> {
            source: "(3(0.1+0.2)-0.9".chars(),
            start_index: 0,
            current_idx: 0,
            skipped: None,

            _num: core::marker::PhantomData::default(),
        };
        let mut lex = crate::Peeking::from_iter(lex);

        while let Some(t) = lex.next() {
            // println!("{t:?} {:?} {:?} {:?}", lex.inner.report_span(), lex.peek(),
            // lex.inner.report_span());
            println!("{t:?} {:?}", lex.inner.report_span());
        }

        println!("{:?}", lex.inner.report_span());
        lex.peek();
        println!("{:?}", lex.inner.report_span());
    }
}
