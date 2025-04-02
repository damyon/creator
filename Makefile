build:
	wasm-pack build --target web --out-dir app

view:
	google-chrome --allow-file-access-from-files app/index.html &

watch: 
	cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --target web --out-dir app"
