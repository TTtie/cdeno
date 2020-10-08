pub type CDenoInterface<'l> = Box<&'l mut dyn deno_core::plugin_api::Interface>;
pub type CDenoPluginInit = extern "C" fn(CDenoInterface);
pub type CDenoOpDispatcher = extern "C" fn(CDenoInterface, Box<&mut [deno_core::plugin_api::ZeroCopyBuf]>, usize) -> *mut deno_core::plugin_api::Op;
pub type CDenoAsyncOpDispatcher = extern "C" fn(*mut std::ffi::c_void, Box<futures::channel::oneshot::Sender<Box<[u8]>>>);