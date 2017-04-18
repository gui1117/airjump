/// https://github.com/cactorium/spring-crabs/tree/emscripten
extern crate libc;

use self::libc::uintptr_t;
use std::os::raw::{c_void, c_char, c_int};

#[allow(non_camel_case_types)]
pub type backtrace_syminfo_callback =
    extern fn(data: *mut c_void,
              pc: uintptr_t,
              symname: *const c_char,
              symval: uintptr_t,
              symsize: uintptr_t);
#[allow(non_camel_case_types)]
pub type backtrace_full_callback =
    extern fn(data: *mut c_void,
              pc: uintptr_t,
              filename: *const c_char,
              lineno: c_int,
              function: *const c_char) -> c_int;
#[allow(non_camel_case_types)]
pub type backtrace_error_callback =
    extern fn(data: *mut c_void,
              msg: *const c_char,
              errnum: c_int);
#[allow(non_camel_case_types)]
pub enum backtrace_state {}

#[no_mangle]
pub extern "C" fn __rbt_backtrace_create_state(_filename: *const c_char,
                                  _threaded: c_int,
                                  _error: backtrace_error_callback,
                                  _data: *mut c_void) -> *mut backtrace_state { 0 as *mut _ }

#[no_mangle]
pub extern "C" fn __rbt_backtrace_syminfo(_state: *mut backtrace_state,
                             _addr: uintptr_t,
                             _cb: backtrace_syminfo_callback,
                             _error: backtrace_error_callback,
                             _data: *mut c_void) -> c_int { 0 }

#[no_mangle]
pub extern "C" fn __rbt_backtrace_pcinfo(_state: *mut backtrace_state,
                            _addr: uintptr_t,
                            _cb: backtrace_full_callback,
                            _error: backtrace_error_callback,
                            _data: *mut c_void) -> c_int { 0 }
