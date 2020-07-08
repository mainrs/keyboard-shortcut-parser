use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::os::raw::c_uint;
use x11_dl::keysym::*;
use x11_dl::xlib::*;

bitflags! {
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct X11KeyModifier: c_uint {
        const ALT = Mod1Mask;
        const CONTROL = ControlMask;
        const SHIFT = ShiftMask;
        const SUPER = Mod4Mask;
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct X11KeySpecial: c_uint {
        const BACKSPACE = XK_BackSpace;
        const TAB = XK_Tab;
        const CLEAR = XK_Clear;
        const ENTER = XK_Return;
        const PAUSE = XK_Pause;
        const CAPSLOCK = XK_Caps_Lock;
        const ESCAPE = XK_Escape;
        const SPACEBAR = XK_space;
        const PAGE_UP = XK_Page_Up;
        const PAGE_DOWN = XK_Page_Down;
        const END = XK_End;
        const HOME = XK_Home;
        const LEFT_ARROW = XK_Left;
        const UP_ARROW = XK_Up;
        const RIGHT_ARROW = XK_Right;
        const DOWN_ARROW = XK_Down;
        const SELECT = XK_Select;
        const PRINT = XK_Print;
        const PRINT_SCREEN = 1;
        const INSERT = XK_Insert;
        const DELETE = XK_Delete;
        const F1 = XK_F1;
        const F2 = XK_F2;
        const F3 = XK_F3;
        const F4 = XK_F4;
        const F5 = XK_F5;
        const F6 = XK_F6;
        const F7 = XK_F7;
        const F8 = XK_F8;
        const F9 = XK_F9;
        const F10 = XK_F10;
        const F11 = XK_F11;
        const F12 = XK_F12;
        const NUMLOCK = XK_Num_Lock;
        const SCROLLLOCK = XK_Scroll_Lock;
    }
}
