import init, { evaluate_f32, evaluate_f64, evaluate_rational, JsSpan } from "./pkg/bindings.js";

let evaluate = evaluate_rational;

const urlp = new URLSearchParams(window.location.search);
const type = urlp.get("type");

if (type == "f32") {
	evaluate = (v) => { +evaluate_f32(v).toFixed(5) };
} else if (type == "f64") {
	evaluate = (v) => { +evaluate_f64(v).toFixed(5) };
}

document.addEventListener("DOMContentLoaded", (_) => {
	function update() {
		try {
			OUTPUT.innerText = `= ${+evaluate(INPUT.value).toFixed(5)}`;
		} catch (span) {
			if (span instanceof JsSpan) {
				OUTPUT.innerText = `Error:\n  ${INPUT.value}\n  ${" ".repeat(span.start)}${"^".repeat(span.end - span.start)}`;
			} else {
				throw span;
			}
		}
	}

	const INPUT = document.getElementById("input");
	const OUTPUT = document.getElementById("result");

	init().then(() => { update(); });
	INPUT.oninput = (_) => { update(); };
});
