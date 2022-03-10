include!("bindings.rs");

struct Dependency;

impl dependency::Dependency for Dependency {
    fn greet(name: String) {
        let message = format!("Hello, {name}!");
        host::print(&message);
    }
}
