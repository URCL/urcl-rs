



build:
	wasm-pack build --target web
	mv ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm
	wasm-opt -O3 -o ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm




webserver:
	wasm-pack build --target web
	mv ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm
	wasm-opt -O3 -o ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm
	python3 -m http.server