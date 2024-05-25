use num_bigint::BigInt;
use num_complex::Complex;
use smolcalc::{traits::*, *};
use wasm_bindgen::prelude::*;

fn sanitize(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('/', "&#x2F;")
}

fn report(src: &str, err: Error) -> String {
    format!(
        "\
<span class=\"error\">Error:</span> {}
  {}
  <span class=\"report_arrow\">{:<3$}{2:^<4$}</span>",
        err.message,
        sanitize(src),
        "",
        err.location.start,
        err.location.end - err.location.start
    )
}

fn evaluate<T: ComputableNumeral>(s: &str) -> Result<EvalMid<T>, String> {
    to_nodes::<T>(s)
        .map_err(|e| report(s, e))
        .and_then(|n| Ok(EvalMid {
            output: n.evaluate().map_err(|e| report(s, e))?,
            latex: latex::LatexDisplay {
                node: &n,
                src: s,
            }.to_string()
        }))
}

// #[wasm_bindgen]
// pub fn enable_panic_hook() { std::panic::set_hook(Box::new(console_error_panic_hook::hook)); }

pub struct EvalMid<T> {
    pub output: T,
    pub latex: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct Eval {
    pub output: String,
    pub latex: String,
}

#[wasm_bindgen]
pub fn evaluate_f32(s: &str) -> Eval {
    pretty_result(evaluate::<f32>(s), |a| trunc(&format!("{a:.5}")).to_string())
}

#[wasm_bindgen]
pub fn evaluate_f64(s: &str) -> Eval {
    pretty_result(evaluate::<f64>(s), |a| trunc(&format!("{a:.13}")).to_string())
}

#[wasm_bindgen]
pub fn evaluate_rational(s: &str) -> Eval {
    pretty_result(evaluate::<rational::Rational<BigInt>>(s), |a| format!("{:#}", a.limit_denom(1_000_000_000_000_000_u64.into())))
}

/*
#[wasm_bindgen]
pub fn evaluate_cmplx_f32(s: &str) -> String {
    evaluate::<Complex<f32>>(s)
        .map(|a| pretty_cmplx(a, |a| format!("{a:.13}")))
        .map(pretty_result)
        .unwrap_or_else(|s| s)
}

#[wasm_bindgen]
pub fn evaluate_cmplx_f64(s: &str) -> String {
    evaluate::<Complex<f64>>(s)
        .map(|a| pretty_cmplx(a, |a| format!("{a:.13}")))
        .map(pretty_result)
        .unwrap_or_else(|s| s)
} */

#[wasm_bindgen]
pub fn evaluate_cmplx_rational(s: &str) -> Eval {
    pretty_result(evaluate::<rational::complex::ComplexRational<BigInt>>(s), |a| format!("{:#}", a.limit_denom(1_000_000_000_000_000_u64.into())))
}

fn pretty_result<T, F: Fn(T) -> String>(e: Result<EvalMid<T>, String>, f: F) -> Eval {
    match e {
        Ok(e) => Eval {
            output: format!("= {}", sanitize(&f(e.output))),
            latex: e.latex,
        },
        Err(e) => Eval {
            output: e,
            latex: String::new(),
        }
    }
}

fn pretty_cmplx<T: num_traits::Float + num_traits::Signed, F: Fn(T) -> String>(
    c: Complex<T>,
    f: F,
) -> String {
    if !c.im.is_negative() {
        format!("{}+{}i", f(c.re), f(c.im))
    } else {
        format!("{}-{}i", f(c.re), f(c.im.abs()))
    }
}

fn trunc(s: &str) -> &str {
    match s.as_bytes().last() {
        Some(b'0') => trunc(&s[..s.len() - 1]),
        Some(b'.') => &s[..s.len() - 1],
        _ => s,
    }
}
