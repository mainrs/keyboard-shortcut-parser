use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use winapi::shared::minwindef::LPARAM;
use winapi::{ctypes::c_int, um::winuser::*};

bitflags! {
    /// Bitflags holding all modifier keys.
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct WinKeyModifier: LPARAM {
        #[allow(missing_docs)]
        const ALT = MOD_ALT;
        #[allow(missing_docs)]
        const CONTROL = MOD_CONTROL;
        #[allow(missing_docs)]
        const SHIFT = MOD_SHIFT;
        #[allow(missing_docs)]
        const SUPER = MOD_WIN;
    }
}

bitflags! {
    /// Bitflags holding all special keys.
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct WinKeySpecial: c_int {
        #[allow(missing_docs)]
        const BACKSPACE = VK_BACK;
        #[allow(missing_docs)]
        const TAB = VK_TAB;
        #[allow(missing_docs)]
        const CLEAR = VK_CLEAR;
        #[allow(missing_docs)]
        const ENTER = VK_RETURN;
        #[allow(missing_docs)]
        const PAUSE = VK_PAUSE;
        #[allow(missing_docs)]
        const CAPSLOCK = VK_CAPITAL;
        #[allow(missing_docs)]
        const ESCAPE = VK_ESCAPE;
        #[allow(missing_docs)]
        const SPACEBAR = VK_SPACE;
        #[allow(missing_docs)]
        const PAGE_UP = VK_PRIOR;
        #[allow(missing_docs)]
        const PAGE_DOWN = VK_NEXT;
        #[allow(missing_docs)]
        const END = VK_END;
        #[allow(missing_docs)]
        const HOME = VK_HOME;
        #[allow(missing_docs)]
        const LEFT_ARROW = VK_LEFT;
        #[allow(missing_docs)]
        const UP_ARROW = VK_UP;
        #[allow(missing_docs)]
        const RIGHT_ARROW = VK_RIGHT;
        #[allow(missing_docs)]
        const DOWN_ARROW = VK_DOWN;
        #[allow(missing_docs)]
        const SELECT = VK_SELECT;
        #[allow(missing_docs)]
        const PRINT = VK_PRINT;
        #[allow(missing_docs)]
        const PRINT_SCREEN = VK_SNAPSHOT;
        #[allow(missing_docs)]
        const INSERT = VK_INSERT;
        #[allow(missing_docs)]
        const DELETE = VK_DELETE;
        #[allow(missing_docs)]
        const F1 = VK_F1;
        #[allow(missing_docs)]
        const F2 = VK_F2;
        #[allow(missing_docs)]
        const F3 = VK_F3;
        #[allow(missing_docs)]
        const F4 = VK_F4;
        #[allow(missing_docs)]
        const F5 = VK_F5;
        #[allow(missing_docs)]
        const F6 = VK_F6;
        #[allow(missing_docs)]
        const F7 = VK_F7;
        #[allow(missing_docs)]
        const F8 = VK_F8;
        #[allow(missing_docs)]
        const F9 = VK_F9;
        #[allow(missing_docs)]
        const F10 = VK_F10;
        #[allow(missing_docs)]
        const F11 = VK_F11;
        #[allow(missing_docs)]
        const F12 = VK_F12;
        #[allow(missing_docs)]
        const NUMLOCK = VK_NUMLOCK;
        #[allow(missing_docs)]
        const SCROLLLOCK = VK_SCROLL;
    }
}
