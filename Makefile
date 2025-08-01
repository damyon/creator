OS_TYPE := $(shell uname -s)

# Set a variable based on the detected OS
ifeq ($(OS_TYPE),Darwin)
  BROWSER = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
else
  BROWSER := $(shell which google-chrome)
endif

build:
	wasm-pack build --target web --out-dir app

view:
	@echo "$(BROWSER)"
	$(BROWSER) --allow-file-access-from-files app/index.html &

watch: 
	bacon

dev:
	cargo install bacon

lint:
	cargo clippy

doc:
	cargo doc --document-private-items
