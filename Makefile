build: dependency.wasm guest.wasm host/src/bindings.rs

run: build
	cargo run -- guest.wasm dependency.wasm

dependency.wasm: dependency.wit host.wit
	wit-bindgen rust-wasm --out-dir dependency/src --import host.wit --export dependency.wit
	cargo build --manifest-path dependency/Cargo.toml --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/dependency.wasm .

guest.wasm: guest.wit host.wit
	wit-bindgen rust-wasm --out-dir guest/src \
		--import dependency.wit \
		--import host.wit \
		--export guest.wit
	cargo build --manifest-path guest/Cargo.toml --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/guest.wasm .

host/src/bindings.rs: guest.wit host.wit
	wit-bindgen wasmer --out-dir host/src \
		--import guest.wit \
		--export host.wit

clean:
	$(RM) -r target *.wasm

.PHONY: install-wit-bindgen clean

