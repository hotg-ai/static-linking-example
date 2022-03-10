# Static Linking with WAPM

## Background

Rune works by letting users create a machine learning pipeline declaratively and
have everything compiled into a single WebAssembly module that can be copied
around or used offline.

An integral part of declaring these ML pipelines is the ability to use custom
operations ("processing block") to pre-process data before it gets passed to a model.

Currently, we build a Rune by generating a Rust crate that imports each
processing block as a normal Rust dependendency then compiling everything to
`wasm32-unknown-unknown`.

We would like to move to something like this:

1. Someone writes their own custom operation (e.g. to rescale an image) and
   uploads the compiled `image-rescale.wasm` file to WAPM (e.g. as
   `@hotg-ai/image-rescale v1.2`)
2. Another user writes a Runefile which depends on `@hotg-ai/image-rescale`
3. The user uses `rune build ./Runefile.yml` to build their Rune
4. The `rune` tool downloads the `image-rescale.wasm` from WAPM and generates a
   Rust crate which uses functionality from it
5. The `rune` tool compiles the generated Rust crate to WebAssembly (e.g. as
   `my-rune.wasm`) and runs `wasmer-link my-rune.wasm image-rescale.wasm` to
   bundle everything into a single "statically linked" WebAssembly module

One of our core goals is to make it easy to deploy a ML pipeline without caring
about which platform it will eventually run on, so having a single file that can
be copied around is important for us.

Some other things to know about Rune:

- There is no guarantee that users will have internet access
- Our SAAS offering includes a security system where your app can download a
  Rune at runtime which has (among other things) been encrypted for just the
  current user
- We fall back to a WASM3 interpreter for devices that Wasmer either doesn't
  support or where we can't load WebAssembly dynamically (i.e. iOS)

## Getting Started

This repository contains 3 Rust crates,

- `dependency` is a Rust crate that will be compiled to a `dependency.wasm` file
  and exports the functions defined in `dependency.wit`
- `guest` is a Rust crate that will be compiled to WebAssembly. It imports
  functions from `dependency.wasm` and exports the functions declared in
  `guest.wit`
- `host` is a Wasmer program that runs on the host. It manually links the
  modules together at runtime by loading `dependency.wasm` and using its exports
  to satisfy the `dependency` imports for `guest.wasm`.

(assume `dependency` is published to WAPM and downloaded via `wapm install`)

An important part of what makes this process nice to work with is
[`wit-bindgen`][wit-binden]. Similar to a `*.proto` file in Protocol Buffers,
`wit-bindgen` lets us declare the functionality each component will provide and
generate glue code to use it.

You can install Wasmer's fork of `wit-bindgen` using the following command:

```console
cargo install --force --git https://github.com/wasmerio/wit-bindgen wit-bindgen-cli --branch wasmer
```

Normally we would use the procedural macros provided by the `wit-bindgen`
project to generate glue code for importing and exporting functions, but we'll
write the code to disk to let you see what is generated.

The `make build` command to write the glue code to disk and compile everything
to WebAssembly.

```console
$ wit-bindgen rust-wasm --out-dir dependency/src --import host.wit --export dependency.wit
Generating "dependency/src/bindings.rs"
$ cargo build --manifest-path dependency/Cargo.toml --target wasm32-unknown-unknown
   Compiling dependency v0.1.0 (~/Documents/hotg-ai/static-linking-example/dependency)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
$ cp target/wasm32-unknown-unknown/debug/dependency.wasm .
$ wit-bindgen rust-wasm --out-dir guest/src \
	--import dependency.wit \
	--import host.wit \
	--export guest.wit
Generating "guest/src/bindings.rs"
$ cargo build --manifest-path guest/Cargo.toml --target wasm32-unknown-unknown
   Compiling guest v0.1.0 (~/Documents/hotg-ai/static-linking-example/guest)
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
$ cp target/wasm32-unknown-unknown/debug/guest.wasm .
$ wit-bindgen wasmer --out-dir host/src \
	--import guest.wit \
	--export host.wit
Generating "host/src/bindings.rs"
```

You can run the `host` executable to see an example where I've worked around the
lack of static linking by manually wiring up the `ImportObjects` for
`guest.wasm` and `dependency.wasm`.

```console
$ cargo run guest.wasm dependency.wasm
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/host guest.wasm dependency.wasm`
Loading the dependency
Loading the guest
Calling loaded()
[*] Hello, Hello!
```

Outstanding questions:

- Is Wasmer able to "statically linking" multiple WebAssembly modules into a
  single `*.wasm` file, using the exports from one module to satisfy the
  imports from the other?
- Are there easier ways to achieve the same result?

[wit-bindgen]: https://github.com/bytecodealliance/wit-bindgen/
