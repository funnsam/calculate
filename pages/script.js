import init, * as bindings from "./pkg/bindings.js";

const EVALUATES = {
	f32: bindings.evaluate_f32,
	f64: bindings.evaluate_f64,
	cmplx_f32: bindings.evaluate_cmplx_f32,
	cmplx_f64: bindings.evaluate_cmplx_f64,
	cmplx: bindings.evaluate_cmplx_rational,
};

document.addEventListener("DOMContentLoaded", (_) => {
	function update() {
		let evaluate = EVALUATES[window.location.hash.slice(1)];
		if (evaluate === undefined) {
			evaluate = bindings.evaluate_rational;
		}

		// sanitization should be done in rust side
		OUTPUT.innerHTML = evaluate(INPUT.value);
	}

	const INPUT = document.getElementById("input");
	const OUTPUT = document.getElementById("result");

	init().then(update);
	INPUT.oninput = (_) => { update(); };
	window.onhashchange = update;

	const SELECTOR = document.getElementById("type_selector");
	SELECTOR.value = window.location.hash;
	SELECTOR.onchange = (_) => {
		window.location.hash = SELECTOR.value;
	};
});
