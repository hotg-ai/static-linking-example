#![allow(dead_code)]
use clap::Parser;
use std::path::PathBuf;
use wasmer::{ImportObject, Instance, Module, Store, WasmerEnv};

use crate::guest::Guest;

include!("bindings.rs");

fn main() {
    let Args { dependency, guest } = Args::parse();

    let store = Store::default();

    println!("Loading the dependency");
    let wasm = std::fs::read(&dependency).unwrap();
    let module = Module::new(&store, &wasm).unwrap();
    let mut imports = ImportObject::default();
    host::add_to_imports(&store, &mut imports, Host);
    let dependency = Instance::new(&module, &imports).unwrap();

    println!("Loading the guest");
    let wasm = std::fs::read(&guest).unwrap();
    let module = Module::new(&store, &wasm).unwrap();
    let mut imports = ImportObject::default();
    imports.register("dependency", dependency.exports);
    host::add_to_imports(&store, &mut imports, Host);

    let (guest, _) = Guest::instantiate(&store, &module, &mut imports).unwrap();

    println!("Calling loaded()");
    guest.loaded().unwrap();
}

#[derive(Debug, Parser)]
struct Args {
    guest: PathBuf,
    dependency: PathBuf,
}

#[derive(Copy, Clone, WasmerEnv)]
struct Host;

impl host::Host for Host {
    fn print(&mut self, msg: &str) {
        println!("[*] {msg}");
    }
}
