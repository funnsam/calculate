import init, { evaluate, JsSpan } from "./pkg/bindings.js";

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
