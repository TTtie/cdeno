use crate::types::*;
use crate::OP_TO_FN_PTR_MAP;
use crate::OP_NAME_TO_ID_MAP;
use deno_core::plugin_api::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar};

#[no_mangle]
pub extern "C" fn cdeno_register_op(
    iface: CDenoInterface,
    name: *const c_char,
    dispatcher: CDenoOpDispatcher,
) -> OpId {
    let cstr = unsafe { CStr::from_ptr(name) };
    let map_str = cstr.to_str().unwrap();

    fn cdeno_trampoline(iface: &mut dyn Interface, buf: &mut [ZeroCopyBuf]) -> Op {
        let op_id_buf: &[u8] = &buf[0];

        // It is safe to assume usize == 8 bytes as Deno bins are only x64
        let mut bytes: [u8; std::mem::size_of::<usize>()] = Default::default();
        bytes.copy_from_slice(&op_id_buf[0..std::mem::size_of::<usize>()]);
        let op_id = usize::from_ne_bytes(bytes);

        OP_TO_FN_PTR_MAP.with(|map| {
            let map = map.borrow_mut();
            if map.contains_key(&op_id) {
                let buf_len = map.len() - 1;
                let op_fn = map.get(&op_id).unwrap();
                println!(
                    "Brace for impact, calling {:?} at {:?}",
                    op_id,
                    (*op_fn) as *const CDenoOpDispatcher
                );

                let boxed_iface = Box::new(iface);
                let boxed_buf = Box::new(&mut buf[1..]);
                println!(
                    "Arg sizes: {:?} {:?}",
                    std::mem::size_of::<CDenoInterface>(),
                    std::mem::size_of::<Box<&mut [ZeroCopyBuf]>>()
                );
                let op = (*op_fn)(boxed_iface, boxed_buf, buf_len);

                return *op;
            };

            Op::Sync(vec![].into_boxed_slice())
        })
    }
    println!(
        "Registering op {:?} at {:?}",
        map_str, dispatcher as *const CDenoOpDispatcher
    );
    let op_id = iface.register_op(map_str, cdeno_trampoline);
    OP_TO_FN_PTR_MAP.with(|map| map.borrow_mut().insert(op_id, dispatcher));
    OP_NAME_TO_ID_MAP.with(|map| map.borrow_mut().insert(map_str.to_string(), op_id));
    op_id
}

#[no_mangle]
pub extern "C" fn cdeno_create_op_sync(char_ptr: *mut c_uchar, len: usize) -> *mut Op {
    println!("Creating a op from {:?} bytes", len);
    let slice = unsafe { std::slice::from_raw_parts(char_ptr, len) };

    let vec = Vec::from(slice);

    println!("Created a sync op from {:?}", vec);

    Box::into_raw(Box::new(Op::Sync(vec.into_boxed_slice())))
}
