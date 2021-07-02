SHELL		= /bin/bash

.PHONY: all test tools build clean

all: build build_web

test: build build_web
	RUST_BACKTRACE=1 cargo test -p aiid -- --nocapture
	cd aiid-js && (which node_modules/.bin/standard || npm ci) && npm test

tools:
	rustup override set nightly-2019-01-24
	rustup target add wasm32-unknown-unknown
	if ! (which wasm-bindgen) || [ "$(shell wasm-bindgen --version)" != "wasm-bindgen 0.2.40" ]; then cargo install --force wasm-bindgen-cli --version "=0.2.40"; fi

build: tools
	cargo build -p aiid --release
	cargo build -p aiid_js --target wasm32-unknown-unknown --release
	wasm-bindgen target/wasm32-unknown-unknown/release/aiid_js.wasm --out-dir aiid-js/lib --out-name bindgen --nodejs --no-typescript

build_web: build
	wasm-bindgen target/wasm32-unknown-unknown/release/aiid_js.wasm --out-dir aiid-js/lib/browser --out-name bindgen --browser --no-typescript
	wasm2es6js --base64 -o aiid-js/lib/browser/bindgen_bg.js aiid-js/lib/browser/bindgen_bg.wasm
	rm aiid-js/lib/browser/bindgen_bg.wasm

clean:
	rm -rf target aiid-js/rust/target aiid-js/lib/bindgen_bg.wasm
