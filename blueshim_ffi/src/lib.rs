#![allow(non_camel_case_types)]
extern crate blueshim;

use std::rc::Rc;

use blueshim::Blueshim;

#[repr(C)]
pub struct blueshim_t {
    inner: Rc<Blueshim>,
}

#[no_mangle]
pub extern "C" fn blueshim_init() -> *mut blueshim_t {
    &mut blueshim_t {
        inner: Blueshim::init().into(),
    }
}
