use core::fmt;
use crate::*;

pub struct LatexDisplay<'a, T> {
    pub node: &'a Node<T>,
    pub src: &'a str,
}

impl<T> fmt::Display for LatexDisplay<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node.kind {
            NodeKind::Number(_) => write!(f, "{}", &self.src[self.node.span.clone()]),
            NodeKind::BiOp(l, BiOpr::Divide, r) => write!(f, r"\left(\frac{{{}}}{{{}}}\right)", LatexDisplay {
                node: l,
                src: self.src,
            }, LatexDisplay {
                node: r,
                src: self.src,
            }),
            NodeKind::BiOp(l, o, r) => write!(f, r"\left({{{}}}{o}{{{}}}\right)", LatexDisplay {
                node: l,
                src: self.src,
            }, LatexDisplay {
                node: r,
                src: self.src,
            }),
            NodeKind::UnOp(op, o) => write!(f, r"\left({op} {}\right)", LatexDisplay {
                node: o,
                src: self.src,
            }),
            NodeKind::Function(n, a) => {
                write!(f, r"\text{{{n}}}\left(")?;

                for (i, r) in a.iter().enumerate() {
                    fmt::Display::fmt(&LatexDisplay {
                        node: r,
                        src: self.src,
                    }, f)?;
                    if i != a.len() - 1 {
                        write!(f, ",")?;
                    }
                }

                write!(f, r"\right)")
            },
        }
    }
}

impl fmt::Display for BiOpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => r"\times",
            Self::Divide => r"\div",
            Self::Modulo => r"\mod",
            Self::Power => r"^",
        })
    }
}

impl fmt::Display for UnOpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Plus => "+",
            Self::Minus => "-",
        })
    }
}
