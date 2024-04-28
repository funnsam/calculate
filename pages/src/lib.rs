use smolcalc::{*, traits::*};
use num_bigint::BigInt;
use num_complex::Complex;
use wasm_bindgen::prelude::*;

fn sanitize(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('/', "&#x2F;")
}

fn report(src: &str, span: Span) -> String {
    format!(
        "\
<span class=\"error\">Error:</span>
  {}
  <span class=\"report_arrow\">{:<2$}{3:^<4$}</span>",
        sanitize(src),
        "",
        span.start,
        "",
        span.end - span.start
    )
}

fn evaluate<T: ComputableNumeral>(s: &str) -> Result<T, String> {
    Ok(to_nodes::<T>(s)
        .map_err(|span| report(s, span))?
        .evaluate()
        .map_err(|span| report(s, span))?
    )
}

#[wasm_bindgen]
pub fn evaluate_f32(s: &str) -> String {
    evaluate::<f32>(s)
        .map(|a| trunc(&format!("{a:.5}")).to_string())
        .map(pretty_result)
        .unwrap_or_else(|s| s)
}

#[wasm_bindgen]
pub fn evaluate_f64(s: &str) -> String {
    evaluate::<f64>(s)
        .map(|a| trunc(&format!("{a:.13}")).to_string())
        .map(pretty_result)
        .unwrap_or_else(|s| s)
}

#[wasm_bindgen]
pub fn evaluate_rational(s: &str) -> String {
    evaluate::<rational::Rational<BigInt>>(s)
        .map(|a| format!("{a:#}"))
        .map(pretty_result)
        .unwrap_or_else(|s| s)
}

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
}

#[wasm_bindgen]
pub fn evaluate_cmplx_rational(s: &str) -> String {
    evaluate::<rational::complex::ComplexRational<BigInt>>(s)
        .map(|a| a.to_string())
        .map(pretty_result)
        .unwrap_or_else(|s| s)
}

fn pretty_result(s: String) -> String {
    format!("= {}", sanitize(&s))
}

fn pretty_cmplx<T: num_traits::Float + num_traits::Signed, F: Fn(T) -> String>(c: Complex<T>, f: F) -> String {
    if !c.im.is_negative() {
        format!("{}+{}i", f(c.re), f(c.im))
    } else {
        format!("{}-{}i", f(c.re), f(c.im.abs()))
    }
}

fn trunc(s: &str) -> &str {
    match s.as_bytes().last() {
        Some(b'0') => trunc(&s[..s.len()-1]),
        Some(b'.') => &s[..s.len()-1],
        _ => s,
    }
}
