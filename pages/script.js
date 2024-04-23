import init, { evaluate_f32, evaluate_f64, evaluate_rational, JsSpan } from "./pkg/bindings.js";

document.addEventListener("DOMContentLoaded", (_) => {
	function update() {
		let evaluate = evaluate_rational;

		const type = window.location.hash.slice(1);
		if (type == "f32") {
			evaluate = (v) => { return +evaluate_f32(v).toFixed(5); };
		} else if (type == "f64") {
			evaluate = (v) => { return +evaluate_f64(v).toFixed(5); };
		} else {
			evaluate = evaluate_rational;
		}

		try {
			OUTPUT.innerText = `= ${evaluate(INPUT.value)}`;
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

	init().then(update);
	INPUT.oninput = (_) => { update(); };
	window.onhashchange = update;
});
