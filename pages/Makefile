build:
	wasm-pack build --target web
	mv ./pkg/bindings_bg.wasm ./pkg/tmp.wasm
	wasm-opt -O3 -o ./pkg/bindings_bg.wasm ./pkg/tmp.wasm
	rm ./pkg/.gitignore
	rm ./pkg/tmp.wasm
