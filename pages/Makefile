# do `cargo install wasm-bindgen-cli` before `make`
build:
	- mkdir target
	cargo build --lib --target wasm32-unknown-unknown --profile web_release --target-dir target
	wasm-bindgen --web --out-dir pkg --no-typescript target/wasm32-unknown-unknown/web_release/bindings.wasm
	wasm-opt --strip-debug -Oz -o ./pkg/bindings_bg.wasm ./pkg/bindings_bg.wasm
