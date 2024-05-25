build:
	wasm-pack build --target web
	mv ./pkg/bindings_bg.wasm ./pkg/tmp.wasm
	wasm-opt --strip-debug -Oz -o ./pkg/bindings_bg.wasm ./pkg/tmp.wasm
	rm ./pkg/.gitignore
	rm ./pkg/tmp.wasm
	# mv ./pkg/bindings.js ./pkg/bindings.mjs
