import init, * as bindings from "./pkg/bindings.js";

const EVALUATES = {
    f32: bindings.evaluate_f32,
    f64: bindings.evaluate_f64,
    cmplx_f32: bindings.evaluate_cmplx_f32,
    cmplx_f64: bindings.evaluate_cmplx_f64,
    cmplx: bindings.evaluate_cmplx_rational,
};

let katex = undefined;

document.addEventListener("DOMContentLoaded", async (_) => {
    const QUERY = new URLSearchParams(window.location.search);

    const INPUT = document.getElementById("input");
    const OUTPUT = document.getElementById("result");
    const SHARE_URL = document.getElementById("share_url");
    const SELECTOR = document.getElementById("type_selector");
    const AUTO_EVAL = document.getElementById("auto_eval");
    const EVAL_BTN = document.getElementById("eval_btn");
    const KATEX_P = document.getElementById("ii_out");
    const KATEX_TOGGLE = document.getElementById("katex_toggle");

    if (QUERY.get("katex") !== null) {
        let link = document.createElement("link");
        link.rel = "stylesheet";
        link.href = "https://cdn.jsdelivr.net/npm/katex@0.16.10/dist/katex.min.css";
        document.getElementsByTagName('head')[0].appendChild(link);

        await import("https://cdn.jsdelivr.net/npm/katex@0.16.10/dist/katex.min.mjs").then((resp) => {
            katex = resp.default;
            KATEX_P.style.display = "initial";
        });
    }

    document.body.style.visibility = 'visible';

    function update() {
        let evaluate = EVALUATES[window.location.hash.slice(1).split("-", 1)[0]];
        if (evaluate === undefined) {
            evaluate = bindings.evaluate_rational;
        }

        // sanitization should be done in rust side
        let e = evaluate(INPUT.value);
        OUTPUT.innerHTML = e.output;

        if (katex !== undefined) {
            if (e.latex != "") {
                KATEX_P.style.display = "initial";
                katex.render(e.latex, document.getElementById("ii"), {
                    throwOnError: false,
                    output: "html",
                });
            } else {
                KATEX_P.style.display = "none";
            }
        }

        let typ = window.location.hash.slice(1).split("-", 1)[0];
        SHARE_URL.innerText = `${window.location.protocol}//${window.location.host}${window.location.pathname}${window.location.search}#${typ}-${btoa(INPUT.value)}`;
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
        if (AUTO_EVAL.checked) {
            update();
        }
    };
    INPUT.onkeydown = (e) => {
        if (!AUTO_EVAL.checked && e.key === "Enter") {
            update();
        }
    };
    window.onhashchange = update;

    SELECTOR.value = window.location.hash.slice(1).split("-", 1)[0];
    SELECTOR.onchange = (_) => {
        window.location.hash = SELECTOR.value;
    };

    EVAL_BTN.onclick = update;

    AUTO_EVAL.onchange = (_) => {
        EVAL_BTN.style.display = AUTO_EVAL.checked ? "none" : "";
    };
    AUTO_EVAL.onchange(); // refresh the button's state

    KATEX_TOGGLE.onchange = (e) => {
        if (e.target.checked) {
            QUERY.set("katex", "");
        } else {
            QUERY.delete("katex");
        }

        window.location.search = QUERY.toString().replace(/=(?=&|$)/gm, '');
    }
    KATEX_TOGGLE.checked = katex !== undefined;
});
