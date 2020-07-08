use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::os::raw::c_uint;
use x11_dl::keysym::*;
use x11_dl::xlib::*;

bitflags! {
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct LinuxKeyModifier: c_uint {
        #[allow(missing_docs)]
        const ALT = Mod1Mask;
        #[allow(missing_docs)]
        const CONTROL = ControlMask;
        #[allow(missing_docs)]
        const SHIFT = ShiftMask;
        #[allow(missing_docs)]
        const SUPER = Mod4Mask;
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct LinuxKeySpecial: c_uint {
        #[allow(missing_docs)]
        const BACKSPACE = XK_BackSpace;
        #[allow(missing_docs)]
        const TAB = XK_Tab;
        #[allow(missing_docs)]
        const CLEAR = XK_Clear;
        #[allow(missing_docs)]
        const ENTER = XK_Return;
        #[allow(missing_docs)]
        const PAUSE = XK_Pause;
        #[allow(missing_docs)]
        const CAPSLOCK = XK_Caps_Lock;
        #[allow(missing_docs)]
        const ESCAPE = XK_Escape;
        #[allow(missing_docs)]
        const SPACEBAR = XK_space;
        #[allow(missing_docs)]
        const PAGE_UP = XK_Page_Up;
        #[allow(missing_docs)]
        const PAGE_DOWN = XK_Page_Down;
        #[allow(missing_docs)]
        const END = XK_End;
        #[allow(missing_docs)]
        const HOME = XK_Home;
        #[allow(missing_docs)]
        const LEFT_ARROW = XK_Left;
        #[allow(missing_docs)]
        const UP_ARROW = XK_Up;
        #[allow(missing_docs)]
        const RIGHT_ARROW = XK_Right;
        #[allow(missing_docs)]
        const DOWN_ARROW = XK_Down;
        #[allow(missing_docs)]
        const SELECT = XK_Select;
        #[allow(missing_docs)]
        const PRINT = XK_Print;
        #[allow(missing_docs)]
        const PRINT_SCREEN = 1;
        #[allow(missing_docs)]
        const INSERT = XK_Insert;
        #[allow(missing_docs)]
        const DELETE = XK_Delete;
        #[allow(missing_docs)]
        const F1 = XK_F1;
        #[allow(missing_docs)]
        const F2 = XK_F2;
        #[allow(missing_docs)]
        const F3 = XK_F3;
        #[allow(missing_docs)]
        const F4 = XK_F4;
        #[allow(missing_docs)]
        const F5 = XK_F5;
        #[allow(missing_docs)]
        const F6 = XK_F6;
        #[allow(missing_docs)]
        const F7 = XK_F7;
        #[allow(missing_docs)]
        const F8 = XK_F8;
        #[allow(missing_docs)]
        const F9 = XK_F9;
        #[allow(missing_docs)]
        const F10 = XK_F10;
        #[allow(missing_docs)]
        const F11 = XK_F11;
        #[allow(missing_docs)]
        const F12 = XK_F12;
        #[allow(missing_docs)]
        const NUMLOCK = XK_Num_Lock;
        #[allow(missing_docs)]
        const SCROLLLOCK = XK_Scroll_Lock;
    }
}
