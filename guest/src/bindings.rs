mod dependency {
  pub fn greet(name: & str,){
    unsafe {
      let vec0 = name;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      #[link(wasm_import_module = "dependency")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "greet")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "dependency_greet")]
        fn wit_import(_: i32, _: i32, );
      }
      wit_import(ptr0, len0);
    }
  }
}
mod host {
  /// Print a message to the console.
  pub fn print(msg: & str,){
    unsafe {
      let vec0 = msg;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      #[link(wasm_import_module = "host")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "print")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "host_print")]
        fn wit_import(_: i32, _: i32, );
      }
      wit_import(ptr0, len0);
    }
  }
}
mod guest {
  #[export_name = "loaded"]
  unsafe extern "C" fn __wit_bindgen_loaded(){
    <super::Guest as Guest>::loaded();
  }
  pub trait Guest {
    /// A function that will be called after the WebAssembly module is loaded.
    fn loaded();
  }
}
