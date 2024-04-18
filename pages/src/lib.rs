use wasm_bindgen::prelude::*;
use calculate::*;

#[wasm_bindgen]
pub struct JsSpan {
    pub start: usize,
    pub end: usize,
}

#[wasm_bindgen]
pub fn evaluate(s: &str) -> Result<f32, JsSpan> {
    Ok(to_nodes(s).map_err(|s| JsSpan {
        start: s.start,
        end: s.end,
    })?.evaluate())
}
