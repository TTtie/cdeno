pub type CDenoInterface<'l> = Box<&'l mut dyn deno_core::plugin_api::Interface>;
pub type CDenoPluginInit = extern "C" fn(CDenoInterface);
pub type CDenoOpDispatcher = extern "C" fn(CDenoInterface, Box<&mut [deno_core::plugin_api::ZeroCopyBuf]>, usize) -> Box<deno_core::plugin_api::Op>;
