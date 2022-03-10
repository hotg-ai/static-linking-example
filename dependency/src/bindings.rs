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
mod dependency {
  #[export_name = "greet"]
  unsafe extern "C" fn __wit_bindgen_greet(arg0: i32, arg1: i32, ){
    let len0 = arg1 as usize;
    <super::Dependency as Dependency>::greet(String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap());
  }
  pub trait Dependency {
    fn greet(name: String,);
  }
}
