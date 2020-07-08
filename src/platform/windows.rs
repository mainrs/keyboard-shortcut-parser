use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winapi::shared::minwindef::LPARAM;
use winapi::{ctypes::c_int, um::winuser::*};

bitflags! {
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct WinKeyModifier: LPARAM {
        const ALT = MOD_ALT;
        const CONTROL = MOD_CONTROL;
        const SHIFT = MOD_SHIFT;
        const SUPER = MOD_WIN;
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct WinKeySpecial: c_int {
        const BACKSPACE = VK_BACK;
        const TAB = VK_TAB;
        const CLEAR = VK_CLEAR;
        const ENTER = VK_RETURN;
        const PAUSE = VK_PAUSE;
        const CAPSLOCK = VK_CAPITAL;
        const ESCAPE = VK_ESCAPE;
        const SPACEBAR = VK_SPACE;
        const PAGE_UP = VK_PRIOR;
        const PAGE_DOWN = VK_NEXT;
        const END = VK_END;
        const HOME = VK_HOME;
        const LEFT_ARROW = VK_LEFT;
        const UP_ARROW = VK_UP;
        const RIGHT_ARROW = VK_RIGHT;
        const DOWN_ARROW = VK_DOWN;
        const SELECT = VK_SELECT;
        const PRINT = VK_PRINT;
        const PRINT_SCREEN = VK_SNAPSHOT;
        const INSERT = VK_INSERT;
        const DELETE = VK_DELETE;
        const F1 = VK_F1;
        const F2 = VK_F2;
        const F3 = VK_F3;
        const F4 = VK_F4;
        const F5 = VK_F5;
        const F6 = VK_F6;
        const F7 = VK_F7;
        const F8 = VK_F8;
        const F9 = VK_F9;
        const F10 = VK_F10;
        const F11 = VK_F11;
        const F12 = VK_F12;
        const NUMLOCK = VK_NUMLOCK;
        const SCROLLLOCK = VK_SCROLL;
    }
}
