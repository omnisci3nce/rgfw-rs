use core::str;
use rgfw_sys::*;
use std::ffi::CString;

pub mod events;
pub mod input;
pub mod utils;

use utils::u8_to_bool;

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

    pub fn should_close(&self) -> bool {
        u8_to_bool(unsafe { RGFW_window_shouldClose(self.ptr) })
    }
    pub fn swap_buffers(&self) {
        unsafe {
            RGFW_window_swapBuffers(self.ptr);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Area {
    w: u32,
    h: u32,
}
impl Area {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            w: width,
            h: height,
        }
    }
}
impl From<Area> for rgfw_sys::RGFW_area {
    fn from(area: Area) -> Self {
        Self {
            w: area.w,
            h: area.h,
        }
    }
}
