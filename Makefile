build:
	 cargo +nightly build -Zbuild-std=std,panic_abort --target=wasm32-wasi --release
build-js:
	 cargo +nightly build -Zbuild-std=std,panic_abort --target=wasm32-wasi --release --features js