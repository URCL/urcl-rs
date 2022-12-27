run: build
	python3 -m http.server

build:
	cargo clean -p urcl-rs --release
	wasm-pack build --target web
	mv ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm
	wasm-opt -O3 -o ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm
	rm ./pkg/.gitignore
	rm ./pkg/urcl_rs_tmp.wasm

build_cli:
	cargo clean -p urcl-rs --release
	cargo build --release