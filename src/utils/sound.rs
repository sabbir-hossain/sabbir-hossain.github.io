#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(module = "/js/audio.js"))]
extern "C" {
    fn input_value();
    // fn scene1_success();
    fn failed();
    fn select_option();
    fn clear_input();
    fn select_home();
    // fn start_running();
    // fn stop_running();
    // fn wheel_moving();
}

// Call this when needed (e.g. on keypress)
#[allow(unused_unsafe)]
pub fn add_input() {
    unsafe {
        input_value();
    }
}

#[allow(unused_unsafe)]
pub fn reset_input() {
    unsafe {
        clear_input();
    }
}

// #[allow(unused_unsafe)]
// pub fn collision_success() {
//     unsafe {
//         scene1_success();
//     }
// }

#[allow(unused_unsafe)]
pub fn status_failed() {
    unsafe {
        failed();
    }
}

#[allow(unused_unsafe)]
pub fn select_scene() {
    unsafe {
        select_option();
    }
}

#[allow(unused_unsafe)]
pub fn select_init_scene() {
    unsafe {
        select_home();
    }
}

#[allow(unused_unsafe)]
pub fn start_engine() {
    unsafe {
        // start_running();
    }
}

#[allow(unused_unsafe)]
pub fn scene3_start() {
    unsafe  {
        // wheel_moving();
    }
}


