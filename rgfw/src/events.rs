#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use rgfw_sys::{
    RGFW_keyPressed, RGFW_keyReleased, RGFW_mouseButtonPressed, RGFW_mouseButtonReleased,
    RGFW_quit, RGFW_window_checkEvent,
};

use crate::input::{KeyCode, MouseBtn};
use crate::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseBtnPressed(MouseBtn),
    MouseBtnReleased(MouseBtn),
    MousePosChanged { x: i32, y: i32 },
    // TODO: Joystick events
    MouseEnter,
    MouseLeave,
    Quit,
}

impl Window {
    /// Returns a raw [`rgfw_sys::RGFW_Event`] struct if an event is there.
    ///
    /// You may want to get a [`rgfw::Event`] instead.
    pub fn raw_check_event(&self) -> Option<rgfw_sys::RGFW_Event> {
        let ev = unsafe { RGFW_window_checkEvent(self.ptr) };
        if ev.is_null() {
            return None;
        }
        let ev = unsafe { *ev };
        Some(ev)
    }
    pub fn events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        while let Some(event) = self.next_event() {
            events.push(event);
        }
        events
    }
    pub fn next_event(&self) -> Option<Event> {
        self.raw_check_event().map(|ev| ev.into())
    }
}

impl From<rgfw_sys::RGFW_Event> for Event {
    fn from(ev: rgfw_sys::RGFW_Event) -> Self {
        match ev.type_ {
            RGFW_keyPressed => Event::KeyPressed(ev.keyCode.into()),
            RGFW_keyReleased => Event::KeyReleased(ev.keyCode.into()),
            RGFW_mouseButtonPressed => Event::MouseBtnPressed(ev.button.into()),
            RGFW_mouseButtonReleased => Event::MouseBtnReleased(ev.button.into()),
            RGFW_mousePosChanged => Event::MousePosChanged {
                x: ev.point.x,
                y: ev.point.y,
            },
            RGFW_quit => Event::Quit,
            _ => unreachable!("An unknown event type ({}) was in event->type!", ev.type_),
        }
    }
}

/*
pub const RGFW_keyPressed: _bindgen_ty_1 = 1;
#[doc = "< a key has been released"]
pub const RGFW_keyReleased: _bindgen_ty_1 = 2;
#[doc = "< a mouse button has been pressed (left,middle,right)"]
pub const RGFW_mouseButtonPressed: _bindgen_ty_1 = 3;
#[doc = "< a mouse button has been released (left,middle,right)"]
pub const RGFW_mouseButtonReleased: _bindgen_ty_1 = 4;
#[doc = "< the position of the mouse has been changed"]
pub const RGFW_mousePosChanged: _bindgen_ty_1 = 5;
#[doc = "< a joystick button was pressed"]
pub const RGFW_jsButtonPressed: _bindgen_ty_1 = 6;
#[doc = "< a joystick button was released"]
pub const RGFW_jsButtonReleased: _bindgen_ty_1 = 7;
#[doc = "< an axis of a joystick was moved"]
pub const RGFW_jsAxisMove: _bindgen_ty_1 = 8;
#[doc = "< the window was moved (by the user)"]
pub const RGFW_windowMoved: _bindgen_ty_1 = 9;
#[doc = "< the window was resized (by the user), [on webASM this means the browser was resized]"]
pub const RGFW_windowResized: _bindgen_ty_1 = 10;
#[doc = "< window is in focus now"]
pub const RGFW_focusIn: _bindgen_ty_1 = 11;
#[doc = "< window is out of focus now"]
pub const RGFW_focusOut: _bindgen_ty_1 = 12;
pub const RGFW_mouseEnter: _bindgen_ty_1 = 13;
pub const RGFW_mouseLeave: _bindgen_ty_1 = 14;
pub const RGFW_windowRefresh: _bindgen_ty_1 = 15;
#[doc = "< the user clicked the quit button"]
pub const RGFW_quit: _bindgen_ty_1 = 16;
#[doc = "< a file has been dropped into the window"]
pub const RGFW_dnd: _bindgen_ty_1 = 17;
#[doc = "< the start of a dnd event, when the place where the file drop is known"]
pub const RGFW_dnd_init: _bindgen_ty_1 = 18;
*/
