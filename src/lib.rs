#![feature(map_into_keys_values)]

use deno_core::plugin_api::*;
use deno_core::serde_json as serde;
use dlopen::symbor::Library;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod ffi;
mod types;
use types::*;

pub use ffi::*;

thread_local! {
    pub static LIB_HOLDER: RefCell<HashMap<usize, Rc<Library>>> = RefCell::new(HashMap::new());
    pub static OP_TO_FN_PTR_MAP: RefCell<HashMap<OpId, CDenoOpDispatcher>> = RefCell::new(HashMap::new());
    pub static OP_NAME_TO_ID_MAP: RefCell<HashMap<String, OpId>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub fn deno_plugin_init(iface: &mut dyn Interface) {
    iface.register_op("cdeno::open_plugin", open_plugin);
}

/// Opens a native plugin and returns a pointer to it
fn open_plugin(iface: &mut dyn Interface, buf: &mut [ZeroCopyBuf]) -> Op {
    let lib_path = String::from_utf8(Vec::from(&*buf[0]));
    if let Err(e) = lib_path {
        eprintln!("Cannot convert the library path: {:?}", e);
        return Op::Sync(vec![].into_boxed_slice());
    }
    let lib_path = lib_path.unwrap();
    let lib = Library::open(lib_path);
    if let Err(e) = lib {
        eprintln!("Cannot open the library: {:?}", e);
        return Op::Sync(vec![].into_boxed_slice());
    }
    let lib = lib.unwrap();
    let cdeno_plugin_init = LIB_HOLDER.with(|v| {
        let l = Rc::new(lib);
        let mut map = v.borrow_mut();
        let map_len = map.len();

        let sym = unsafe { l.symbol::<CDenoPluginInit>("cdeno_plugin_init") };
        if let Err(e) = sym {
            eprintln!("Cannot find the init symbol: {:?}", e);
            return Err(Op::Sync(vec![].into_boxed_slice()));
        }
        let sym = sym.unwrap();
        let cdeno_plugin_init = *sym;

        map.insert(map_len, l);
        Ok(cdeno_plugin_init)
    });

    match cdeno_plugin_init {
        Ok(cdeno_plugin_init) => {
            let boxed_iface = Box::new(iface);

            cdeno_plugin_init(boxed_iface);

            let current_ops = OP_NAME_TO_ID_MAP.with(|map| {
                let mut serde_map = serde::Map::new();
                for (k, v) in map.borrow_mut().iter() {
                    serde_map.insert(k.clone(), serde::Value::String(v.to_string()));
                }

                serde::Value::Object(serde_map)
            });
            Op::Sync(serde::to_vec(&current_ops).unwrap().into_boxed_slice())
        }
        Err(e) => e,
    }
}
