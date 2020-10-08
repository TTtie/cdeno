use crate::types::*;
use crate::OP_NAME_TO_ID_MAP;
use crate::OP_TO_FN_PTR_MAP;
use deno_core::plugin_api::*;
use futures::channel::oneshot::{channel, Sender};
use futures::FutureExt;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar};
use std::thread::spawn;

/// Registers an op callback.
#[no_mangle]
pub extern "C" fn cdeno_register_op(
    iface: *mut CDenoInterface,
    name: *const c_char,
    dispatcher: CDenoOpDispatcher,
) -> OpId {
    let cstr = unsafe { CStr::from_ptr(name) };
    let map_str = cstr.to_str().unwrap();

    let interface = unsafe { &mut *iface };

    fn cdeno_trampoline(iface: &mut dyn Interface, buf: &mut [ZeroCopyBuf]) -> Op {
        let op_id_buf: &[u8] = &buf[0];

        // It is safe to assume usize == 8 bytes as Deno bins are only x64
        let mut bytes: [u8; std::mem::size_of::<usize>()] = Default::default();
        bytes.copy_from_slice(&op_id_buf[0..std::mem::size_of::<usize>()]);
        let op_id = usize::from_ne_bytes(bytes);

        OP_TO_FN_PTR_MAP.with(|map| {
            let map = map.borrow_mut();
            if map.contains_key(&op_id) {
                let op_fn = map.get(&op_id).unwrap();
                println!(
                    "Brace for impact, calling {:?} at {:?}",
                    op_id,
                    (*op_fn) as *const CDenoOpDispatcher
                );

                let boxed_iface = Box::new(iface);
                let boxed_buf = Box::new(&mut buf[1..]);
                let buf_len = boxed_buf.len();
                let op = unsafe { Box::from_raw((*op_fn)(boxed_iface, boxed_buf, buf_len)) };

                return *op;
            };

            Op::Sync(vec![].into_boxed_slice())
        })
    }
    
    let op_id = interface.register_op(map_str, cdeno_trampoline);
    OP_TO_FN_PTR_MAP.with(|map| map.borrow_mut().insert(op_id, dispatcher));
    OP_NAME_TO_ID_MAP.with(|map| map.borrow_mut().insert(map_str.to_string(), op_id));
    op_id
}

/// Creates a synchronous op.
#[no_mangle]
pub extern "C" fn cdeno_create_op_sync(char_ptr: *mut c_uchar, len: usize) -> *mut Op {
    println!("Creating a op from {:?} bytes", len);
    let slice = unsafe { std::slice::from_raw_parts(char_ptr, len) };

    let vec = Vec::from(slice);

    println!("Created a sync op from {:?}", vec);

    Box::into_raw(Box::new(Op::Sync(vec.into_boxed_slice())))
}

#[repr(C)]
/// A data holder for async ops.
pub struct CDenoAsyncOpData {
    /// A pointer to a data structure.
    data: *mut c_void,
}

unsafe impl Send for CDenoAsyncOpData {}

/// Creates an asynchronous Deno op.
#[no_mangle]
pub extern "C" fn cdeno_create_op_async(
    worker: CDenoAsyncOpDispatcher,
    data: CDenoAsyncOpData,
) -> *mut Op {
    let fut = async move {
        let (tx, rx) = channel();
        spawn(move || {
            worker(data.data, Box::new(tx));
        });
        rx.await.unwrap()
    };

    Box::into_raw(Box::new(Op::Async(fut.boxed())))
}

/// Responds to/resolves an asynchronous op
#[no_mangle]
pub extern "C" fn cdeno_async_op_respond(
    tx: Box<Sender<Box<[u8]>>>,
    char_ptr: *mut c_uchar,
    len: usize,
) {
    println!("Creating an async op response from {:?} bytes", len);
    let slice = unsafe { std::slice::from_raw_parts(char_ptr, len) };

    let vec = Vec::from(slice);

    println!("Created a async op response from {:?}", vec);
    tx.send(vec.into_boxed_slice()).unwrap();
}

/// Represents the data of a zero-copy buffer
#[repr(C)]
pub struct ZeroCopyData {
    /// The length of a zero copy buffer
    len: usize,
    /// A pointer to the zero copy buffer data.
    data: *const c_uchar,
}

/// Gets data from a zero-copy buffer.
#[no_mangle]
pub extern "C" fn cdeno_get_zero_copy_buf(
    zero_copy: *const Box<&mut [ZeroCopyBuf]>,
    index: usize,
) -> ZeroCopyData {
    let no_copy = unsafe { &*zero_copy };
    println!("Getting zero copy buf at index {:?}", index);
    if no_copy.len() > index {
        let buf: &[u8] = &no_copy[index];
        ZeroCopyData {
            len: buf.len(),
            data: buf.as_ptr(),
        }
    } else {
        ZeroCopyData {
            len: 0,
            data: [].as_ptr(),
        }
    }
}
