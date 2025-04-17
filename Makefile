build:
	wasm-pack build --target web --out-dir app

view:
	google-chrome --allow-file-access-from-files app/index.html &

watch: 
	bacon

dev:
	cargo install bacon

lint:
	cargo clippy

doc:
	cargo doc
