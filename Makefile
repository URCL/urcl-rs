run: build
	python3 -m http.server

build:
	cargo clean -p urcl-rs --release
	wasm-pack build --target web
	mv ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm
	wasm-opt -O3 -o ./pkg/urcl_rs_bg.wasm ./pkg/urcl_rs_tmp.wasm
	rm ./pkg/.gitignore
	rm ./pkg/urcl_rs_tmp.wasm

FNAME = target/release/urcl-rs
ifeq ($(OS), Windows_NT)
	FNAME = target/release/urcl-rs.exe
endif

RMNAME = urcl-rs
ifeq ($(OS), Windows_NT)
	RMNAME = urcl-rs.exe
endif

cli:
	cargo build --release
	rm $(RMNAME) -f
	mv $(FNAME) . -f

discord:
	cargo build --release --features "bot"
	rm $(RMNAME) -f
	mv $(FNAME) . -f