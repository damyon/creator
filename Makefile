build:
	wasm-pack build --target docs --out-dir docs

dev: build
	php -S 0.0.0.0:8080 -t docs/
