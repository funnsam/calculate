pub mod traits;
use traits::*;

/// A span in `char`s
pub type Span = std::ops::Range<usize>;

type PeekingLexer<'src, Number> = Peeking<Lexer<'src, Number>, Result<Token<Number>, Span>>;

pub fn to_nodes<T: Clone + Numeral>(s: &str) -> Result<Node<T>, Span> {
    let lex = Lexer::<T> {
        source: s.chars(),
        start_index: 0,
        end_index: 0,
        skipped: None,

        _num: std::marker::PhantomData::default(),
    };
    let mut lex = Peeking::from_iter(lex);

    let e = parse_expr_climb(&mut lex, 0)?;

    if lex.next().is_some() {
        return Err(lex.report_span());
    }

    Ok(e)
}

fn parse_expr_climb<T: Clone + Numeral>(lex: &mut PeekingLexer<'_, T>, percedence: usize) -> Result<Node<T>, Span> {
    let mut rest = parse_single(lex)?;

    loop {
        match lex.peek() {
            Some(Ok(Token::Operator(op))) if op.binary().is_some() => {
                let op = op.binary().unwrap();

                if op.percedence() < percedence {
                    break;
                }

                lex.next();

                let rhs = parse_expr_climb(lex, op.percedence() + op.is_left_associative() as usize)?;

                let rest_start = rest.span.start;
                let rhs_end = rhs.span.end;

                if op.is_left_associative() {
                    rest = Node {
                        kind: NodeKind::BiOp(Box::new(rest), op, Box::new(rhs)),
                        span: rest_start..rhs_end,
                    };
                } else {
                    add_node_right(&mut rest, op, rhs);
                }
            },
            Some(Ok(Token::BStart(_) | Token::Number(_))) if BiOpr::Multiply.percedence() >= percedence => {
                let rhs = parse_expr_climb(lex, BiOpr::Multiply.percedence() + BiOpr::Multiply.is_left_associative() as usize)?;

                let rest_start = rest.span.start;
                let rhs_end = rhs.span.end;

                rest = Node {
                    kind: NodeKind::BiOp(Box::new(rest), BiOpr::Multiply, Box::new(rhs)),
                    span: rest_start..rhs_end,
                };
            },
            _ => break,
        }
    }

    Ok(rest)
}

fn add_node_right<T: Clone>(rest: &mut Node<T>, op: BiOpr, right: Node<T>) {
    match &mut rest.kind {
        NodeKind::BiOp(_, _, r) => {
            rest.span.end = right.span.end;
            add_node_right(r, op, right);
        },
        _ => {
            *rest = Node {
                span: rest.span.start..right.span.end,
                kind: NodeKind::BiOp(Box::new(rest.clone()), op, Box::new(right)),
            };
        },
    }
}

