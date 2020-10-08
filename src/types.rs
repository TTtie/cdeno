/// A boxed Deno interface.
pub type CDenoInterface<'l> = Box<&'l mut dyn deno_core::plugin_api::Interface>;

/// The function syntax of `cdeno_plugin_init`.
pub type CDenoPluginInit = extern "C" fn(iface: CDenoInterface);

/// An op callback. Returns a pointer to the op result.
pub type CDenoOpDispatcher = extern "C" fn(iface: CDenoInterface, zero_copy: Box<&mut [deno_core::plugin_api::ZeroCopyBuf]>, zero_copy_len: usize) -> *mut deno_core::plugin_api::Op;

/// An asynchronous op worker.
pub type CDenoAsyncOpDispatcher = extern "C" fn(data: *mut std::ffi::c_void, done_channel: Box<futures::channel::oneshot::Sender<Box<[u8]>>>);