use rgfw_sys::{RGFW_isMouseHeld, RGFW_isMousePressed, RGFW_isPressed, RGFW_window_mouseHold};

use crate::{utils::u8_to_bool, Area, Window};

mod enums;
pub use enums::*;

/// Equivalent to calling:
/// ```c
/// RGFW_isPressed(NULL, key);
/// ```
/// and doesn't require a window pointer
pub fn is_pressed(key: KeyCode) -> bool {
    u8_to_bool(unsafe { RGFW_isPressed(std::ptr::null_mut(), key as u8) })
}

// RGFWDEF b8 	RGFW_Error (void)
// RGFWDEF b8 	RGFW_isPressed (RGFW_window *win, u8 key)
// RGFWDEF b8 	RGFW_wasPressed (RGFW_window *win, u8 key)
// RGFWDEF b8 	RGFW_isHeld (RGFW_window *win, u8 key)
// RGFWDEF b8 	RGFW_isReleased (RGFW_window *win, u8 key)
// RGFWDEF b8 	RGFW_isClicked (RGFW_window *win, u8 key)
// RGFWDEF b8 	RGFW_isMousePressed (RGFW_window *win, u8 button)
// RGFWDEF b8 	RGFW_isMouseHeld (RGFW_window *win, u8 button)
// RGFWDEF b8 	RGFW_isMouseReleased (RGFW_window *win, u8 button)
// RGFWDEF b8 	RGFW_wasMousePressed (RGFW_window *win, u8 button)
impl Window {
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        u8_to_bool(unsafe { RGFW_isPressed(self.ptr, key as u8) })
    }
    pub fn is_mouse_pressed(&self, mouse: MouseBtn) -> bool {
        u8_to_bool(unsafe { RGFW_isMousePressed(self.ptr, mouse as u8) })
    }
    pub fn is_mouse_held(&self, mouse: MouseBtn) -> bool {
        u8_to_bool(unsafe { RGFW_isMouseHeld(self.ptr, mouse as u8) })
    }

    pub fn mouse_hold(&mut self, area: Area) {
        unsafe {
            RGFW_window_mouseHold(self.ptr, area.into());
        }
    }
}