fn parse_single<T: Clone + Numeral>(lex: &mut PeekingLexer<'_, T>) -> Result<Node<T>, Span> {
    let t = lex.next();

    match t.ok_or_else(|| lex.report_span())?? {
        Token::Number(num) => Ok(Node { kind: NodeKind::Number(num), span: lex.report_span() }),
        Token::BStart(k) => {
            let inner = parse_expr_climb(lex, 0)?;
            if let Some(Ok(Token::BEnd(ke))) = lex.next() {
                if k == ke {
                    Ok(inner)
                } else {
                    Err(lex.report_span())
                }
            } else {
                Err(lex.report_span())
            }
        },
        Token::Operator(op) if op.unary().is_some() => {
            let op_span = lex.report_span();
            let op = op.unary().unwrap();
            let expr = parse_expr_climb(lex, op.percedence())?;
            let expr_end = expr.span.end;
            Ok(Node { kind: NodeKind::UnOp(op, Box::new(expr)), span: op_span.start..expr_end })
        },
        _ => Err(lex.report_span()),
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
}

#[derive(Debug, Clone, Copy)]
enum OperatorRaw {
    Plus, Minus, Multiply, Divide, Power,
}

#[derive(Debug, Clone, Copy)]
pub enum BiOpr {
    Add, Subtract, Multiply, Divide, Power,
}

#[derive(Debug, Clone, Copy)]
pub enum UnOpr {
    Plus, Minus,
}

impl OperatorRaw {
    fn binary(self) -> Option<BiOpr> {
        match self {
            Self::Plus => Some(BiOpr::Add),
            Self::Minus => Some(BiOpr::Subtract),
            Self::Multiply => Some(BiOpr::Multiply),
            Self::Divide => Some(BiOpr::Divide),
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
            Self::Multiply | Self::Divide => 2,
            Self::Add | Self::Subtract => 1,
        }
    }

    fn is_left_associative(self) -> bool {
        !matches!(self, Self::Power)
    }

    #[cfg(not(feature = "any_float"))]
    fn operate(self, l: f32, r: f32) -> f32 {
        match self {
            Self::Add => l + r,
            Self::Subtract => l - r,
            Self::Multiply => l * r,
            Self::Divide => l / r,
            Self::Power => l.powf(r),
        }
    }

    #[cfg(feature = "any_float")]
    fn operate<F: num_traits::Float>(self, l: F, r: F) -> F {
        match self {
            Self::Add => l + r,
            Self::Subtract => l - r,
            Self::Multiply => l * r,
            Self::Divide => l / r,
            Self::Power => l.powf(r),
        }
    }
}

impl UnOpr {
    fn percedence(self) -> usize {
        match self {
            Self::Plus | Self::Minus => 4,
        }
    }

    #[cfg(not(feature = "any_float"))]
    fn operate(self, v: f32) -> f32 {
        match self {
            Self::Plus => v,
            Self::Minus => -v,
        }
    }

    #[cfg(feature = "any_float")]
    fn operate<F: num_traits::Float>(self, v: F) -> F {
        match self {
            Self::Plus => v,
            Self::Minus => -v,
        }
    }
}

#[cfg(not(feature = "any_float"))]
impl Node<f32> {
    pub fn evaluate(&self) -> f32 {
        match &self.kind {
            NodeKind::BiOp(l, op, r) => op.operate(l.evaluate(), r.evaluate()),
            NodeKind::UnOp(op, v) => op.operate(v.evaluate()),
            NodeKind::Number(v) => *v,
        }
    }
}

#[cfg(feature = "any_float")]
impl<F: num_traits::Float> Node<F> {
    pub fn evaluate(&self) -> F {
        match &self.kind {
            NodeKind::BiOp(l, op, r) => op.operate(l.evaluate(), r.evaluate()),
            NodeKind::UnOp(op, v) => op.operate(v.evaluate()),
            NodeKind::Number(v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
enum Token<Number> {
    Operator(OperatorRaw),
    Number(Number),
    BStart(BKind),
    BEnd(BKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BKind {
    Round,
    Square,
    Curly,
}

struct Lexer<'src, Number> {
    source: std::str::Chars<'src>,
    start_index: usize,
    end_index: usize,
    skipped: Option<char>,

    _num: std::marker::PhantomData<Number>,
}

impl<Number: Numeral> Iterator for Lexer<'_, Number> {
    type Item = Result<Token<Number>, Span>;

    fn next(&mut self) -> Option<Result<Token<Number>, Span>> {
        let c = self.next_char()?;
        self.start_index = self.end_index - 1;

        match c {
            '+' => Some(Ok(Token::Operator(OperatorRaw::Plus))),
            '-' => Some(Ok(Token::Operator(OperatorRaw::Minus))),
            '*' | '×' => Some(Ok(Token::Operator(OperatorRaw::Multiply))),
            '/' | '÷' => Some(Ok(Token::Operator(OperatorRaw::Divide))),
            '^' => Some(Ok(Token::Operator(OperatorRaw::Power))),
            '(' => Some(Ok(Token::BStart(BKind::Round))),
            '[' => Some(Ok(Token::BStart(BKind::Square))),
            '{' => Some(Ok(Token::BStart(BKind::Curly))),
            ')' => Some(Ok(Token::BEnd(BKind::Round))),
            ']' => Some(Ok(Token::BEnd(BKind::Square))),
            '}' => Some(Ok(Token::BEnd(BKind::Curly))),
            '0'..='9' | '.' => {
                let mut acc = c.to_string();

                while let Some(c) = self.next_char() {
                    if matches!(c, '0'..='9' | '.') {
                        acc.push(c);
                    } else {
                        self.skipped = Some(c);
                        break;
                    }
                }

                self.end_index -= 1;

                Some(acc.parse().map_or_else(|_| Err(self.report_span()),|a| Ok(Token::Number(a))))
            },
            _ if c.is_whitespace() => self.next(),
            _ => if let Some(c) = Number::from_constant(c) {
                Some(Ok(Token::Number(c)))
            } else {
                Some(Err(self.report_span()))
            },
        }
    }
}

impl<Number> Lexer<'_, Number> {
    fn next_char(&mut self) -> Option<char> {
        self.end_index += 1;

        if self.skipped.is_some() {
            return core::mem::take(&mut self.skipped);
        }

        self.source.next()
    }

    fn report_span(&self) -> Span {
        self.start_index..self.end_index
    }
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
        Self { inner, peeked: None }
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

impl<Inner: Iterator<Item = Item>, Item> std::ops::Deref for Peeking<Inner, Item> {
    type Target = Inner;

    fn deref(&self) -> &Inner {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests() {
        let test = "1 + 2 * 3(4.5 + -3.5)";
        assert_eq!(super::to_nodes::<f32>(test).unwrap().evaluate(), 7.0);
    }
}
