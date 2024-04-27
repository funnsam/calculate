use smolcalc::{*, traits::*};
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

fn evaluate<T: ComputableNumeral + std::string::ToString>(s: &str) -> Result<T, JsSpan> {
    Ok(to_nodes::<T>(s)
        .map_err(|s| JsSpan {
            start: s.start,
            end: s.end,
        })?
        .evaluate()
        .map_err(|s| JsSpan {
            start: s.start,
            end: s.end,
        })?)
}

#[wasm_bindgen]
pub fn evaluate_f32(s: &str) -> Result<String, JsSpan> {
    evaluate::<f32>(s)
        .map(|a| trunc(&format!("{a:.5}")).to_string())
}

#[wasm_bindgen]
pub fn evaluate_f64(s: &str) -> Result<String, JsSpan> {
    evaluate::<f64>(s)
        .map(|a| trunc(&format!("{a:.13}")).to_string())
}

#[wasm_bindgen]
pub fn evaluate_rational(s: &str) -> Result<String, JsSpan> {
    evaluate::<rational::Rational<BigInt>>(s)
        .map(|a| format!("{a:#}"))
}

#[wasm_bindgen]
pub fn evaluate_cmplx_f32(s: &str) -> Result<String, JsSpan> {
    evaluate::<Complex<f32>>(s)
        .map(|a| format!("{}+{}i", trunc(&format!("{:.5}", a.re)), trunc(&format!("{:.5}", a.im))))
}

#[wasm_bindgen]
pub fn evaluate_cmplx_f64(s: &str) -> Result<String, JsSpan> {
    evaluate::<Complex<f64>>(s)
        .map(|a| format!("{}+{}i", trunc(&format!("{:.13}", a.re)), trunc(&format!("{:.13}", a.im))))
}

#[wasm_bindgen]
pub fn evaluate_cmplx_rational(s: &str) -> Result<String, JsSpan> {
    evaluate::<rational::complex::ComplexRational<BigInt>>(s)
        .map(|a| a.to_string())
}

fn trunc(s: &str) -> &str {
    match s.as_bytes().last() {
        Some(b'0') => trunc(&s[..s.len()-1]),
        Some(b'.') => &s[..s.len()-1],
        _ => s,
    }
}
