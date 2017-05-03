use std::cell::RefCell;
use std::ptr::null_mut;
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();
extern {
    fn emscripten_set_main_loop(func : em_callback_func, fps : c_int, simulate_infinite_loop : c_int);
    fn emscripten_cancel_main_loop();
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn set_main_loop_callback<F>(callback : F) where F : FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });

    unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F : FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

pub fn cancel_main_loop() {
    unsafe { emscripten_cancel_main_loop() }
}

extern {
    pub fn emscripten_request_fullscreen_strategy(target: *const c_char, deferUntilInEventHandler: c_int, fullscreenStrategy: *const EmscriptenFullscreenStrategy) -> c_int;
    pub fn emscripten_enter_soft_fullscreen(target: *const c_char, fullscreenStrategy: *const EmscriptenFullscreenStrategy);
}

pub const EMSCRIPTEN_FULLSCREEN_SCALE_DEFAULT: ::std::os::raw::c_int = 0;
pub const EMSCRIPTEN_FULLSCREEN_SCALE_STRETCH: ::std::os::raw::c_int = 1;
pub const EMSCRIPTEN_FULLSCREEN_SCALE_ASPECT: ::std::os::raw::c_int = 2;
pub const EMSCRIPTEN_FULLSCREEN_SCALE_CENTER: ::std::os::raw::c_int = 3;
pub const EMSCRIPTEN_FULLSCREEN_CANVAS_SCALE_NONE: ::std::os::raw::c_int = 0;
pub const EMSCRIPTEN_FULLSCREEN_CANVAS_SCALE_STDDEF: ::std::os::raw::c_int = 1;
pub const EMSCRIPTEN_FULLSCREEN_CANVAS_SCALE_HIDEF: ::std::os::raw::c_int = 2;
pub const EMSCRIPTEN_FULLSCREEN_FILTERING_DEFAULT: ::std::os::raw::c_int = 0;
pub const EMSCRIPTEN_FULLSCREEN_FILTERING_NEAREST: ::std::os::raw::c_int = 1;
pub const EMSCRIPTEN_FULLSCREEN_FILTERING_BILINEAR: ::std::os::raw::c_int = 2;

#[repr(C)]
pub struct EmscriptenFullscreenStrategy {
    pub scale_mode: c_int,
    pub canvas_resolution_scale_mode: c_int,
    pub filtering_mode: c_int,
    pub canvas_resized_callback: EmCallbackResizedCallback,
    pub canvas_resized_callback_userdata: *mut c_void,
}

pub type EmCallbackResizedCallback = Option<unsafe extern fn(eventType: c_int, reserved: *const c_void, userData: *mut c_void) -> c_int>;

pub fn request_soft_fullscreen_strategy() {
    unsafe {
        let strategy = EmscriptenFullscreenStrategy {
            scale_mode: EMSCRIPTEN_FULLSCREEN_SCALE_DEFAULT,
            canvas_resolution_scale_mode: EMSCRIPTEN_FULLSCREEN_CANVAS_SCALE_STDDEF,
            filtering_mode: EMSCRIPTEN_FULLSCREEN_FILTERING_DEFAULT,
            canvas_resized_callback: None,
            canvas_resized_callback_userdata: 0 as *mut c_void,
        };
        emscripten_enter_soft_fullscreen(ptr::null(), &strategy);
    }
}
