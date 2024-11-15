build:
	wasm-pack build --target web --out-dir web

dev: build
	php -S 0.0.0.0:8080 -t web/
