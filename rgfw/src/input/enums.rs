//! All the big enums for keys, mouse, joypad, etc

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum MouseBtn {
    Left = 1,
    Middle = 2,
    Right = 3,
    ScrollUp = 4,
    ScrollDown = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum KeyCode {
    Null = rgfw_sys::RGFW_KEY_NULL as isize,
    Escape = rgfw_sys::RGFW_Escape as isize,
    F1 = rgfw_sys::RGFW_F1 as isize,
    F2 = rgfw_sys::RGFW_F2 as isize,
    F3 = rgfw_sys::RGFW_F3 as isize,
    F4 = rgfw_sys::RGFW_F4 as isize,
    F5 = rgfw_sys::RGFW_F5 as isize,
    F6 = rgfw_sys::RGFW_F6 as isize,
    F7 = rgfw_sys::RGFW_F7 as isize,
    F8 = rgfw_sys::RGFW_F8 as isize,
    F9 = rgfw_sys::RGFW_F9 as isize,
    F10 = rgfw_sys::RGFW_F10 as isize,
    F11 = rgfw_sys::RGFW_F11 as isize,
    F12 = rgfw_sys::RGFW_F12 as isize,
    Backtick = rgfw_sys::RGFW_Backtick as isize,
    Num0 = rgfw_sys::RGFW_0 as isize,
    Num1 = rgfw_sys::RGFW_1 as isize,
    Num2 = rgfw_sys::RGFW_2 as isize,
    Num3 = rgfw_sys::RGFW_3 as isize,
    Num4 = rgfw_sys::RGFW_4 as isize,
    Num5 = rgfw_sys::RGFW_5 as isize,
    Num6 = rgfw_sys::RGFW_6 as isize,
    Num7 = rgfw_sys::RGFW_7 as isize,
    Num8 = rgfw_sys::RGFW_8 as isize,
    Num9 = rgfw_sys::RGFW_9 as isize,
    Minus = rgfw_sys::RGFW_Minus as isize,
    Equals = rgfw_sys::RGFW_Equals as isize,
    Backspace = rgfw_sys::RGFW_BackSpace as isize,
    Tab = rgfw_sys::RGFW_Tab as isize,
    CapsLock = rgfw_sys::RGFW_CapsLock as isize,
    ShiftLeft = rgfw_sys::RGFW_ShiftL as isize,
    ControlLeft = rgfw_sys::RGFW_ControlL as isize,
    AltLeft = rgfw_sys::RGFW_AltL as isize,
    SuperLeft = rgfw_sys::RGFW_SuperL as isize,
    ShiftRight = rgfw_sys::RGFW_ShiftR as isize,
    ControlRight = rgfw_sys::RGFW_ControlR as isize,
    AltRight = rgfw_sys::RGFW_AltR as isize,
    SuperRight = rgfw_sys::RGFW_SuperR as isize,
    Space = rgfw_sys::RGFW_Space as isize,
    A = rgfw_sys::RGFW_a as isize,
    B = rgfw_sys::RGFW_b as isize,
    C = rgfw_sys::RGFW_c as isize,
    D = rgfw_sys::RGFW_d as isize,
    E = rgfw_sys::RGFW_e as isize,
    F = rgfw_sys::RGFW_f as isize,
    G = rgfw_sys::RGFW_g as isize,
    H = rgfw_sys::RGFW_h as isize,
    I = rgfw_sys::RGFW_i as isize,
    J = rgfw_sys::RGFW_j as isize,
    K = rgfw_sys::RGFW_k as isize,
    L = rgfw_sys::RGFW_l as isize,
    M = rgfw_sys::RGFW_m as isize,
    N = rgfw_sys::RGFW_n as isize,
    O = rgfw_sys::RGFW_o as isize,
    P = rgfw_sys::RGFW_p as isize,
    Q = rgfw_sys::RGFW_q as isize,
    R = rgfw_sys::RGFW_r as isize,
    S = rgfw_sys::RGFW_s as isize,
    T = rgfw_sys::RGFW_t as isize,
    U = rgfw_sys::RGFW_u as isize,
    V = rgfw_sys::RGFW_v as isize,
    W = rgfw_sys::RGFW_w as isize,
    X = rgfw_sys::RGFW_x as isize,
    Y = rgfw_sys::RGFW_y as isize,
    Z = rgfw_sys::RGFW_z as isize,
    Period = rgfw_sys::RGFW_Period as isize,
    Comma = rgfw_sys::RGFW_Comma as isize,
    Slash = rgfw_sys::RGFW_Slash as isize,
    Bracket = rgfw_sys::RGFW_Bracket as isize,
    CloseBracket = rgfw_sys::RGFW_CloseBracket as isize,
    Semicolon = rgfw_sys::RGFW_Semicolon as isize,
    Return = rgfw_sys::RGFW_Return as isize,
    Quote = rgfw_sys::RGFW_Quote as isize,
    BackSlash = rgfw_sys::RGFW_BackSlash as isize,
    Up = rgfw_sys::RGFW_Up as isize,
    Down = rgfw_sys::RGFW_Down as isize,
    Left = rgfw_sys::RGFW_Left as isize,
    Right = rgfw_sys::RGFW_Right as isize,
    Delete = rgfw_sys::RGFW_Delete as isize,
    Insert = rgfw_sys::RGFW_Insert as isize,
    End = rgfw_sys::RGFW_End as isize,
    Home = rgfw_sys::RGFW_Home as isize,
    PageUp = rgfw_sys::RGFW_PageUp as isize,
    PageDown = rgfw_sys::RGFW_PageDown as isize,
    Numlock = rgfw_sys::RGFW_Numlock as isize,
    KpSlash = rgfw_sys::RGFW_KP_Slash as isize,
    Multiply = rgfw_sys::RGFW_Multiply as isize,
    KpMinus = rgfw_sys::RGFW_KP_Minus as isize,
    Kp1 = rgfw_sys::RGFW_KP_1 as isize,
    Kp2 = rgfw_sys::RGFW_KP_2 as isize,
    Kp3 = rgfw_sys::RGFW_KP_3 as isize,
    Kp4 = rgfw_sys::RGFW_KP_4 as isize,
    Kp5 = rgfw_sys::RGFW_KP_5 as isize,
    Kp6 = rgfw_sys::RGFW_KP_6 as isize,
    Kp7 = rgfw_sys::RGFW_KP_7 as isize,
    Kp8 = rgfw_sys::RGFW_KP_8 as isize,
    Kp9 = rgfw_sys::RGFW_KP_9 as isize,
    Kp0 = rgfw_sys::RGFW_KP_0 as isize,
    KpPeriod = rgfw_sys::RGFW_KP_Period as isize,
    KpReturn = rgfw_sys::RGFW_KP_Return as isize,
}

/* We don't particularly care about error handling for (ideally) impossible keycode values
coming from RGFW */

impl From<u8> for KeyCode {
    fn from(value: u8) -> Self {
        FromPrimitive::from_u8(value).unwrap_or_else(|| panic!("Invalid event->keycode!"))
    }
}

impl From<u8> for MouseBtn {
    fn from(value: u8) -> Self {
        FromPrimitive::from_u8(value).unwrap_or_else(|| panic!("Invalid event->keycode!"))
    }
}
