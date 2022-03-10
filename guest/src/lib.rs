#![allow(dead_code)]

include!("bindings.rs");

struct Guest;

impl guest::Guest for Guest {
    fn loaded() {
        dependency::greet("World");
    }
}
