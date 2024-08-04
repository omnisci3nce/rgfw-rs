use std::ffi::CString;

use rgfw_sys::*;

pub mod events;

/// Wrapper around the raw window pointer
pub struct Window(*mut RGFW_window);

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            RGFW_window_close(self.0);
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
        let window_name = CString::new(title.as_bytes())
            .unwrap()
            .as_bytes_with_nul()
            .as_ptr() as *const _;
        let rect = rgfw_sys::RGFW_rect {
            x: 500,
            y: 500,
            w: 500,
            h: 500,
        };
        let win = unsafe { RGFW_createWindow(window_name, rect, 0) };
        Window(win)
    }
    pub fn get_window_ptr(&self) -> *mut RGFW_window {
        self.0
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        u8_to_bool(unsafe { RGFW_isPressed(self.0, key as u8) })
    }

    pub fn should_close(&self) -> bool {
        u8_to_bool(unsafe { RGFW_window_shouldClose(self.0) })
    }
    pub fn swap_buffers(&self) {
        unsafe {
            RGFW_window_swapBuffers(self.0);
        }
    }
}

fn u8_to_bool(int: u8) -> bool {
    int != 0
}
