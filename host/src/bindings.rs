pub mod guest {
  #[allow(unused_imports)]
  use wit_bindgen_wasmer::{anyhow, wasmer};
  
  /// Auxiliary data associated with the wasm exports.
  #[derive(Default)]
  pub struct GuestData {
  }
  impl wasmer::WasmerEnv for GuestData {
    fn init_with_instance(&mut self, instance: &wasmer::Instance) -> Result<(), wasmer::HostEnvInitError>{
      let _ = instance;
      Ok(())}
  }
  impl Clone for GuestData {
    fn clone(&self) -> Self {
      Self::default()
    }}
    pub struct Guest {
      state: std::sync::Arc<std::sync::Mutex<GuestData>>,
      func_loaded: wasmer::NativeFunc<(), ()>,
    }
    impl Guest {
      #[allow(unused_variables)]
      
      /// Adds any intrinsics, if necessary for this exported wasm
      /// functionality to the `ImportObject` provided.
      ///
      /// This function returns the `GuestData` which needs to be
      /// passed through to `Guest::new`.
      fn add_to_imports(
      store: &wasmer::Store,
      imports: &mut wasmer::ImportObject,
      ) -> std::sync::Arc<std::sync::Mutex<GuestData>> {
        let state = std::sync::Arc::new(std::sync::Mutex::new(Default::default()));
        state
      }
      
      /// Instantiates the provided `module` using the specified
      /// parameters, wrapping up the result in a structure that
      /// translates between wasm and the host.
      ///
      /// The `imports` provided will have intrinsics added to it
      /// automatically, so it's not necessary to call
      /// `add_to_imports` beforehand. This function will
      /// instantiate the `module` otherwise using `imports`, and
      /// both an instance of this structure and the underlying
      /// `wasmer::Instance` will be returned.
      pub fn instantiate(
      store: &wasmer::Store,
      module: &wasmer::Module,
      imports: &mut wasmer::ImportObject,
      ) -> anyhow::Result<(Self, wasmer::Instance)> {
        let state = Self::add_to_imports(store, imports);
        let instance = wasmer::Instance::new(module, &*imports)?;
        Ok((Self::new(&instance, state)?, instance))
      }
      
      /// Low-level creation wrapper for wrapping up the exports
      /// of the `instance` provided in this structure of wasm
      /// exports.
      ///
      /// This function will extract exports from the `instance`
      /// and wrap them all up in the returned structure which can
      /// be used to interact with the wasm module.
      pub fn new(
      instance: &wasmer::Instance,
      state: std::sync::Arc<std::sync::Mutex<GuestData>>,
      ) -> Result<Self, wasmer::ExportError> {
        let func_loaded= instance.exports.get_native_function::<(), ()>("loaded")?;
        Ok(Guest{
          func_loaded,
          state,
          
        })
      }
      /// A function that will be called after the WebAssembly module is loaded.
      pub fn loaded(&self,)-> Result<(), wasmer::RuntimeError> {
        self.func_loaded.call()?;
        Ok(())
      }
    }
  }
  pub mod host {
  #[allow(unused_imports)]
  use wit_bindgen_wasmer::{anyhow, wasmer};
  pub trait Host: Sized + wasmer::WasmerEnv + 'static{
    /// Print a message to the console.
    fn print(&mut self,msg: & str,);
    
  }
  
  pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
  where T: Host
  {
    #[derive(Clone)]struct EnvWrapper<T: Host> {
      data: T,
      memory: wasmer::LazyInit<wasmer::Memory>,
    }
    unsafe impl<T: Host> Send for EnvWrapper<T> {}
    unsafe impl<T: Host> Sync for EnvWrapper<T> {}
    impl<T: Host> wasmer::WasmerEnv for EnvWrapper<T> {
      fn init_with_instance(&mut self, instance: &wasmer::Instance) -> Result<(), wasmer::HostEnvInitError>{
        self.data.init_with_instance(instance)?;self.memory.initialize(instance.exports.get_with_generics_weak("memory")?);
        Ok(())}
    }
    let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper {
      data,
      memory: wasmer::LazyInit::new(),
    }));
    let mut exports = wasmer::Exports::new();
    exports.insert("print", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<(), wasmer::RuntimeError> {
      let env = &mut *env.lock().unwrap();
      let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
      let host = &mut env.data;
      let ptr0 = arg0;
      let len0 = arg1;
      let param0 = _bc.slice_str(ptr0, len0)?;
      host.print(param0, );
      Ok(())
    }));
    imports.register("host", exports);
  }
}
