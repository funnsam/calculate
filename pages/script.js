import init, * as bindings from "./pkg/bindings.js";

const EVALUATES = {
	f32: bindings.evaluate_f32,
	f64: bindings.evaluate_f64,
	cmplx_f32: bindings.evaluate_cmplx_f32,
	cmplx_f64: bindings.evaluate_cmplx_f64,
	cmplx: bindings.evaluate_cmplx_rational,
};

document.addEventListener("DOMContentLoaded", (_) => {
	const INPUT = document.getElementById("input");
	const OUTPUT = document.getElementById("result");
	const SHARE_URL = document.getElementById("share_url");
	const SELECTOR = document.getElementById("type_selector");
	const AUTO_EVAL = document.getElementById("auto_eval");
	const EVAL_BTN = document.getElementById("eval_btn");

	document.body.style.visibility = 'visible';

	function update() {
		let evaluate = EVALUATES[window.location.hash.slice(1).split("-", 1)[0]];
		if (evaluate === undefined) {
			evaluate = bindings.evaluate_rational;
		}

		// sanitization should be done in rust side
		OUTPUT.innerHTML = evaluate(INPUT.value);

		let typ = window.location.hash.slice(1).split("-", 1)[0];
		SHARE_URL.innerText = `${window.location.protocol}//${window.location.host}${window.location.pathname}#${typ}-${btoa(INPUT.value)}`;
	}

	let expr = window.location.hash.slice(1).split("-", 2)[1];
	if (expr !== undefined) {
		try {
			INPUT.value = atob(expr);
		} catch {}
	}

	init().then(() => {
		// bindings.enable_panic_hook();
		update();
	});
	INPUT.oninput = (_) => {
		if (AUTO_EVAL.value) {
			update();
		}
	};
	window.onhashchange = update;

	SELECTOR.value = window.location.hash.slice(1).split("-", 1)[0];
	SELECTOR.onchange = (_) => {
		window.location.hash = SELECTOR.value;
	};

	EVAL_BTN.onclick = update;
});
