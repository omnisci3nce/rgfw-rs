use rgfw_sys::RGFW_window_checkEvent;

use crate::Window;

impl Window {
    pub fn check_event(&self) -> Option<rgfw_sys::RGFW_Event> {
        let ev = unsafe { RGFW_window_checkEvent(self.0) };
        if ev.is_null() {
            return None;
        }
        let ev = unsafe { *ev };
        Some(ev)
    }
}
