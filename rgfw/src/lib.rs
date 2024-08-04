use core::str;
use std::ffi::CString;

use rgfw_sys::*;

pub mod events;

/// Wrapper around the raw window pointer
pub struct Window {
    ptr: *mut RGFW_window,
    title: CString,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            RGFW_window_close(self.ptr);
        }
    }
}

pub enum Key {
    Up = rgfw_sys::RGFW_Up as isize,
    Down = rgfw_sys::RGFW_Down as isize,
    Left = rgfw_sys::RGFW_Left as isize,
    Right = rgfw_sys::RGFW_Right as isize,
    Escape = rgfw_sys::RGFW_Escape as isize,
}

impl Window {
    pub fn create(title: &str) -> Self {
        let window_title = CString::new(title.as_bytes()).unwrap(); // We need to hang on to this for the same lifetime as the window pointer
        let window_name = window_title.as_bytes_with_nul().as_ptr() as *const _;
        let rect = rgfw_sys::RGFW_rect {
            x: 500,
            y: 500,
            w: 500,
            h: 500,
        };
        let win = unsafe { RGFW_createWindow(window_name, rect, 0) };
        Window {
            ptr: win,
            title: window_title,
        }
    }

    pub fn title(&self) -> &str {
        str::from_utf8(self.title.as_bytes()).expect("Window::title should be a valid string")
    }

    pub fn get_window_ptr(&self) -> *mut RGFW_window {
        self.ptr
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        u8_to_bool(unsafe { RGFW_isPressed(self.ptr, key as u8) })
    }

    pub fn should_close(&self) -> bool {
        u8_to_bool(unsafe { RGFW_window_shouldClose(self.ptr) })
    }
    pub fn swap_buffers(&self) {
        unsafe {
            RGFW_window_swapBuffers(self.ptr);
        }
    }
}

fn u8_to_bool(int: u8) -> bool {
    int != 0
}
