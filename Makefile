



build:
	wasm-pack build --target web


webserver:
	wasm-pack build --target web
	python3 -m http.server